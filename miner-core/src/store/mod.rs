pub mod vault;

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use zeroize::Zeroizing;

use crate::config::{AccountSettings, GlobalSettings};
use crate::error::{Error, Result};
use crate::events::{EventCategory, EventContext, EventKind, MinerEvent};
use crate::requests::{
    bucket_end, RequestEntry, RequestFilter, RequestGroup, RequestKind, RequestOutcome,
    RequestSource, SocketEvent,
};
use crate::spam::{SpamBalance, SpamRow};
use crate::twitch::auth::TokenSet;
use vault::{Sealed, Vault, VaultMode};

const DATABASE_FILE: &str = "miner.db";
const GLOBAL_SETTINGS_KEY: &str = "global";
const VAULT_MODE_KEY: &str = "vault_mode";
const VAULT_SALT_KEY: &str = "vault_salt";
const VAULT_CHECK_KEY: &str = "vault_check";
const MASKED: &str = "[redacted]";
const PRUNE_BATCH: usize = 5000;

// ! There is no migration: data this build cannot read is never replaced by
// defaults, the user is told to start from an empty folder.
const FOREIGN_DATA: &str =
    "the data folder was written by another version of the app: delete it, then sign in again";

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS setting (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS account (
    id           TEXT PRIMARY KEY,
    login        TEXT NOT NULL,
    display_name TEXT NOT NULL,
    avatar_url   TEXT,
    settings     TEXT NOT NULL,
    position     INTEGER NOT NULL,
    created_at   INTEGER NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS credential (
    account_id TEXT PRIMARY KEY REFERENCES account(id) ON DELETE CASCADE,
    nonce      TEXT NOT NULL,
    ciphertext TEXT NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS history (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id TEXT NOT NULL,
    at         INTEGER NOT NULL,
    kind       TEXT NOT NULL,
    title      TEXT NOT NULL,
    detail     TEXT,
    context    TEXT NOT NULL
) STRICT;
CREATE INDEX IF NOT EXISTS history_account ON history(account_id, id DESC);
CREATE INDEX IF NOT EXISTS history_at ON history(at);
CREATE INDEX IF NOT EXISTS history_kind ON history(kind);

CREATE TABLE IF NOT EXISTS request (
    id         INTEGER PRIMARY KEY,
    account_id TEXT,
    at         INTEGER NOT NULL,
    kind       TEXT NOT NULL,
    target     TEXT NOT NULL,
    socket     TEXT,
    outcome    TEXT NOT NULL,
    status     INTEGER,
    detail     TEXT,
    duration   INTEGER NOT NULL,
    count      INTEGER,
    longest    INTEGER
) STRICT;

CREATE TABLE IF NOT EXISTS spam_channel (
    account_id TEXT NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    login      TEXT NOT NULL,
    added_at   INTEGER,
    used_at    INTEGER,
    balance    INTEGER,
    balance_at INTEGER,
    PRIMARY KEY (account_id, login)
) STRICT;
";

// ! Says where and how the data broke, never the stored value: settings hold
// proxy passwords and webhook URLs.
fn foreign_data(what: &str, detail: impl fmt::Display) -> Error {
    Error::Storage(format!("{what} unreadable ({detail}): {FOREIGN_DATA}"))
}

fn json_error(error: &serde_json::Error) -> String {
    format!(
        "{:?} error at line {}, column {}",
        error.classify(),
        error.line(),
        error.column()
    )
}

fn put_setting(connection: &Connection, key: &str, value: &str) -> Result<()> {
    connection.execute(
        "INSERT INTO setting(key, value) VALUES(?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;

    Ok(())
}

// ! Only the conditions a page sets, so SQLite picks an index for them: a
// condition written `(?1 IS NULL OR ...)` makes it read the table row by row.
fn page_conditions(filter: &HistoryFilter, before: bool, table: &str) -> String {
    let mut conditions = Vec::new();

    if filter.account_id.is_some() {
        conditions.push(format!("{table}account_id = :account"));
    }

    if before {
        conditions.push(format!("{table}id < :before"));
    }

    conditions
        .iter()
        .map(|condition| format!(" AND {condition}"))
        .collect()
}

// ! One read per kind through the kind index, each cut at the page size: a
// plain `kind IN (...)` sorts every matching row. Kinds are fixed names,
// never user input: they are written into the SQL.
fn kinds_condition(filter: &HistoryFilter, before: bool) -> String {
    if filter.category.is_none() && filter.kinds.is_none() {
        return "1".to_owned();
    }

    let conditions = page_conditions(filter, before, "");
    let branches: Vec<String> = EventKind::ALL
        .into_iter()
        .filter(|kind| filter.kinds.is_none_or(|kinds| kinds.contains(kind)))
        .filter_map(|kind| {
            let miner = match filter.category {
                None => "",
                Some(EventCategory::Drops) if kind.follows_miner() => {
                    " AND json_extract(context, '$.miner') = 'DROPS'"
                }
                Some(EventCategory::Points) if kind.follows_miner() => {
                    " AND json_extract(context, '$.miner') = 'POINTS'"
                }
                Some(EventCategory::Other) if kind.follows_miner() => {
                    " AND json_extract(context, '$.miner') IS NULL"
                }
                Some(category) if kind.category(None) == category => "",
                Some(_) => return None,
            };

            Some(format!(
                "SELECT id FROM (SELECT id FROM history WHERE kind = '{}'{miner}{conditions}
                     ORDER BY id DESC LIMIT :limit)",
                kind.as_str()
            ))
        })
        .collect();

    if branches.is_empty() {
        return "0".to_owned();
    }

    format!("h.id IN ({})", branches.join(" UNION ALL "))
}

fn history_sql(filter: &HistoryFilter, before: bool) -> String {
    let kinds = kinds_condition(filter, before);
    let conditions = page_conditions(filter, before, "h.");

    format!(
        "SELECT h.id, h.account_id, h.at, h.kind, h.title, h.detail, h.context,
                COALESCE(a.login, '')
         FROM history h LEFT JOIN account a ON a.id = h.account_id
         WHERE {kinds}{conditions}
         ORDER BY h.id DESC LIMIT :limit"
    )
}

fn requests_sql(filter: &RequestFilter, before: bool) -> String {
    let mut conditions = vec!["at < :until".to_owned()];

    if before {
        conditions.push("id < :before".to_owned());
    }

    if filter.account_id.is_some() {
        conditions.push("account_id = :account".to_owned());
    }

    if let Some(kinds) = filter.kinds {
        let names: Vec<String> = kinds
            .iter()
            .map(|kind| format!("'{}'", kind.as_str()))
            .collect();

        conditions.push(if names.is_empty() {
            "0".to_owned()
        } else {
            format!("kind IN ({})", names.join(", "))
        });
    }

    if filter.errors_only {
        conditions.push(format!("outcome <> '{}'", RequestOutcome::Ok.as_str()));
    }

    format!(
        "SELECT id, account_id, at, kind, target, socket, outcome, status, detail, duration,
                count, longest
         FROM request WHERE {}
         ORDER BY id DESC LIMIT :limit",
        conditions.join(" AND ")
    )
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HistoryFilter<'a> {
    pub account_id: Option<&'a str>,
    pub category: Option<EventCategory>,
    pub kinds: Option<&'a [EventKind]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRecord {
    pub id: String,
    pub login: String,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub settings: AccountSettings,
    pub position: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Credential {
    pub tokens: TokenSet,
    pub device_id: String,
}

// ! The tokens stay out of `{:?}`: the log redaction only catches what looks
// like a key.
impl fmt::Debug for Credential {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Credential")
            .field("tokens", &MASKED)
            .field("device_id", &self.device_id)
            .finish()
    }
}

struct AccountRow {
    id: String,
    login: String,
    display_name: String,
    avatar_url: Option<String>,
    settings: String,
    position: i64,
    created_at: i64,
}

impl AccountRow {
    fn record(self) -> Result<AccountRecord> {
        let settings = serde_json::from_str(&self.settings)
            .map_err(|error| foreign_data("the settings of an account", json_error(&error)))?;
        let created_at = DateTime::from_timestamp(self.created_at, 0)
            .ok_or_else(|| foreign_data("the creation date of an account", "out of range"))?;

        Ok(AccountRecord {
            id: self.id,
            login: self.login,
            display_name: self.display_name,
            avatar_url: self.avatar_url,
            settings,
            position: self.position,
            created_at,
        })
    }
}

// ! Parsed outside the SQLite callback: that callback can only fail the whole
// page, and one broken entry must cost only itself.
struct HistoryRow {
    id: i64,
    account_id: String,
    account_login: String,
    at: i64,
    kind: String,
    title: String,
    detail: Option<String>,
    context: String,
}

impl HistoryRow {
    fn event(self) -> Result<MinerEvent> {
        let kind = EventKind::from_name(&self.kind)
            .ok_or_else(|| foreign_data("a history kind", "unknown name"))?;
        let context: EventContext = serde_json::from_str(&self.context)
            .map_err(|error| foreign_data("a history context", json_error(&error)))?;
        let at = DateTime::from_timestamp_millis(self.at)
            .ok_or_else(|| foreign_data("a history date", "out of range"))?;
        let id =
            u64::try_from(self.id).map_err(|_| foreign_data("a history id", "out of range"))?;
        let mut event = MinerEvent {
            id,
            account_id: self.account_id,
            account_login: self.account_login,
            at,
            kind,
            category: EventCategory::default(),
            title: self.title,
            detail: self.detail,
            context,
        };

        event.restore_miner();
        event.categorize();

        Ok(event)
    }
}

struct RequestRow {
    id: i64,
    account_id: Option<String>,
    at: i64,
    kind: String,
    target: String,
    socket: Option<String>,
    outcome: String,
    status: Option<i64>,
    detail: Option<String>,
    duration: i64,
    count: Option<i64>,
    longest: Option<i64>,
}

impl RequestRow {
    fn entry(self) -> Result<RequestEntry> {
        let unsigned = |value: i64, what: &str| {
            u64::try_from(value).map_err(|_| foreign_data(what, "out of range"))
        };
        let kind = RequestKind::from_name(&self.kind)
            .ok_or_else(|| foreign_data("a request kind", "unknown name"))?;
        let outcome = RequestOutcome::from_name(&self.outcome)
            .ok_or_else(|| foreign_data("a request outcome", "unknown name"))?;
        let socket = match self.socket.as_deref() {
            Some(name) => Some(
                SocketEvent::from_name(name)
                    .ok_or_else(|| foreign_data("a request socket event", "unknown name"))?,
            ),
            None => None,
        };
        let at = DateTime::from_timestamp_millis(self.at)
            .ok_or_else(|| foreign_data("a request date", "out of range"))?;
        let status = match self.status {
            Some(status) => Some(
                u16::try_from(status)
                    .map_err(|_| foreign_data("a request status", "out of range"))?,
            ),
            None => None,
        };
        let group = match (self.count, self.longest) {
            (Some(count), Some(longest)) => Some(RequestGroup {
                count: u32::try_from(count)
                    .map_err(|_| foreign_data("a request count", "out of range"))?,
                max_ms: unsigned(longest, "a request duration")?,
                until: bucket_end(at),
            }),
            _ => None,
        };

        Ok(RequestEntry {
            id: unsigned(self.id, "a request id")?,
            source: RequestSource::Stored,
            at,
            account_id: self.account_id,
            kind,
            target: self.target,
            socket,
            outcome,
            status,
            detail: self.detail,
            duration_ms: unsigned(self.duration, "a request duration")?,
            group,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(rename = "VaultStatusType")]
pub struct VaultStatus {
    pub mode: VaultMode,
    pub unlocked: bool,
}

#[derive(Clone)]
pub struct Store {
    connection: Arc<Mutex<Connection>>,
    pub dir: PathBuf,
}

impl Store {
    pub fn open(dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(dir).map_err(|e| Error::Storage(e.to_string()))?;

        let path = dir.join(DATABASE_FILE);

        create_private(&path)?;

        let connection = Connection::open(&path)?;

        // ! `secure_delete`: a deleted row (an old sealed credential) is
        // overwritten, not left in a free page.
        connection.execute_batch(
            "PRAGMA journal_mode = WAL; PRAGMA foreign_keys = ON; PRAGMA busy_timeout = 5000;
             PRAGMA secure_delete = ON;",
        )?;
        connection.execute_batch(SCHEMA)?;

        let store = Self {
            connection: Arc::new(Mutex::new(connection)),
            dir: dir.to_owned(),
        };

        store.verify()?;

        Ok(store)
    }

    fn verify(&self) -> Result<()> {
        self.global_settings()?;
        self.vault_mode()?;
        self.accounts()?;

        Ok(())
    }

    pub fn in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;

        connection.execute_batch("PRAGMA foreign_keys = ON;")?;
        connection.execute_batch(SCHEMA)?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            dir: std::env::temp_dir(),
        })
    }

    fn lock(&self) -> MutexGuard<'_, Connection> {
        crate::util::locked(&self.connection)
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        Ok(self
            .lock()
            .query_row("SELECT value FROM setting WHERE key = ?1", [key], |r| {
                r.get(0)
            })
            .optional()?)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        put_setting(&self.lock(), key, value)
    }

    pub fn delete_setting(&self, key: &str) -> Result<()> {
        self.lock()
            .execute("DELETE FROM setting WHERE key = ?1", [key])?;

        Ok(())
    }

    pub fn global_settings(&self) -> Result<GlobalSettings> {
        match self.get_setting(GLOBAL_SETTINGS_KEY)? {
            Some(json) => serde_json::from_str(&json)
                .map_err(|error| foreign_data("the app settings", json_error(&error))),
            None => Ok(GlobalSettings::default()),
        }
    }

    pub fn save_global_settings(&self, settings: &GlobalSettings) -> Result<()> {
        self.set_setting(GLOBAL_SETTINGS_KEY, &serde_json::to_string(settings)?)
    }

    pub fn vault_mode(&self) -> Result<VaultMode> {
        match self.get_setting(VAULT_MODE_KEY)? {
            Some(name) => VaultMode::from_name(&name)
                .ok_or_else(|| foreign_data("the vault mode", "unknown name")),
            None => Ok(VaultMode::KeyFile),
        }
    }

    pub fn open_default_vault(&self) -> Result<Option<Vault>> {
        match self.vault_mode()? {
            VaultMode::KeyFile => Ok(Some(Vault::from_key_file(&self.dir)?)),
            VaultMode::Passphrase => Ok(None),
        }
    }

    pub fn unlock_with_passphrase(&self, passphrase: &str) -> Result<Vault> {
        let salt = self
            .get_setting(VAULT_SALT_KEY)?
            .ok_or(Error::InvalidVault)?;
        let check: Sealed = serde_json::from_str(
            &self
                .get_setting(VAULT_CHECK_KEY)?
                .ok_or(Error::InvalidVault)?,
        )
        .map_err(|_| Error::InvalidVault)?;
        let vault = Vault::from_passphrase(passphrase, &salt)?;

        vault.verify(&check)?;

        Ok(vault)
    }

    pub fn rekey(
        &self,
        current: &Vault,
        next: &Vault,
        mode: VaultMode,
        salt: Option<&str>,
    ) -> Result<()> {
        let passphrase = match mode {
            VaultMode::Passphrase => Some((
                salt.ok_or(Error::InvalidVault)?,
                serde_json::to_string(&next.check_blob()?)?,
            )),
            VaultMode::KeyFile => None,
        };
        let mut connection = self.lock();
        let transaction = connection.transaction()?;
        let sealed: Vec<(String, Sealed)> = transaction
            .prepare("SELECT account_id, nonce, ciphertext FROM credential ORDER BY account_id")?
            .query_map([], |row| {
                Ok((
                    row.get(0)?,
                    Sealed {
                        nonce: row.get(1)?,
                        ciphertext: row.get(2)?,
                    },
                ))
            })?
            .collect::<rusqlite::Result<_>>()?;

        for (account_id, credential) in sealed {
            let resealed = next.seal(&current.open(&credential)?)?;

            transaction.execute(
                "UPDATE credential SET nonce = ?2, ciphertext = ?3 WHERE account_id = ?1",
                params![account_id, resealed.nonce, resealed.ciphertext],
            )?;
        }

        put_setting(&transaction, VAULT_MODE_KEY, mode.as_str())?;

        match &passphrase {
            Some((salt, check)) => {
                put_setting(&transaction, VAULT_SALT_KEY, salt)?;
                put_setting(&transaction, VAULT_CHECK_KEY, check)?;
            }
            None => {
                transaction.execute(
                    "DELETE FROM setting WHERE key IN (?1, ?2)",
                    params![VAULT_SALT_KEY, VAULT_CHECK_KEY],
                )?;
            }
        }

        transaction.commit()?;

        // ! The credentials sealed with the old key leave the write-ahead log for
        // good; best effort, the new ones are committed already.
        if let Err(error) = connection.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);") {
            tracing::debug!(%error, "the write-ahead log was not emptied after the rekey");
        }

        // ! Only once the passphrase seals everything: the old key opens
        // nothing any more.
        if passphrase.is_some() {
            vault::remove_key_file(&self.dir);
        }

        Ok(())
    }

    pub fn accounts(&self) -> Result<Vec<AccountRecord>> {
        let connection = self.lock();
        let mut statement = connection
            .prepare(
                "SELECT id, login, display_name, avatar_url, settings, position, created_at FROM account ORDER BY position, created_at",
            )
            .map_err(|error| foreign_data("the account table", error))?;
        let rows = statement
            .query_map([], |row| {
                Ok(AccountRow {
                    id: row.get(0)?,
                    login: row.get(1)?,
                    display_name: row.get(2)?,
                    avatar_url: row.get(3)?,
                    settings: row.get(4)?,
                    position: row.get(5)?,
                    created_at: row.get(6)?,
                })
            })
            .map_err(|error| foreign_data("the account table", error))?;
        let mut accounts = Vec::new();

        for row in rows {
            let row = row.map_err(|error| foreign_data("an account", error))?;

            accounts.push(row.record()?);
        }

        Ok(accounts)
    }

    pub fn account(&self, id: &str) -> Result<Option<AccountRecord>> {
        Ok(self.accounts()?.into_iter().find(|a| a.id == id))
    }

    pub fn upsert_account(&self, record: &AccountRecord) -> Result<()> {
        self.lock().execute(
            "INSERT INTO account(id, login, display_name, avatar_url, settings, position, created_at)
             VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET login = excluded.login, display_name = excluded.display_name,
                 avatar_url = excluded.avatar_url, settings = excluded.settings, position = excluded.position",
            params![
                record.id,
                record.login,
                record.display_name,
                record.avatar_url,
                serde_json::to_string(&record.settings)?,
                record.position,
                record.created_at.timestamp(),
            ],
        )?;

        Ok(())
    }

    pub fn save_account_settings(&self, id: &str, settings: &AccountSettings) -> Result<()> {
        let mut connection = self.lock();
        let transaction = connection.transaction()?;
        let changed = transaction.execute(
            "UPDATE account SET settings = ?2 WHERE id = ?1",
            params![id, serde_json::to_string(settings)?],
        )?;

        if changed == 0 {
            return Err(Error::UnknownAccount(id.to_owned()));
        }

        let balances_only: Vec<String> = transaction
            .prepare(
                "SELECT login FROM spam_channel
                 WHERE account_id = ?1 AND added_at IS NULL AND used_at IS NULL",
            )?
            .query_map(params![id], |row| row.get(0))?
            .collect::<rusqlite::Result<_>>()?;

        for login in balances_only {
            if !settings.points.streamers.iter().any(|s| s.login == login) {
                transaction.execute(
                    "DELETE FROM spam_channel WHERE account_id = ?1 AND login = ?2",
                    params![id, login],
                )?;
            }
        }

        transaction.commit()?;

        Ok(())
    }

    pub fn reorder_accounts(&self, ids: &[String]) -> Result<()> {
        let mut connection = self.lock();
        let transaction = connection.transaction()?;

        for (position, id) in ids.iter().enumerate() {
            transaction.execute(
                "UPDATE account SET position = ?2 WHERE id = ?1",
                params![id, position as i64],
            )?;
        }

        transaction.commit()?;

        Ok(())
    }

    pub fn delete_account(&self, id: &str) -> Result<()> {
        let mut connection = self.lock();
        let transaction = connection.transaction()?;

        transaction.execute("DELETE FROM history WHERE account_id = ?1", [id])?;
        transaction.execute("DELETE FROM request WHERE account_id = ?1", [id])?;
        transaction.execute("DELETE FROM account WHERE id = ?1", [id])?;
        transaction.commit()?;

        Ok(())
    }

    pub fn save_credential(
        &self,
        account_id: &str,
        credential: &Credential,
        vault: &Vault,
    ) -> Result<()> {
        let plain = Zeroizing::new(serde_json::to_vec(credential)?);
        let sealed = vault.seal(&plain)?;

        self.lock().execute(
            "INSERT INTO credential(account_id, nonce, ciphertext) VALUES(?1, ?2, ?3)
             ON CONFLICT(account_id) DO UPDATE SET nonce = excluded.nonce, ciphertext = excluded.ciphertext",
            params![account_id, sealed.nonce, sealed.ciphertext],
        )?;

        Ok(())
    }

    pub fn load_credential(&self, account_id: &str, vault: &Vault) -> Result<Option<Credential>> {
        let sealed: Option<Sealed> = self
            .lock()
            .query_row(
                "SELECT nonce, ciphertext FROM credential WHERE account_id = ?1",
                [account_id],
                |row| {
                    Ok(Sealed {
                        nonce: row.get(0)?,
                        ciphertext: row.get(1)?,
                    })
                },
            )
            .optional()?;
        let Some(sealed) = sealed else {
            return Ok(None);
        };
        let plain = vault.open(&sealed)?;

        Ok(Some(
            serde_json::from_slice(&plain).map_err(|_| Error::InvalidVault)?,
        ))
    }

    pub fn push_history(&self, event: &MinerEvent) -> Result<u64> {
        let stored = event.stored();
        let connection = self.lock();

        connection.execute(
            "INSERT INTO history(account_id, at, kind, title, detail, context) VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                stored.account_id,
                stored.at.timestamp_millis(),
                stored.kind.as_str(),
                stored.title,
                stored.detail,
                serde_json::to_string(&stored.context)?,
            ],
        )?;

        u64::try_from(connection.last_insert_rowid())
            .map_err(|_| Error::Storage("history id out of range".to_owned()))
    }

    pub fn history(
        &self,
        filter: &HistoryFilter,
        before_id: Option<u64>,
        limit: usize,
    ) -> Result<(Vec<MinerEvent>, Option<u64>)> {
        let connection = self.lock();
        let before = before_id.map(|id| i64::try_from(id).unwrap_or(i64::MAX));
        let mut statement = connection.prepare(&history_sql(filter, before.is_some()))?;
        let fetched = (limit + 1) as i64;
        let mut named: Vec<(&str, &dyn rusqlite::ToSql)> = vec![(":limit", &fetched)];

        if let Some(account_id) = &filter.account_id {
            named.push((":account", account_id));
        }

        if let Some(before) = &before {
            named.push((":before", before));
        }

        let mut rows: Vec<HistoryRow> = statement
            .query_map(named.as_slice(), |row| {
                Ok(HistoryRow {
                    id: row.get(0)?,
                    account_id: row.get(1)?,
                    at: row.get(2)?,
                    kind: row.get(3)?,
                    title: row.get(4)?,
                    detail: row.get(5)?,
                    context: row.get(6)?,
                    account_login: row.get(7)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?;

        // ! The page and its cursor follow the stored rows, so a skipped row
        // never ends the journal early.
        let next = if rows.len() > limit {
            rows.truncate(limit);
            rows.last().and_then(|row| u64::try_from(row.id).ok())
        } else {
            None
        };

        let events = rows
            .into_iter()
            .filter_map(|row| {
                let id = row.id;

                row.event()
                    .inspect_err(|error| tracing::warn!(id, "history entry skipped: {error}"))
                    .ok()
            })
            .collect();

        Ok((events, next))
    }

    pub fn prune_history(&self, cutoff: DateTime<Utc>) -> Result<usize> {
        let mut pruned = 0;

        loop {
            let deleted = self.lock().execute(
                "DELETE FROM history WHERE id IN (SELECT id FROM history WHERE at < ?1 LIMIT ?2)",
                params![cutoff.timestamp_millis(), PRUNE_BATCH as i64],
            )?;

            pruned += deleted;

            if deleted < PRUNE_BATCH {
                return Ok(pruned);
            }
        }
    }

    pub fn push_requests(&self, rows: &[RequestEntry]) -> Result<()> {
        let mut connection = self.lock();
        let transaction = connection.transaction()?;

        {
            let mut insert = transaction.prepare_cached(
                "INSERT INTO request(account_id, at, kind, target, socket, outcome, status, detail,
                     duration, count, longest)
                 VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            )?;

            for row in rows {
                insert.execute(params![
                    row.account_id,
                    row.at.timestamp_millis(),
                    row.kind.as_str(),
                    row.target,
                    row.socket.map(SocketEvent::as_str),
                    row.outcome.as_str(),
                    row.status,
                    row.detail,
                    i64::try_from(row.duration_ms).unwrap_or(i64::MAX),
                    row.group.as_ref().map(|group| group.count),
                    row.group
                        .as_ref()
                        .map(|group| i64::try_from(group.max_ms).unwrap_or(i64::MAX)),
                ])?;
            }
        }

        transaction.commit()?;

        Ok(())
    }

    pub fn requests(
        &self,
        filter: &RequestFilter,
        until: DateTime<Utc>,
        before_id: Option<u64>,
        limit: usize,
    ) -> Result<(Vec<RequestEntry>, Option<u64>)> {
        let connection = self.lock();
        let before = before_id.map(|id| i64::try_from(id).unwrap_or(i64::MAX));
        let mut statement = connection.prepare(&requests_sql(filter, before.is_some()))?;
        let fetched = (limit + 1) as i64;
        let until = until.timestamp_millis();
        let mut named: Vec<(&str, &dyn rusqlite::ToSql)> =
            vec![(":limit", &fetched), (":until", &until)];

        if let Some(account_id) = &filter.account_id {
            named.push((":account", account_id));
        }

        if let Some(before) = &before {
            named.push((":before", before));
        }

        let mut rows: Vec<RequestRow> = statement
            .query_map(named.as_slice(), |row| {
                Ok(RequestRow {
                    id: row.get(0)?,
                    account_id: row.get(1)?,
                    at: row.get(2)?,
                    kind: row.get(3)?,
                    target: row.get(4)?,
                    socket: row.get(5)?,
                    outcome: row.get(6)?,
                    status: row.get(7)?,
                    detail: row.get(8)?,
                    duration: row.get(9)?,
                    count: row.get(10)?,
                    longest: row.get(11)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?;

        let next = if rows.len() > limit {
            rows.truncate(limit);
            rows.last().and_then(|row| u64::try_from(row.id).ok())
        } else {
            None
        };

        let entries = rows
            .into_iter()
            .filter_map(|row| {
                let id = row.id;

                row.entry()
                    .inspect_err(|error| tracing::warn!(id, "request entry skipped: {error}"))
                    .ok()
            })
            .collect();

        Ok((entries, next))
    }

    pub fn prune_requests(&self, cutoff: DateTime<Utc>) -> Result<usize> {
        let mut pruned = 0;

        loop {
            let removed = self.lock().execute(
                "DELETE FROM request WHERE id IN (SELECT id FROM request WHERE at < ?1 LIMIT ?2)",
                params![cutoff.timestamp_millis(), PRUNE_BATCH as i64],
            )?;

            pruned += removed;

            if removed < PRUNE_BATCH {
                return Ok(pruned);
            }
        }
    }

    pub fn spam_rows(&self) -> Result<HashMap<String, Vec<SpamRow>>> {
        let connection = self.lock();
        let mut statement = connection.prepare(
            "SELECT account_id, login, added_at, used_at, balance, balance_at FROM spam_channel",
        )?;
        let rows = statement.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                SpamRow {
                    login: row.get(1)?,
                    added_at: row.get::<_, Option<i64>>(2)?.and_then(moment),
                    used_at: row.get::<_, Option<i64>>(3)?.and_then(moment),
                    balance: row.get(4)?,
                    balance_at: row.get::<_, Option<i64>>(5)?.and_then(moment),
                },
            ))
        })?;
        let mut by_account: HashMap<String, Vec<SpamRow>> = HashMap::new();

        for row in rows {
            let (account_id, row) = row?;

            by_account.entry(account_id).or_default().push(row);
        }

        Ok(by_account)
    }

    pub fn add_spam_channel(&self, account_id: &str, login: &str, at: DateTime<Utc>) -> Result<()> {
        self.lock().execute(
            "INSERT INTO spam_channel(account_id, login, added_at) VALUES(?1, ?2, ?3)
             ON CONFLICT(account_id, login) DO UPDATE
                 SET added_at = coalesce(spam_channel.added_at, excluded.added_at)",
            params![account_id, login, at.timestamp_millis()],
        )?;

        Ok(())
    }

    pub fn mark_spam_used(&self, account_id: &str, login: &str, at: DateTime<Utc>) -> Result<()> {
        self.lock().execute(
            "INSERT INTO spam_channel(account_id, login, used_at) VALUES(?1, ?2, ?3)
             ON CONFLICT(account_id, login) DO UPDATE SET used_at = excluded.used_at",
            params![account_id, login, at.timestamp_millis()],
        )?;

        Ok(())
    }

    pub fn record_spam_balance(
        &self,
        account_id: &str,
        login: &str,
        balance: i64,
        at: DateTime<Utc>,
        in_points: bool,
    ) -> Result<(SpamBalance, bool)> {
        let connection = self.lock();
        let kept: Option<(Option<i64>, Option<i64>)> = connection
            .query_row(
                "SELECT balance, balance_at FROM spam_channel WHERE account_id = ?1 AND login = ?2",
                params![account_id, login],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;

        if let Some((Some(kept), Some(changed_at))) = kept {
            if let Some(changed_at) = moment(changed_at).filter(|_| kept == balance) {
                return Ok((
                    SpamBalance {
                        balance,
                        changed_at,
                    },
                    false,
                ));
            }
        }

        if kept.is_none() && !in_points {
            return Ok((
                SpamBalance {
                    balance,
                    changed_at: at,
                },
                false,
            ));
        }

        connection.execute(
            "INSERT INTO spam_channel(account_id, login, balance, balance_at) VALUES(?1, ?2, ?3, ?4)
             ON CONFLICT(account_id, login) DO UPDATE SET balance = excluded.balance,
                 balance_at = excluded.balance_at",
            params![account_id, login, balance, at.timestamp_millis()],
        )?;

        Ok((
            SpamBalance {
                balance,
                changed_at: at,
            },
            true,
        ))
    }

    pub fn forget_spam_channel(&self, account_id: &str, login: &str) -> Result<()> {
        self.lock().execute(
            "DELETE FROM spam_channel WHERE account_id = ?1 AND login = ?2",
            params![account_id, login],
        )?;

        Ok(())
    }
}

fn moment(millis: i64) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp_millis(millis)
}

// ! Created private before SQLite opens it, so the file is never readable by
// others, not even for an instant; SQLite gives its -wal and -shm files the
// same mode.
fn create_private(path: &Path) -> Result<()> {
    let mut options = std::fs::OpenOptions::new();

    options.write(true).create(true).truncate(false);

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        options.mode(0o600);
    }

    options
        .open(path)
        .map_err(|error| Error::Storage(error.to_string()))?;

    Ok(())
}
