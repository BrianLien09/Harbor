//! Device-local Zen cost history. This is not an account balance or invoice.
//! Reads only the provider, cost and ID columns of recent assistant records.

use std::{
    collections::HashSet,
    path::Path,
    time::{Duration, Instant},
};

use chrono::{DateTime, Utc};
use rusqlite::{Connection, OpenFlags, limits::Limit};
use serde::{Deserialize, Serialize};

const LOOKBACK_DAYS: i64 = 30;
const MAX_ROWS: usize = 25_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalSpend {
    pub micro_usd: u64,
    pub requests: u64,
}

impl LocalSpend {
    pub fn amount(self) -> String {
        let cents = self.micro_usd.saturating_add(5_000) / 10_000;
        format!("${}.{:02}", cents / 100, cents % 100)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenCodeDetails {
    pub go: bool,
    pub zen: bool,
    pub spend: Option<LocalSpend>,
}

pub(super) fn local_spend(path: &Path, now: DateTime<Utc>) -> rusqlite::Result<Option<LocalSpend>> {
    if !path.is_file() {
        return Ok(None);
    }
    let mut db = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;
    db.busy_timeout(Duration::from_millis(20))?;
    // Connection-local settings: no schema, index, journal or checkpoint changes.
    db.pragma_update(None, "query_only", true)?;
    db.pragma_update(None, "trusted_schema", false)?;
    db.pragma_update(None, "cache_size", -512)?;
    db.set_limit(Limit::SQLITE_LIMIT_LENGTH, 1024 * 1024)?;
    let started = Instant::now();
    let mut steps = 0;
    db.progress_handler(
        1_000,
        Some(move || {
            steps += 1;
            steps > 2_500 || started.elapsed() > Duration::from_millis(250)
        }),
    )?;
    read_spend(&db, now)
}

fn read_spend(db: &Connection, now: DateTime<Utc>) -> rusqlite::Result<Option<LocalSpend>> {
    let since = (now - chrono::Duration::days(LOOKBACK_DAYS)).timestamp_millis();
    let until = now.timestamp_millis();
    let mut seen = HashSet::new();
    let mut total = LocalSpend {
        micro_usd: 0,
        requests: 0,
    };
    // A migrated message can exist in both schemas. Prefer v2 and count its ID
    // only once; never sum step-finish parts on top of their assistant's cost.
    for (table, predicate) in [
        (
            "session_message",
            "type = 'assistant' AND json_extract(data, '$.model.providerID') = 'opencode'",
        ),
        (
            "message",
            "json_extract(data, '$.role') = 'assistant' AND json_extract(data, '$.providerID') = 'opencode'",
        ),
    ] {
        let exists: bool = db.query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_schema WHERE type = 'table' AND name = ?1)",
            [table],
            |row| row.get(0),
        )?;
        if !exists {
            continue;
        }
        // Table and predicate are compile-time constants, not external input.
        let sql = format!(
            "SELECT id, json_extract(data, '$.cost') FROM {table}
             WHERE time_created >= ?1 AND time_created <= ?2
             AND CASE WHEN json_valid(data) THEN ({predicate}
                 AND json_type(data, '$.cost') IN ('integer', 'real')) ELSE 0 END
             LIMIT {}",
            MAX_ROWS + 1
        );
        let mut statement = db.prepare(&sql)?;
        let mut rows = statement.query([since, until])?;
        let mut read = 0;
        while let Some(row) = rows.next()? {
            read += 1;
            if read > MAX_ROWS {
                return Err(rusqlite::Error::InvalidQuery);
            }
            let id: String = row.get(0)?;
            if id.len() > 256 {
                return Err(rusqlite::Error::InvalidQuery);
            }
            if !seen.insert(id) {
                continue;
            }
            if seen.len() > MAX_ROWS {
                return Err(rusqlite::Error::InvalidQuery);
            }
            let dollars: f64 = row.get(1)?;
            // Reject corrupt totals instead of presenting a partial/overflowed
            // number as a reliable financial measurement.
            if !dollars.is_finite() || !(0.0..=1_000_000.0).contains(&dollars) {
                return Err(rusqlite::Error::InvalidQuery);
            }
            total.micro_usd = total
                .micro_usd
                .checked_add((dollars * 1_000_000.0).round() as u64)
                .ok_or(rusqlite::Error::InvalidQuery)?;
            total.requests += 1;
        }
    }
    Ok((total.requests > 0).then_some(total))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    fn fixture(db: &Connection) -> Result<()> {
        db.execute_batch("CREATE TABLE message(id TEXT PRIMARY KEY, time_created INTEGER, data TEXT);
            CREATE TABLE session_message(id TEXT PRIMARY KEY, type TEXT, time_created INTEGER, data TEXT);")?;
        Ok(())
    }

    #[test]
    fn zen_cost_excludes_go_other_providers_user_messages_and_outside_the_period() -> Result<()> {
        let db = Connection::open_in_memory()?;
        fixture(&db)?;
        let now = Utc::now();
        for (id, role, provider, cost, time) in [
            ("zen", "assistant", "opencode", 1.25, now),
            ("free", "assistant", "opencode", 0.0, now),
            ("go", "assistant", "opencode-go", 12.0, now),
            ("codex", "assistant", "openai", 14.0, now),
            ("user", "user", "opencode", 15.0, now),
            (
                "old",
                "assistant",
                "opencode",
                20.0,
                now - chrono::Duration::days(31),
            ),
            (
                "future",
                "assistant",
                "opencode",
                30.0,
                now + chrono::Duration::days(1),
            ),
        ] {
            let data = serde_json::json!({"role": role, "providerID": provider, "cost": cost});
            db.execute(
                "INSERT INTO message VALUES (?1, ?2, ?3)",
                rusqlite::params![id, time.timestamp_millis(), data.to_string()],
            )?;
        }
        assert_eq!(
            read_spend(&db, now)?,
            Some(LocalSpend {
                micro_usd: 1_250_000,
                requests: 2
            })
        );
        Ok(())
    }

    #[test]
    fn migrated_messages_are_counted_once_and_invalid_json_is_ignored() -> Result<()> {
        let db = Connection::open_in_memory()?;
        fixture(&db)?;
        let now = Utc::now();
        let legacy = r#"{"role":"assistant","providerID":"opencode","cost":5}"#;
        let current = r#"{"model":{"providerID":"opencode"},"cost":2.5}"#;
        db.execute(
            "INSERT INTO message VALUES ('same', ?1, ?2)",
            rusqlite::params![now.timestamp_millis(), legacy],
        )?;
        db.execute(
            "INSERT INTO session_message VALUES ('same','assistant',?1,?2)",
            rusqlite::params![now.timestamp_millis(), current],
        )?;
        db.execute(
            "INSERT INTO message VALUES ('broken',?1,'invalid json')",
            [now.timestamp_millis()],
        )?;
        assert_eq!(
            read_spend(&db, now)?,
            Some(LocalSpend {
                micro_usd: 2_500_000,
                requests: 1
            })
        );
        Ok(())
    }

    #[test]
    fn missing_history_is_unknown_and_corrupt_costs_do_not_become_zero() -> Result<()> {
        let db = Connection::open_in_memory()?;
        fixture(&db)?;
        let now = Utc::now();
        assert_eq!(read_spend(&db, now)?, None);
        db.execute(
            "INSERT INTO message VALUES ('bad',?1,?2)",
            rusqlite::params![
                now.timestamp_millis(),
                r#"{"role":"assistant","providerID":"opencode","cost":-4}"#
            ],
        )?;
        assert!(read_spend(&db, now).is_err());
        Ok(())
    }

    #[test]
    fn excessive_history_returns_unknown_instead_of_a_truncated_total() -> Result<()> {
        let db = Connection::open_in_memory()?;
        fixture(&db)?;
        let now = Utc::now();
        db.execute(
            "WITH RECURSIVE n(x) AS (VALUES(1) UNION ALL SELECT x+1 FROM n WHERE x < ?1)
             INSERT INTO message SELECT 'msg_' || x, ?2,
             '{\"role\":\"assistant\",\"providerID\":\"opencode\",\"cost\":0.01}' FROM n",
            rusqlite::params![i64::try_from(MAX_ROWS + 1)?, now.timestamp_millis()],
        )?;
        assert!(read_spend(&db, now).is_err());
        Ok(())
    }

    #[test]
    fn wal_reads_do_not_modify_the_database_or_block_its_writer() -> Result<()> {
        let root = std::env::temp_dir().join(format!(
            "harbor-zen-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        ));
        std::fs::create_dir(&root)?;
        let result = (|| -> Result<()> {
            let path = root.join("opencode.db");
            let db = Connection::open(&path)?;
            db.pragma_update(None, "journal_mode", "WAL")?;
            fixture(&db)?;
            let now = Utc::now();
            db.execute(
                "INSERT INTO message VALUES ('zen',?1,?2)",
                rusqlite::params![
                    now.timestamp_millis(),
                    r#"{"role":"assistant","providerID":"opencode","cost":0.25}"#
                ],
            )?;
            let before = std::fs::read(&path)?;
            db.execute_batch("BEGIN IMMEDIATE")?;
            assert_eq!(
                local_spend(&path, now)?.map(LocalSpend::amount).as_deref(),
                Some("$0.25")
            );
            db.execute_batch("ROLLBACK")?;
            assert_eq!(std::fs::read(&path)?, before);
            assert!(local_spend(&root.join("missing.db"), now)?.is_none());
            assert!(!root.join("missing.db").exists());
            Ok(())
        })();
        std::fs::remove_dir_all(&root)?;
        result
    }
}
