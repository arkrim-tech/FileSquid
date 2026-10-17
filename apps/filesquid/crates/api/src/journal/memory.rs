//! In-memory journal store for tests and for hosts without a database.

use super::{
    retention_cutoff, retention_entry, JournalEntry, JournalError, JournalPage, JournalQuery,
    JournalStore, RetentionPolicy,
};
use std::sync::Mutex;

/// Journal store that keeps entries in memory (lost when the process ends).
pub struct MemoryJournal {
    host: String,
    state: Mutex<(Vec<JournalEntry>, i64)>,
}

impl MemoryJournal {
    /// Create an empty store for `host`.
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            state: Mutex::new((Vec::new(), 0)),
        }
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, (Vec<JournalEntry>, i64)>, JournalError> {
        self.state
            .lock()
            .map_err(|_| JournalError("memory journal lock poisoned".into()))
    }
}

impl JournalStore for MemoryJournal {
    fn host(&self) -> &str {
        &self.host
    }

    fn append(&self, entries: &[JournalEntry]) -> Result<Vec<i64>, JournalError> {
        let mut guard = self.lock()?;
        let mut ids = Vec::with_capacity(entries.len());
        for entry in entries {
            guard.1 += 1;
            let mut stored = entry.clone().redacted();
            stored.id = Some(guard.1);
            if stored.host.is_empty() {
                stored.host = self.host.clone();
            }
            ids.push(guard.1);
            guard.0.push(stored);
        }
        Ok(ids)
    }

    fn query(&self, query: &JournalQuery) -> Result<JournalPage, JournalError> {
        let guard = self.lock()?;
        let limit = query.effective_limit();
        let matching = |e: &&JournalEntry| query.past_cursor(e) && query.matches(e);
        let hits: Vec<JournalEntry> = if query.descending {
            guard
                .0
                .iter()
                .rev()
                .filter(matching)
                .take(limit + 1)
                .cloned()
                .collect()
        } else {
            guard
                .0
                .iter()
                .filter(matching)
                .take(limit + 1)
                .cloned()
                .collect()
        };
        Ok(JournalPage::from_overfetch(hits, limit))
    }

    fn count(&self, query: &JournalQuery) -> Result<u64, JournalError> {
        Ok(self.lock()?.0.iter().filter(|e| query.matches(e)).count() as u64)
    }

    fn apply_retention(&self, policy: RetentionPolicy, now_ms: i64) -> Result<u64, JournalError> {
        let Some(cutoff) = retention_cutoff(policy, now_ms) else {
            return Ok(0);
        };
        let deleted = {
            let mut guard = self.lock()?;
            let before = guard.0.len();
            guard.0.retain(|e| e.time_ms >= cutoff);
            (before - guard.0.len()) as u64
        };
        self.append(&[retention_entry(deleted, now_ms)])?;
        Ok(deleted)
    }

    fn ping(&self) -> Result<(), JournalError> {
        self.lock().map(|_| ())
    }
}
