//! Namespaced statements and a metadata record.

use crate::{reject_file_dcterms_term, MetadataError, MetadataNamespace};

/// One populated key/value pair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statement {
    /// Namespace prefix (`exif`, `dcterms`, `schema`).
    pub namespace: MetadataNamespace,
    /// Local name (`title`, `GPSLatitude`).
    pub term: String,
    /// Literal value.
    pub value: String,
}

impl Statement {
    /// Prefix used in the UI, CSV, and CLI (`dcterms`).
    pub fn prefix(&self) -> &'static str {
        match self.namespace {
            MetadataNamespace::Exif => "exif",
            MetadataNamespace::Dcterms => "dcterms",
            MetadataNamespace::SchemaOrg => "schema",
        }
    }

    /// `prefix:term`.
    pub fn curie(&self) -> String {
        format!("{}:{}", self.prefix(), self.term)
    }
}

/// Canonical record stored as `metadata.md`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Record {
    /// Populated statements only.
    pub statements: Vec<Statement>,
    /// Optional Markdown body (long-form notes).
    pub body: String,
}

impl Record {
    /// Insert or replace a statement after register checks.
    pub fn set(
        &mut self,
        namespace: MetadataNamespace,
        term: &str,
        value: impl Into<String>,
    ) -> Result<(), MetadataError> {
        if namespace == MetadataNamespace::Dcterms {
            reject_file_dcterms_term(term)?;
        }
        let value = value.into();
        if let Some(existing) = self
            .statements
            .iter_mut()
            .find(|s| s.namespace == namespace && s.term == term)
        {
            existing.value = value;
            return Ok(());
        }
        self.statements.push(Statement {
            namespace,
            term: term.to_string(),
            value,
        });
        Ok(())
    }

    /// Find a statement by CURIE.
    pub fn get(&self, curie: &str) -> Option<&Statement> {
        self.statements.iter().find(|s| s.curie() == curie)
    }
}

/// Parse `exif:GPSLatitude` / `dcterms:title` / `schema:certification`.
pub fn parse_curie(curie: &str) -> Result<(MetadataNamespace, &str), MetadataError> {
    let Some((prefix, term)) = curie.split_once(':') else {
        return Err(MetadataError::UnknownDctermsTerm);
    };
    if term.is_empty() {
        return Err(MetadataError::UnknownDctermsTerm);
    }
    let namespace = match prefix {
        "exif" => MetadataNamespace::Exif,
        "dcterms" => MetadataNamespace::Dcterms,
        "schema" => MetadataNamespace::SchemaOrg,
        "dc" => {
            crate::map_dc_to_dcterms(term).ok_or(MetadataError::UnknownDctermsTerm)?;
            MetadataNamespace::Dcterms
        }
        _ => return Err(MetadataError::UnknownDctermsTerm),
    };
    if namespace == MetadataNamespace::Dcterms {
        let term = crate::map_dc_to_dcterms(term).unwrap_or(term);
        reject_file_dcterms_term(term)?;
        return Ok((namespace, term));
    }
    Ok((namespace, term))
}
