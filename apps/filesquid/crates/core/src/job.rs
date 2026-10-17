//! Durable job checkpoint protocol types.
//!
//! Job checkpoints are in-progress job state. They are not OCFL collection
//! snapshots. Durable collection completion follows [`CanonicalReceipt`].

use serde::{Deserialize, Serialize};

use crate::{DomainError, OperationId};

/// Default maximum concurrent active transfers.
pub const DEFAULT_ACTIVE_TRANSFERS: u32 = 4;

/// Default maximum CPU-heavy workers.
pub const DEFAULT_CPU_HEAVY_WORKERS: u32 = 2;

/// Default staging cap in GiB.
pub const DEFAULT_STAGING_CAP_GIB: u64 = 10;

/// Minimum destination reserve in GiB.
pub const RESERVE_MIN_GIB: u64 = 2;

/// Destination reserve as a percent of capacity.
pub const RESERVE_PERCENT: u64 = 5;

/// Maximum attempts per resource, including the initial attempt.
pub const MAX_ATTEMPTS_PER_RESOURCE: u32 = 5;

/// Exponential retry backoff cap in seconds (jitter is applied by hosts).
pub const RETRY_BACKOFF_CAP_SECS: u64 = 60;

/// Restart MUST reach a resumable job state within this many seconds.
pub const RESTART_RESUMABLE_WITHIN_SECS: u64 = 30;

/// Proposed migration process memory bound in GiB, excluding ML models.
pub const MIGRATION_PROCESS_MEMORY_GIB: u64 = 2;

/// Destination reserve: max(2 GiB, 5% of destination capacity).
pub fn reserve_gib(destination_capacity_gib: u64) -> u64 {
    let five_percent = destination_capacity_gib * RESERVE_PERCENT / 100;
    five_percent.max(RESERVE_MIN_GIB)
}

/// Admit `count` active transfers against [`DEFAULT_ACTIVE_TRANSFERS`].
///
/// # Errors
///
/// Returns [`DomainError::TransferCapExceeded`] when `count` is greater than 4.
pub fn admit_active_transfers(count: u32) -> Result<(), DomainError> {
    if count > DEFAULT_ACTIVE_TRANSFERS {
        return Err(DomainError::TransferCapExceeded);
    }
    Ok(())
}

/// Durable in-progress job state. Not an OCFL collection snapshot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobCheckpoint {
    operation_id: OperationId,
    attempt: u32,
}

impl JobCheckpoint {
    /// Construct a checkpoint for the first attempt.
    pub fn new(operation_id: OperationId) -> Self {
        Self {
            operation_id,
            attempt: 1,
        }
    }

    /// Return the operation id.
    pub fn operation_id(&self) -> &OperationId {
        &self.operation_id
    }

    /// Return the current attempt number (1-based).
    pub fn attempt(&self) -> u32 {
        self.attempt
    }

    /// Job checkpoints are not OCFL collection snapshots.
    pub fn is_ocfl_collection_snapshot(&self) -> bool {
        false
    }
}

/// Canonical receipt that proves durable collection completion.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalReceipt {
    operation_id: OperationId,
}

impl CanonicalReceipt {
    /// Construct a receipt for a committed operation.
    pub fn new(operation_id: OperationId) -> Self {
        Self { operation_id }
    }

    /// Return the operation id.
    pub fn operation_id(&self) -> &OperationId {
        &self.operation_id
    }

    /// Canonical receipts are durable completion evidence.
    pub fn is_durable_completion(&self) -> bool {
        true
    }
}

/// Exclusive worker lease with a fencing token.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobLease {
    operation_id: OperationId,
    fencing_token: u64,
}

impl JobLease {
    /// Grant a lease to a worker.
    pub fn grant(operation_id: OperationId, fencing_token: u64) -> Self {
        Self {
            operation_id,
            fencing_token,
        }
    }

    /// Return the operation id.
    pub fn operation_id(&self) -> &OperationId {
        &self.operation_id
    }

    /// Return the fencing token owned by the current holder.
    pub fn fencing_token(&self) -> u64 {
        self.fencing_token
    }

    /// Replace the holder with a strictly newer fencing token.
    ///
    /// On error the lease is left unchanged so the current holder still owns it.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::NonIncreasingLease`] when `next_token` is not
    /// greater than the current token.
    pub fn take_over(&mut self, next_token: u64) -> Result<(), DomainError> {
        if next_token <= self.fencing_token {
            return Err(DomainError::NonIncreasingLease);
        }
        self.fencing_token = next_token;
        Ok(())
    }

    /// Return whether `holder_token` currently owns the lease.
    pub fn allows_commit(&self, holder_token: u64) -> bool {
        self.fencing_token == holder_token
    }
}
