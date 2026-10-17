//! Unit tests for job checkpoint protocol types.

use filesquid_core::{
    admit_active_transfers, reserve_gib, CanonicalReceipt, DomainError, JobCheckpoint, JobLease,
    OperationId, DEFAULT_ACTIVE_TRANSFERS, DEFAULT_CPU_HEAVY_WORKERS, DEFAULT_STAGING_CAP_GIB,
    MAX_ATTEMPTS_PER_RESOURCE, MIGRATION_PROCESS_MEMORY_GIB, RESERVE_MIN_GIB,
    RESTART_RESUMABLE_WITHIN_SECS, RETRY_BACKOFF_CAP_SECS,
};

fn op() -> OperationId {
    OperationId::new("op-job-1").expect("operation")
}

#[test]
fn checkpoint_is_not_an_ocfl_collection_snapshot() {
    let checkpoint = JobCheckpoint::new(op());
    assert_eq!(checkpoint.operation_id().as_str(), "op-job-1");
    assert!(!checkpoint.is_ocfl_collection_snapshot());
    assert_eq!(checkpoint.attempt(), 1);
}

#[test]
fn canonical_receipt_is_durable_completion() {
    let receipt = CanonicalReceipt::new(op());
    assert!(receipt.is_durable_completion());
    assert_eq!(receipt.operation_id().as_str(), "op-job-1");
}

#[test]
fn worker_caps_match_adr_007() {
    assert_eq!(DEFAULT_ACTIVE_TRANSFERS, 4);
    assert_eq!(DEFAULT_CPU_HEAVY_WORKERS, 2);
}

#[test]
fn staging_and_reserve_bounds() {
    assert_eq!(DEFAULT_STAGING_CAP_GIB, 10);
    assert_eq!(reserve_gib(100), 5);
    assert_eq!(reserve_gib(10), RESERVE_MIN_GIB);
    assert_eq!(RESERVE_MIN_GIB, 2);
}

#[test]
fn retry_and_restart_bounds() {
    assert_eq!(MAX_ATTEMPTS_PER_RESOURCE, 5);
    assert_eq!(RETRY_BACKOFF_CAP_SECS, 60);
    assert_eq!(RESTART_RESUMABLE_WITHIN_SECS, 30);
    assert_eq!(MIGRATION_PROCESS_MEMORY_GIB, 2);
}

#[test]
fn expired_lease_cannot_commit_after_replacement() {
    let mut lease = JobLease::grant(op(), 1);
    lease.take_over(2).expect("takeover");
    assert!(!lease.allows_commit(1));
    assert!(lease.allows_commit(2));
}

#[test]
fn take_over_rejects_non_increasing_token() {
    let mut lease = JobLease::grant(op(), 2);
    assert_eq!(
        lease.take_over(2).unwrap_err(),
        DomainError::NonIncreasingLease
    );
    assert!(lease.allows_commit(2));
}

#[test]
fn transfer_cap_rejects_five_active_transfers() {
    assert_eq!(
        admit_active_transfers(5).unwrap_err(),
        DomainError::TransferCapExceeded
    );
    admit_active_transfers(4).expect("at cap");
}
