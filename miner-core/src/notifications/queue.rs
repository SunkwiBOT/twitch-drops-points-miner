use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};
use std::time::Duration;

use reqwest::StatusCode;
use tokio::sync::mpsc::{self, error::TryRecvError};
use tokio::sync::oneshot;
use tokio::task::AbortHandle;
use tokio::time::Instant;

use super::{discord, Answer, Notice, Shared};
use crate::api::NotificationTarget;
use crate::error::{Error, Result};

pub(crate) const TARGETS: [NotificationTarget; 3] = [
    NotificationTarget::Discord,
    NotificationTarget::Telegram,
    NotificationTarget::Webhook,
];

const QUEUE_MAX: usize = 500;
const ATTEMPTS: u32 = 6;
const BACKOFF: Duration = Duration::from_secs(2);
const RETRY_AFTER_MAX: Duration = Duration::from_secs(300);
// ! A burst (a "Claim all", several accounts at once) is gathered into one
// Discord message: it ends after a second without a new notification.
const GATHER_QUIET: Duration = Duration::from_secs(1);
const GATHER_MAX: Duration = Duration::from_secs(5);
// ! Telegram allows about one message a second in a chat.
const TELEGRAM_PACE: Duration = Duration::from_secs(1);
const LOSSES_SAID_AFTER: Duration = Duration::from_secs(60);

pub(crate) fn name(target: NotificationTarget) -> &'static str {
    match target {
        NotificationTarget::Discord => "Discord",
        NotificationTarget::Telegram => "Telegram",
        NotificationTarget::Webhook => "webhook",
    }
}

type Reply = oneshot::Sender<Result<()>>;

enum Item {
    Notice(Notice),
    Test(Notice, Reply),
}

#[derive(Default)]
struct Counters {
    waiting: AtomicUsize,
    lost: AtomicUsize,
}

struct Queue {
    sender: mpsc::Sender<Item>,
    counters: Arc<Counters>,
    task: AbortHandle,
}

pub(crate) struct Queues {
    discord: Queue,
    telegram: Queue,
    webhook: Queue,
}

impl Queues {
    pub fn start(shared: &Arc<Shared>) -> Self {
        let open = |target: NotificationTarget| {
            let (sender, receiver) = mpsc::channel(QUEUE_MAX);
            let counters = Arc::new(Counters::default());
            let task = tokio::spawn(deliver(
                Arc::downgrade(shared),
                target,
                receiver,
                counters.clone(),
            ))
            .abort_handle();

            Queue {
                sender,
                counters,
                task,
            }
        };

        Self {
            discord: open(NotificationTarget::Discord),
            telegram: open(NotificationTarget::Telegram),
            webhook: open(NotificationTarget::Webhook),
        }
    }

    fn queue(&self, target: NotificationTarget) -> &Queue {
        match target {
            NotificationTarget::Discord => &self.discord,
            NotificationTarget::Telegram => &self.telegram,
            NotificationTarget::Webhook => &self.webhook,
        }
    }

    pub fn push(&self, target: NotificationTarget, notice: Notice) {
        let queue = self.queue(target);

        queue.counters.waiting.fetch_add(1, Ordering::SeqCst);

        if queue.sender.try_send(Item::Notice(notice)).is_err() {
            lower(&queue.counters.waiting, 1);
            queue.counters.lost.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn test(
        &self,
        target: NotificationTarget,
        notice: Notice,
    ) -> oneshot::Receiver<Result<()>> {
        let (reply, answer) = oneshot::channel();
        let _refused = self
            .queue(target)
            .sender
            .try_send(Item::Test(notice, reply));

        answer
    }

    // ! Nothing is waited for: the tasks end and what they still held is counted.
    pub fn shutdown(&self) -> usize {
        [&self.discord, &self.telegram, &self.webhook]
            .into_iter()
            .map(|queue| {
                queue.task.abort();

                queue.counters.waiting.swap(0, Ordering::SeqCst)
                    + queue.counters.lost.swap(0, Ordering::SeqCst)
            })
            .sum()
    }
}

// ! An aborted task may still finish its batch after `shutdown` read the count.
fn lower(counter: &AtomicUsize, by: usize) {
    let _ = counter.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |count| {
        Some(count.saturating_sub(by))
    });
}

enum Next {
    Wait(Duration),
    Drop(Error),
}

fn backoff(attempt: u32) -> Duration {
    BACKOFF * 2u32.pow(attempt - 1)
}

fn retry(target: NotificationTarget, attempt: u32, outcome: &Result<Answer>) -> Next {
    match outcome {
        Ok(answer) if answer.status == StatusCode::TOO_MANY_REQUESTS => {
            let wait = answer.retry_after.unwrap_or_else(|| backoff(attempt));

            if wait > RETRY_AFTER_MAX {
                return Next::Drop(Error::Notification(format!(
                    "{} asked to wait {} s",
                    name(target),
                    wait.as_secs()
                )));
            }

            Next::Wait(wait)
        }
        Ok(answer) if answer.status.is_server_error() => Next::Wait(backoff(attempt)),
        Ok(answer) => Next::Drop(refused(target, answer.status)),
        Err(Error::Network(_) | Error::Transport(_)) => Next::Wait(backoff(attempt)),
        Err(error) => Next::Drop(error.clone()),
    }
}

fn refused(target: NotificationTarget, status: StatusCode) -> Error {
    Error::Notification(format!("{} answered {}", name(target), status.as_u16()))
}

fn last_error(target: NotificationTarget, outcome: Result<Answer>) -> Error {
    match outcome {
        Ok(answer) => refused(target, answer.status),
        Err(error) => error,
    }
}

fn pause_after(target: NotificationTarget, answer: &Answer) -> Duration {
    let pause = answer.pause.unwrap_or_default().min(RETRY_AFTER_MAX);

    if target == NotificationTarget::Telegram {
        return pause + TELEGRAM_PACE;
    }

    pause
}

async fn send_with_retries(
    shared: &Shared,
    target: NotificationTarget,
    notices: &[Notice],
    mut reply: Option<&mut Reply>,
) -> Result<Duration> {
    let mut attempt = 1;

    loop {
        let unwanted = match reply.as_deref() {
            Some(reply) => reply.is_closed(),
            None => !shared.enabled(target),
        };

        if unwanted {
            return Ok(Duration::ZERO);
        }

        let outcome = shared.send(target, notices).await;

        if let Ok(answer) = &outcome {
            if answer.status.is_success() {
                return Ok(pause_after(target, answer));
            }
        }

        let wait = match retry(target, attempt, &outcome) {
            Next::Drop(error) => return Err(error),
            Next::Wait(_) if attempt == ATTEMPTS => return Err(last_error(target, outcome)),
            Next::Wait(wait) => wait,
        };

        match reply.as_deref_mut() {
            Some(reply) => tokio::select! {
                () = tokio::time::sleep(wait) => {}
                () = reply.closed() => return Ok(Duration::ZERO),
            },
            None => tokio::time::sleep(wait).await,
        }

        attempt += 1;
    }
}

async fn gather(
    target: NotificationTarget,
    first: Notice,
    receiver: &mut mpsc::Receiver<Item>,
) -> (Vec<Notice>, Option<Item>) {
    let mut batch = vec![first];

    if target != NotificationTarget::Discord {
        return (batch, None);
    }

    let started = Instant::now();

    loop {
        if batch.len() == discord::EMBEDS_MAX {
            return (batch, None);
        }

        let next = match receiver.try_recv() {
            Ok(item) => item,
            Err(TryRecvError::Disconnected) => return (batch, None),
            Err(TryRecvError::Empty) => {
                let wait = GATHER_QUIET.min(GATHER_MAX.saturating_sub(started.elapsed()));

                match tokio::time::timeout(wait, receiver.recv()).await {
                    Ok(Some(item)) => item,
                    _ => return (batch, None),
                }
            }
        };

        let Item::Notice(notice) = next else {
            return (batch, Some(next));
        };

        batch.push(notice);

        if !discord::fits(&batch) {
            let carried = batch.pop().map(Item::Notice);

            return (batch, carried);
        }
    }
}

fn losses_due(drained: bool, since_first: Duration) -> bool {
    drained || since_first >= LOSSES_SAID_AFTER
}

fn say_losses(target: NotificationTarget, dropped: usize, failure: Option<Error>) {
    let reason = failure.map_or_else(|| "too many waiting".to_owned(), |error| error.to_string());

    tracing::warn!(
        "{dropped} {} {} dropped: {reason}",
        name(target),
        if dropped == 1 {
            "notification"
        } else {
            "notifications"
        }
    );
}

async fn deliver(
    shared: Weak<Shared>,
    target: NotificationTarget,
    mut receiver: mpsc::Receiver<Item>,
    counters: Arc<Counters>,
) {
    let mut carried: Option<Item> = None;
    let mut failure: Option<Error> = None;
    let mut first_loss: Option<Instant> = None;

    loop {
        let first = match carried.take() {
            Some(item) => item,
            None => match receiver.recv().await {
                Some(item) => item,
                None => return,
            },
        };

        let pause = match first {
            Item::Test(notice, mut reply) => {
                let Some(live) = shared.upgrade() else {
                    return;
                };
                let outcome = send_with_retries(
                    &live,
                    target,
                    std::slice::from_ref(&notice),
                    Some(&mut reply),
                )
                .await;
                let pause = outcome.as_ref().copied().unwrap_or_default();

                let _ = reply.send(outcome.map(|_| ()));

                pause
            }
            Item::Notice(notice) => {
                let (batch, rest) = gather(target, notice, &mut receiver).await;
                let Some(live) = shared.upgrade() else {
                    return;
                };

                carried = rest;

                let outcome = send_with_retries(&live, target, &batch, None).await;

                lower(&counters.waiting, batch.len());

                outcome.unwrap_or_else(|error| {
                    counters.lost.fetch_add(batch.len(), Ordering::SeqCst);
                    failure = Some(error);

                    Duration::ZERO
                })
            }
        };

        if counters.lost.load(Ordering::SeqCst) > 0 {
            let since = *first_loss.get_or_insert_with(Instant::now);
            let drained = carried.is_none() && receiver.is_empty();

            if losses_due(drained, since.elapsed()) {
                say_losses(
                    target,
                    counters.lost.swap(0, Ordering::SeqCst),
                    failure.take(),
                );
                first_loss = None;
            }
        }

        if !pause.is_zero() {
            tokio::time::sleep(pause).await;
        }
    }
}
