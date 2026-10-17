//! One set of counters and gauges, exposed as Prometheus text (`/metrics` on `filesquid-server`)
//! and as JSON (`GET /api/v1/admin/metrics`) so both always agree (FileSquid T-056).

use std::sync::atomic::{AtomicU64, Ordering};

/// Process-wide counters and gauges for FileSquid activity and health.
#[derive(Debug, Default)]
pub struct Metrics {
    journal_entries: AtomicU64,
    journal_write_failures: AtomicU64,
    journal_queue_depth: AtomicU64,
    warnings: AtomicU64,
    errors: AtomicU64,
    processes_started: AtomicU64,
    processes_completed: AtomicU64,
    processes_failed: AtomicU64,
    processes_cancelled: AtomicU64,
    processes_running: AtomicU64,
    processes_stalled: AtomicU64,
    health_status: AtomicU64,
}

/// Prometheus metric kind.
#[derive(Clone, Copy)]
enum Kind {
    Counter,
    Gauge,
}

impl Metrics {
    /// Count journal entries written to the store.
    pub fn journal_entry_written(&self, n: u64) {
        self.journal_entries.fetch_add(n, Ordering::Relaxed);
    }

    /// Count journal entries that could not be stored after retries.
    pub fn journal_write_failed(&self, n: u64) {
        self.journal_write_failures.fetch_add(n, Ordering::Relaxed);
    }

    /// Set the current journal queue depth.
    pub fn set_queue_depth(&self, depth: u64) {
        self.journal_queue_depth.store(depth, Ordering::Relaxed);
    }

    /// Count a journaled warning.
    pub fn warning(&self) {
        self.warnings.fetch_add(1, Ordering::Relaxed);
    }

    /// Count a journaled error.
    pub fn error(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Count a registered process.
    pub fn process_started(&self) {
        self.processes_started.fetch_add(1, Ordering::Relaxed);
    }

    /// Count a process that completed.
    pub fn process_completed(&self) {
        self.processes_completed.fetch_add(1, Ordering::Relaxed);
    }

    /// Count a process that failed.
    pub fn process_failed(&self) {
        self.processes_failed.fetch_add(1, Ordering::Relaxed);
    }

    /// Count a process that was cancelled.
    pub fn process_cancelled(&self) {
        self.processes_cancelled.fetch_add(1, Ordering::Relaxed);
    }

    /// Set the running and stalled process gauges.
    pub fn set_process_gauges(&self, running: u64, stalled: u64) {
        self.processes_running.store(running, Ordering::Relaxed);
        self.processes_stalled.store(stalled, Ordering::Relaxed);
    }

    /// Set the health gauge: 0 ok, 1 degraded, 2 failing.
    pub fn set_health(&self, level: u64) {
        self.health_status.store(level, Ordering::Relaxed);
    }

    /// All metrics as (name, kind, help, value), in a stable order.
    fn rows(&self) -> Vec<(&'static str, Kind, &'static str, u64)> {
        let g = |a: &AtomicU64| a.load(Ordering::Relaxed);
        vec![
            (
                "filesquid_up",
                Kind::Gauge,
                "1 while the FileSquid host is running",
                1,
            ),
            (
                "filesquid_journal_entries_total",
                Kind::Counter,
                "Activity journal entries written",
                g(&self.journal_entries),
            ),
            (
                "filesquid_journal_write_failures_total",
                Kind::Counter,
                "Journal entries that could not be stored",
                g(&self.journal_write_failures),
            ),
            (
                "filesquid_journal_queue_depth",
                Kind::Gauge,
                "Entries waiting to be written to the journal",
                g(&self.journal_queue_depth),
            ),
            (
                "filesquid_warnings_total",
                Kind::Counter,
                "Warnings journaled",
                g(&self.warnings),
            ),
            (
                "filesquid_errors_total",
                Kind::Counter,
                "Errors journaled",
                g(&self.errors),
            ),
            (
                "filesquid_processes_started_total",
                Kind::Counter,
                "Processes registered",
                g(&self.processes_started),
            ),
            (
                "filesquid_processes_completed_total",
                Kind::Counter,
                "Processes completed",
                g(&self.processes_completed),
            ),
            (
                "filesquid_processes_failed_total",
                Kind::Counter,
                "Processes failed",
                g(&self.processes_failed),
            ),
            (
                "filesquid_processes_cancelled_total",
                Kind::Counter,
                "Processes cancelled",
                g(&self.processes_cancelled),
            ),
            (
                "filesquid_processes_running",
                Kind::Gauge,
                "Processes running now",
                g(&self.processes_running),
            ),
            (
                "filesquid_processes_stalled",
                Kind::Gauge,
                "Processes stalled now",
                g(&self.processes_stalled),
            ),
            (
                "filesquid_health_status",
                Kind::Gauge,
                "Health: 0 ok, 1 degraded, 2 failing",
                g(&self.health_status),
            ),
        ]
    }

    /// JSON object mapping metric names to values.
    pub fn snapshot(&self) -> serde_json::Value {
        let map = self
            .rows()
            .into_iter()
            .map(|(name, _, _, v)| (name.to_string(), serde_json::Value::from(v)))
            .collect();
        serde_json::Value::Object(map)
    }

    /// Prometheus text exposition of the same values.
    pub fn to_prometheus(&self) -> String {
        self.rows()
            .into_iter()
            .map(|(name, kind, help, v)| {
                let kind = match kind {
                    Kind::Counter => "counter",
                    Kind::Gauge => "gauge",
                };
                format!("# HELP {name} {help}\n# TYPE {name} {kind}\n{name} {v}\n")
            })
            .collect()
    }
}
