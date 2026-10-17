//! Compatibility and parity register types for F-040.

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

use crate::DomainError;

/// Register status for a capability row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CapabilityStatus {
    /// Equivalent user outcome.
    Equivalent,
    /// Equivalent through a documented adapter.
    AdapterEquivalent,
    /// Partial / limited outcome.
    Limited,
    /// Preserved but not editable.
    PreservationOnly,
    /// Unavailable on this profile.
    Unavailable,
}

impl CapabilityStatus {
    /// Return whether this status counts as editable parity.
    pub fn counts_as_editable_parity(self) -> bool {
        matches!(self, Self::Equivalent | Self::AdapterEquivalent)
    }

    /// Return the status token.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Equivalent => "equivalent",
            Self::AdapterEquivalent => "adapter-equivalent",
            Self::Limited => "limited",
            Self::PreservationOnly => "preservation-only",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Platform, format, outcome, requirement, and evidence for one register row.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CapabilityInventory {
    platform: String,
    format: String,
    outcome: String,
    requirement_id: String,
    evidence: String,
}

impl CapabilityInventory {
    /// Construct inventory fields for a capability row.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::EmptyName`] when any field is empty.
    pub fn new(
        platform: impl Into<String>,
        format: impl Into<String>,
        outcome: impl Into<String>,
        requirement_id: impl Into<String>,
        evidence: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let inventory = Self {
            platform: platform.into(),
            format: format.into(),
            outcome: outcome.into(),
            requirement_id: requirement_id.into(),
            evidence: evidence.into(),
        };
        if inventory.platform.is_empty()
            || inventory.format.is_empty()
            || inventory.outcome.is_empty()
            || inventory.requirement_id.is_empty()
            || inventory.evidence.is_empty()
        {
            return Err(DomainError::EmptyName);
        }
        Ok(inventory)
    }

    /// Return the platform token.
    pub fn platform(&self) -> &str {
        &self.platform
    }

    /// Return the format token.
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Return the equivalent outcome.
    pub fn outcome(&self) -> &str {
        &self.outcome
    }

    /// Return the requirement id.
    pub fn requirement_id(&self) -> &str {
        &self.requirement_id
    }

    /// Return the test evidence token.
    pub fn evidence(&self) -> &str {
        &self.evidence
    }
}

impl<'de> Deserialize<'de> for CapabilityInventory {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Raw {
            platform: String,
            format: String,
            outcome: String,
            requirement_id: String,
            evidence: String,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(
            raw.platform,
            raw.format,
            raw.outcome,
            raw.requirement_id,
            raw.evidence,
        )
        .map_err(de::Error::custom)
    }
}

/// One capability row with platform, format, outcome, requirement, and evidence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CapabilityRow {
    id: String,
    platform: String,
    format: String,
    outcome: String,
    requirement_id: String,
    evidence: String,
    status: CapabilityStatus,
    parent_id: Option<String>,
}

impl CapabilityRow {
    /// Construct a capability row from an identifier and inventory.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::EmptyName`] when `id` is empty.
    pub fn new(
        id: impl Into<String>,
        status: CapabilityStatus,
        parent_id: Option<String>,
        inventory: CapabilityInventory,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            return Err(DomainError::EmptyName);
        }
        Ok(Self {
            id,
            platform: inventory.platform,
            format: inventory.format,
            outcome: inventory.outcome,
            requirement_id: inventory.requirement_id,
            evidence: inventory.evidence,
            status,
            parent_id,
        })
    }

    /// Return the row id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return status.
    pub fn status(&self) -> CapabilityStatus {
        self.status
    }

    /// Return the optional parent capability id.
    pub fn parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }

    /// Return whether this row contributes a supported (non-unavailable) format.
    pub fn is_supported(&self) -> bool {
        self.status != CapabilityStatus::Unavailable
    }

    /// Return the platform recorded on this row.
    pub fn platform(&self) -> &str {
        &self.platform
    }

    /// Return the format recorded on this row.
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Return the equivalent outcome recorded on this row.
    pub fn outcome(&self) -> &str {
        &self.outcome
    }

    /// Return the requirement id recorded on this row.
    pub fn requirement_id(&self) -> &str {
        &self.requirement_id
    }

    /// Return the test evidence recorded on this row.
    pub fn evidence(&self) -> &str {
        &self.evidence
    }
}

impl<'de> Deserialize<'de> for CapabilityRow {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Raw {
            id: String,
            platform: String,
            format: String,
            outcome: String,
            requirement_id: String,
            evidence: String,
            status: CapabilityStatus,
            parent_id: Option<String>,
        }
        let raw = Raw::deserialize(deserializer)?;
        let inventory = CapabilityInventory::new(
            raw.platform,
            raw.format,
            raw.outcome,
            raw.requirement_id,
            raw.evidence,
        )
        .map_err(de::Error::custom)?;
        Self::new(raw.id, raw.status, raw.parent_id, inventory).map_err(de::Error::custom)
    }
}

/// Nonempty register of capability rows.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CapabilityRegister {
    rows: Vec<CapabilityRow>,
}

impl CapabilityRegister {
    /// Build a register.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::EmptyCapabilitySet`] when `rows` is empty.
    /// Returns [`DomainError::EmptySupportedSet`] when every row is unavailable.
    pub fn new(rows: Vec<CapabilityRow>) -> Result<Self, DomainError> {
        if rows.is_empty() {
            return Err(DomainError::EmptyCapabilitySet);
        }
        if rows.iter().all(|row| !row.is_supported()) {
            return Err(DomainError::EmptySupportedSet);
        }
        Ok(Self { rows })
    }

    /// Return whether `id` is present.
    pub fn contains(&self, id: &str) -> bool {
        self.rows.iter().any(|row| row.id == id)
    }

    /// Return a row by id.
    pub fn row(&self, id: &str) -> Option<&CapabilityRow> {
        self.rows.iter().find(|row| row.id == id)
    }

    /// Return whether any supported set exists.
    pub fn supported_set_nonempty(&self) -> bool {
        self.rows.iter().any(CapabilityRow::is_supported)
    }

    /// Return whether an unavailable child remains visible (not concealed).
    pub fn exposes_unavailable_child(&self, id: &str) -> bool {
        self.rows.iter().any(|row| {
            row.id == id && row.status == CapabilityStatus::Unavailable && row.parent_id.is_some()
        })
    }
}

impl<'de> Deserialize<'de> for CapabilityRegister {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Raw {
            rows: Vec<CapabilityRow>,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.rows).map_err(de::Error::custom)
    }
}
