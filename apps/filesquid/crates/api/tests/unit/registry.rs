//! Unit tests for the process registry (K6): heartbeats, stalled detection, state journaling.

use filesquid_api::journal::{JournalQuery, JournalStore, MemoryJournal};
use filesquid_api::journal_writer::{JournalWriter, WriterConfig};
use filesquid_api::metrics::Metrics;
use filesquid_api::registry::{Clock, ProcessRegistry, ProcessSpec, ProcessState};
use filesquid_api::EventBus;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

struct Fixture {
    now: Arc<AtomicI64>,
    store: Arc<MemoryJournal>,
    writer: JournalWriter,
    registry: ProcessRegistry,
    events: EventBus,
}

fn fixture() -> Fixture {
    let now = Arc::new(AtomicI64::new(1_000_000));
    let clock_now = now.clone();
    let clock: Clock = Arc::new(move || clock_now.load(Ordering::SeqCst));
    let store = Arc::new(MemoryJournal::new("host-a"));
    let events = EventBus::new();
    let metrics = Arc::new(Metrics::default());
    let writer = JournalWriter::start(
        store.clone(),
        WriterConfig { capacity: 16 },
        events.clone(),
        metrics.clone(),
    );
    let registry = ProcessRegistry::new(writer.sender(), events.clone(), metrics, clock);
    Fixture {
        now,
        store,
        writer,
        registry,
        events,
    }
}

fn ops(f: &Fixture) -> Vec<String> {
    f.writer.sender().flush();
    f.store
        .query(&JournalQuery::default())
        .unwrap()
        .entries
        .into_iter()
        .map(|e| e.operation)
        .collect()
}

#[test]
fn registering_and_finishing_journals_state_changes() {
    let f = fixture();
    let handle = f.registry.register(ProcessSpec::new("import", 30_000));
    handle.progress(3, 10);
    handle.complete();
    assert_eq!(ops(&f), ["process.started", "process.completed"]);
    let view = f.registry.snapshot();
    assert_eq!(view[0].state, ProcessState::Completed);
    assert_eq!(view[0].done, 3);
    f.writer.shutdown();
}

#[test]
fn a_missed_heartbeat_marks_the_process_stalled_once() {
    let f = fixture();
    let mut rx = f.events.subscribe();
    let handle = f.registry.register(ProcessSpec::new("import", 30_000));
    f.now.fetch_add(31_000, Ordering::SeqCst);
    assert_eq!(f.registry.check_stalled().len(), 1);
    assert!(
        f.registry.check_stalled().is_empty(),
        "stalled is journaled once"
    );
    assert_eq!(f.registry.snapshot()[0].state, ProcessState::Stalled);
    let topics: Vec<String> = std::iter::from_fn(|| rx.try_recv().ok())
        .map(|e| e.topic)
        .collect();
    assert!(topics.contains(&"process.update".to_string()));
    handle.heartbeat();
    assert_eq!(f.registry.snapshot()[0].state, ProcessState::Running);
    assert_eq!(
        ops(&f),
        ["process.started", "process.stalled", "process.resumed"]
    );
    f.writer.shutdown();
}

#[test]
fn dropping_an_unfinished_handle_cancels_it_and_failure_is_journaled_as_error() {
    let f = fixture();
    {
        let _abandoned = f.registry.register(ProcessSpec::new("scan", 10_000));
    }
    let failing = f.registry.register(ProcessSpec::new("upload", 10_000));
    failing.fail("ENOSPC");
    assert_eq!(
        ops(&f),
        [
            "process.started",
            "process.cancelled",
            "process.started",
            "process.failed"
        ]
    );
    f.writer.sender().flush();
    let failed = f
        .store
        .query(&JournalQuery::default())
        .unwrap()
        .entries
        .pop()
        .unwrap();
    assert_eq!(failed.error_code.as_deref(), Some("ENOSPC"));
    f.writer.shutdown();
}

#[test]
fn rate_and_eta_follow_progress() {
    let f = fixture();
    let handle = f.registry.register(ProcessSpec::new("import", 30_000));
    f.now.fetch_add(10_000, Ordering::SeqCst);
    handle.progress(50, 100);
    let view = &f.registry.snapshot()[0];
    assert!((view.rate_per_second.unwrap() - 5.0).abs() < 1e-9);
    assert_eq!(view.eta_seconds, Some(10));
    handle.complete();
    f.writer.shutdown();
}

#[test]
fn finished_processes_are_pruned_when_they_finish_and_gauges_follow() {
    let f = fixture();
    let metrics = Arc::new(Metrics::default());
    let registry = ProcessRegistry::new(
        f.writer.sender(),
        f.events.clone(),
        metrics.clone(),
        Arc::new(|| 1_000_000),
    );
    let handles: Vec<_> = (0..205)
        .map(|_| registry.register(ProcessSpec::new("import", 30_000)))
        .collect();
    assert_eq!(metrics.snapshot()["filesquid_processes_running"], 205);
    handles.iter().for_each(|h| h.complete());
    assert_eq!(registry.snapshot().len(), 200);
    assert_eq!(metrics.snapshot()["filesquid_processes_running"], 0);
    f.writer.shutdown();
}
