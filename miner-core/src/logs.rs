use std::collections::VecDeque;
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex, PoisonError, TryLockError};

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tracing::field::{Field, Visit};
use tracing::{Event, Level, Subscriber};
use tracing_appender::rolling::{InitError, RollingFileAppender, Rotation};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;
use ts_rs::TS;

pub const CAPACITY: usize = 4000;

pub const LOG_FILES_KEPT: usize = 14;

const OWN_TARGETS: [&str; 3] = ["miner_core", "twitch_miner", "twitchminer"];

pub fn daily_files(dir: &Path) -> std::result::Result<RollingFileAppender, InitError> {
    RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("miner.log")
        .max_log_files(LOG_FILES_KEPT)
        .build(dir)
}

pub struct Redacted<M>(pub M);

impl<'a, M: MakeWriter<'a>> MakeWriter<'a> for Redacted<M> {
    type Writer = RedactedLine<M::Writer>;

    fn make_writer(&'a self) -> Self::Writer {
        RedactedLine {
            inner: self.0.make_writer(),
            buffer: Vec::new(),
        }
    }
}

pub struct RedactedLine<W: Write> {
    inner: W,
    buffer: Vec<u8>,
}

impl<W: Write> RedactedLine<W> {
    fn emit(&mut self) -> io::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        let text = redact(&String::from_utf8_lossy(&self.buffer));
        self.buffer.clear();
        self.inner.write_all(text.as_bytes())
    }
}

impl<W: Write> Write for RedactedLine<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.buffer.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.emit()?;
        self.inner.flush()
    }
}

impl<W: Write> Drop for RedactedLine<W> {
    fn drop(&mut self) {
        let _ = self.emit();
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(repr(enum = name))]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<&Level> for LogLevel {
    fn from(level: &Level) -> Self {
        match *level {
            Level::ERROR => Self::Error,
            Level::WARN => Self::Warn,
            Level::INFO => Self::Info,
            Level::DEBUG => Self::Debug,
            Level::TRACE => Self::Trace,
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ! `pad`, not `write_str`: "{:<5}" lines the levels up in the report.
        f.pad(match self {
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(rename = "LogLineType")]
pub struct LogLine {
    pub id: u64,
    pub at: DateTime<Utc>,
    pub level: LogLevel,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub account: Option<String>,
    pub text: String,
}

#[derive(Default)]
struct Ring {
    lines: VecDeque<LogLine>,
    next_id: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LogFilter<'a> {
    pub account: Option<&'a str>,
    pub levels: Option<&'a [LogLevel]>,
}

impl LogFilter<'_> {
    fn keeps(&self, line: &LogLine) -> bool {
        self.account
            .is_none_or(|account| line.account.as_deref() == Some(account))
            && self
                .levels
                .is_none_or(|levels| levels.contains(&line.level))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogPage {
    pub lines: Vec<LogLine>,
    pub next_before_id: Option<u64>,
}

#[derive(Clone, Default)]
pub struct LogBuffer {
    ring: Arc<Mutex<Ring>>,
}

impl LogBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&self, line: LogLine) {
        let mut ring = self.ring.lock().unwrap_or_else(|p| p.into_inner());
        let id = ring.next_id;
        ring.next_id += 1;

        if ring.lines.len() == CAPACITY {
            ring.lines.pop_front();
        }

        ring.lines.push_back(LogLine { id, ..line });
    }

    pub fn page(&self, filter: &LogFilter, before_id: Option<u64>, limit: usize) -> LogPage {
        let ring = self.ring.lock().unwrap_or_else(|p| p.into_inner());
        let mut older = ring
            .lines
            .iter()
            .rev()
            .filter(|l| before_id.is_none_or(|before| l.id < before))
            .filter(|l| filter.keeps(l));
        let mut lines: Vec<LogLine> = older.by_ref().take(limit).cloned().collect();

        let next_before_id = match (lines.last(), older.next()) {
            (Some(oldest), Some(_)) => Some(oldest.id),
            _ => None,
        };

        lines.reverse();

        LogPage {
            lines,
            next_before_id,
        }
    }

    pub fn lines(&self, filter: &LogFilter, limit: usize) -> Vec<LogLine> {
        self.page(filter, None, limit).lines
    }

    pub fn export(&self, filter: &LogFilter, limit: usize) -> String {
        self.lines(filter, limit)
            .iter()
            .map(|l| {
                format!(
                    "{} {:<5} {}{}",
                    l.at.format("%Y-%m-%d %H:%M:%S"),
                    l.level,
                    l.account
                        .as_ref()
                        .map(|a| format!("[{a}] "))
                        .unwrap_or_default(),
                    l.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub fn redact(text: &str) -> String {
    static PATTERNS: OnceLock<Vec<(Regex, &'static str)>> = OnceLock::new();
    let patterns = PATTERNS.get_or_init(|| {
        vec![
            (Regex::new(r"(?i)(oauth|bearer)\s+[a-z0-9]{20,}").unwrap(), "$1 [redacted]"),
            (Regex::new(r"(?i)(token|sig|signature|auth_token|access_token|refresh_token|device_code)=([^&\s\x22']+)").unwrap(), "$1=[redacted]"),
            (Regex::new(r"(?i)\x22(access_token|refresh_token|auth_token|device_code|token|sig)\x22\s*:\s*\x22[^\x22]*\x22").unwrap(), "\"$1\":\"[redacted]\""),
            (Regex::new(r"(?i)(socks4a?|socks5h?|https?)://([^:/@\s]+):([^@\s]+)@").unwrap(), "$1://$2:[redacted]@"),
            (Regex::new(r"(?i)(discord(?:app)?\.com/api/webhooks/)[0-9]+/[A-Za-z0-9_-]+").unwrap(), "$1[redacted]"),
            (Regex::new(r"\bbot[0-9]+:[A-Za-z0-9_-]+").unwrap(), "bot[redacted]"),
            (Regex::new(r"\b[A-Za-z0-9]{30,}\b").unwrap(), "[redacted]"),
        ]
    });
    let mut out = text.to_owned();

    for (regex, replacement) in patterns {
        out = regex.replace_all(&out, *replacement).into_owned();
    }

    out
}

static FILE_WRITER: Mutex<Option<Box<dyn Send>>> = Mutex::new(None);

#[must_use = "the log file is written to the end when this is dropped"]
pub struct FileWriter(());

impl Drop for FileWriter {
    fn drop(&mut self) {
        drop(take_file_writer());
    }
}

pub fn keep_file_writer(guard: impl Send + 'static) -> FileWriter {
    *FILE_WRITER.lock().unwrap_or_else(PoisonError::into_inner) = Some(Box::new(guard));

    FileWriter(())
}

fn take_file_writer() -> Option<Box<dyn Send>> {
    match FILE_WRITER.try_lock() {
        Ok(mut writer) => writer.take(),
        Err(TryLockError::Poisoned(writer)) => writer.into_inner().take(),
        Err(TryLockError::WouldBlock) => None,
    }
}

pub fn log_panics() {
    static HOOK: std::sync::Once = std::sync::Once::new();

    HOOK.call_once(|| {
        let previous = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |info| {
            tracing::error!("panic: {info}");

            // ! With `panic = "abort"` the process ends right after this hook:
            // the file's writer thread is given the time to write the line first.
            if cfg!(panic = "abort") {
                drop(take_file_writer());
            }

            previous(info);
        }));
    });
}

#[derive(Default)]
struct Collector {
    message: String,
    account: Option<String>,
}

impl Visit for Collector {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        match field.name() {
            "message" => self.message = format!("{value:?}"),
            "account" => self.account = Some(format!("{value:?}").trim_matches('"').to_owned()),
            name => {
                if !self.message.is_empty() {
                    self.message.push(' ');
                }

                self.message.push_str(&format!("{name}={value:?}"));
            }
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "message" => self.message = value.to_owned(),
            "account" => self.account = Some(value.to_owned()),
            name => {
                if !self.message.is_empty() {
                    self.message.push(' ');
                }

                self.message.push_str(&format!("{name}={value}"));
            }
        }
    }
}

pub struct LogLayer {
    pub buffer: LogBuffer,
}

impl<S: Subscriber> Layer<S> for LogLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let metadata = event.metadata();

        if *metadata.level() > Level::INFO {
            return;
        }

        let target = metadata.target();
        let own = OWN_TARGETS.iter().any(|crate_name| {
            target
                .strip_prefix(crate_name)
                .is_some_and(|rest| rest.is_empty() || rest.starts_with("::"))
        });

        if !own {
            return;
        }

        let mut collector = Collector::default();
        event.record(&mut collector);

        self.buffer.push(LogLine {
            id: 0,
            at: Utc::now(),
            level: LogLevel::from(metadata.level()),
            target: target.rsplit("::").next().unwrap_or(target).to_owned(),
            account: collector.account,
            text: redact(&collector.message),
        });
    }
}
