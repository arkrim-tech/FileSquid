//! Versioned health payload served by every FileSquid host at `GET /api/v1/health`.

use serde::{Deserialize, Serialize};

/// Schema version for [`HealthStatus`]. Hosts MUST accept this version.
pub const HEALTH_SCHEMA_VERSION: u32 = 1;

/// Process health payload shared by every FileSquid host.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthStatus {
    /// Contract version for this payload. Currently [`HEALTH_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Always `"ok"` when the process can answer.
    pub status: &'static str,
    /// Product identifier (`filesquid`).
    pub product: &'static str,
}

/// Return the process health contract.
pub fn health() -> HealthStatus {
    HealthStatus {
        schema_version: HEALTH_SCHEMA_VERSION,
        status: "ok",
        product: "filesquid",
    }
}
