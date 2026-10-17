//! Admin routes of the FileSquid API: health, processes, journal (query and JSON Lines export)
//! and metrics (SRS-F-061). Only the desktop owner or a server administrator may call them.

use crate::journal::{parse_rfc3339_ms, JournalQuery, Level, Outcome, MAX_PAGE};
use crate::monitor::Monitor;
use crate::ApiState;
use axum::extract::{Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;

/// Export stops after this many pages ([`MAX_PAGE`] entries each) to bound one response.
const MAX_EXPORT_PAGES: usize = 1_000;

/// Response header set when an export stopped at the export page cap (`MAX_EXPORT_PAGES`) with more entries
/// left; pass its value as `after` to continue.
pub const NEXT_CURSOR_HEADER: &str = "x-journal-next-cursor";

/// Who may call admin routes.
#[derive(Clone, Debug)]
pub enum AdminAuth {
    /// The desktop owner: the `filesquid://` scheme is reachable only from the app's webviews.
    Owner,
    /// Server administrators presenting `Authorization: Bearer <token>`. Interim until
    /// accounts and roles arrive (Task FS-1.4).
    Bearer(String),
}

impl AdminAuth {
    /// Bearer-token authorisation.
    pub fn bearer(token: impl Into<String>) -> Self {
        AdminAuth::Bearer(token.into())
    }

    /// Check request headers: `401` without credentials, `403` with wrong ones.
    pub fn check(&self, headers: &HeaderMap) -> Result<(), StatusCode> {
        let AdminAuth::Bearer(expected) = self else {
            return Ok(());
        };
        let presented = headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or(StatusCode::UNAUTHORIZED)?;
        if constant_time_eq(presented.as_bytes(), expected.as_bytes()) {
            Ok(())
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

/// Generate a random 256-bit admin token as 64 hex characters.
pub fn generate_token() -> Result<String, String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|e| e.to_string())?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// Compare without early exit so timing does not reveal the token.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// Admin routes, relative to the API prefix.
pub(crate) fn routes() -> Router<ApiState> {
    Router::new()
        .route("/admin/health", get(health))
        .route("/admin/processes", get(processes))
        .route("/admin/journal", get(journal))
        .route("/admin/journal/export", get(export))
        .route("/admin/metrics", get(metrics))
}

/// Resolve the monitor and check the caller (`503` before the monitor starts).
fn authorize(state: &ApiState, headers: &HeaderMap) -> Result<Monitor, StatusCode> {
    let monitor = state.monitor().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    monitor.auth().check(headers)?;
    Ok(monitor)
}

/// Run blocking store or filesystem work off the async executor.
async fn blocking<T: Send + 'static>(
    f: impl FnOnce() -> T + Send + 'static,
) -> Result<T, StatusCode> {
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn health(State(state): State<ApiState>, headers: HeaderMap) -> Response {
    let monitor = match authorize(&state, &headers) {
        Ok(m) => m,
        Err(status) => return status.into_response(),
    };
    match blocking(move || monitor.health_report()).await {
        Ok(report) => Json(report).into_response(),
        Err(status) => status.into_response(),
    }
}

async fn processes(State(state): State<ApiState>, headers: HeaderMap) -> Response {
    match authorize(&state, &headers) {
        Ok(m) => Json(m.registry().snapshot()).into_response(),
        Err(status) => status.into_response(),
    }
}

async fn metrics(State(state): State<ApiState>, headers: HeaderMap) -> Response {
    match authorize(&state, &headers) {
        Ok(m) => Json(m.metrics().snapshot()).into_response(),
        Err(status) => status.into_response(),
    }
}

/// Query-string filters for the journal routes.
#[derive(Debug, Default, Deserialize)]
struct JournalParams {
    from: Option<String>,
    to: Option<String>,
    actor: Option<String>,
    operation: Option<String>,
    outcome: Option<String>,
    level: Option<String>,
    process: Option<String>,
    after: Option<i64>,
    limit: Option<usize>,
    order: Option<String>,
}

impl JournalParams {
    /// Convert to a store query; `Err` names the invalid parameter.
    fn to_query(&self) -> Result<JournalQuery, &'static str> {
        let time = |v: &Option<String>, name| {
            v.as_deref()
                .map(|t| parse_rfc3339_ms(t).ok_or(name))
                .transpose()
        };
        Ok(JournalQuery {
            from_ms: time(&self.from, "from")?,
            to_ms: time(&self.to, "to")?,
            actor: self.actor.clone(),
            operation: self.operation.clone(),
            outcome: self
                .outcome
                .as_deref()
                .map(|o| Outcome::parse(o).ok_or("outcome"))
                .transpose()?,
            level: self
                .level
                .as_deref()
                .map(|l| Level::parse(l).ok_or("level"))
                .transpose()?,
            process_id: self.process.clone(),
            after_id: self.after,
            limit: self.limit.unwrap_or(0),
            descending: match self.order.as_deref() {
                None | Some("asc") => false,
                Some("desc") => true,
                Some(_) => return Err("order"),
            },
        })
    }
}

fn bad_request(param: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        format!("invalid query parameter: {param}"),
    )
        .into_response()
}

async fn journal(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Query(params): Query<JournalParams>,
) -> Response {
    let monitor = match authorize(&state, &headers) {
        Ok(m) => m,
        Err(status) => return status.into_response(),
    };
    let query = match params.to_query() {
        Ok(q) => q,
        Err(p) => return bad_request(p),
    };
    let store = monitor.store();
    match blocking(move || store.query(&query)).await {
        Ok(Ok(page)) => Json(page).into_response(),
        Ok(Err(e)) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        Err(status) => status.into_response(),
    }
}

async fn export(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Query(params): Query<JournalParams>,
) -> Response {
    let monitor = match authorize(&state, &headers) {
        Ok(m) => m,
        Err(status) => return status.into_response(),
    };
    let mut query = match params.to_query() {
        Ok(q) => q,
        Err(p) => return bad_request(p),
    };
    query.limit = MAX_PAGE;
    query.descending = false;
    let store = monitor.store();
    match blocking(move || export_pages(store.as_ref(), query, MAX_EXPORT_PAGES)).await {
        Ok(Ok((text, next))) => export_response(text, next),
        Ok(Err(e)) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
        Err(status) => status.into_response(),
    }
}

/// Read up to `max_pages` pages as JSON Lines (store I/O). Returns the text and, when
/// entries remain after the cap, the cursor to continue from.
fn export_pages(
    store: &dyn crate::journal::JournalStore,
    mut query: JournalQuery,
    max_pages: usize,
) -> Result<(String, Option<i64>), String> {
    let mut out = String::new();
    for _ in 0..max_pages {
        let page = store.query(&query).map_err(|e| e.to_string())?;
        for entry in &page.entries {
            out.push_str(&serde_json::to_string(entry).map_err(|e| e.to_string())?);
            out.push('\n');
        }
        let Some(cursor) = page.next_cursor else {
            return Ok((out, None));
        };
        query.after_id = Some(cursor);
    }
    Ok((out, query.after_id))
}

/// JSON Lines attachment; a truncated export carries [`NEXT_CURSOR_HEADER`].
fn export_response(text: String, next: Option<i64>) -> Response {
    let mut response = (
        [
            (header::CONTENT_TYPE, "application/x-ndjson"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"filesquid-journal.jsonl\"",
            ),
        ],
        text,
    )
        .into_response();
    if let Some(cursor) = next {
        response
            .headers_mut()
            .insert(NEXT_CURSOR_HEADER, header::HeaderValue::from(cursor));
    }
    response
}

/// Return the server admin token: `configured` if given, else the token stored in
/// `<data_dir>/admin-token`, else a new random token written there with owner-only
/// permissions (Unix mode 0600). Returns the token and whether it was created. Performs
/// filesystem I/O. Interim until accounts and roles arrive (Task FS-1.4).
pub fn load_or_create_token(
    data_dir: &std::path::Path,
    configured: Option<&str>,
) -> Result<(String, bool), String> {
    if let Some(token) = configured.filter(|t| !t.trim().is_empty()) {
        return Ok((token.trim().to_string(), false));
    }
    let path = data_dir.join("admin-token");
    if let Ok(existing) = std::fs::read_to_string(&path) {
        if !existing.trim().is_empty() {
            return Ok((existing.trim().to_string(), false));
        }
    }
    std::fs::create_dir_all(data_dir).map_err(|e| e.to_string())?;
    let token = generate_token()?;
    write_private(&path, &token)?;
    Ok((token, true))
}

/// Write `text` to `path`, readable and writable by the owner only on Unix.
fn write_private(path: &std::path::Path, text: &str) -> Result<(), String> {
    use std::io::Write;
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|e| format!("write {}: {e}", path.display()))?;
    file.write_all(text.as_bytes()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::journal::{JournalEntry, JournalStore, MemoryJournal};

    fn store_with(n: usize) -> MemoryJournal {
        let store = MemoryJournal::new("host-a");
        let entries: Vec<_> = (0..n)
            .map(|_| JournalEntry::new("ocfl.commit", Outcome::Succeeded))
            .collect();
        store.append(&entries).expect("append");
        store
    }

    #[test]
    fn an_export_that_hits_the_page_cap_returns_a_continuation_cursor() {
        let store = store_with(5);
        let query = JournalQuery {
            limit: 2,
            ..Default::default()
        };
        let (text, next) = export_pages(&store, query, 2).expect("export");
        assert_eq!(text.lines().count(), 4);
        assert_eq!(next, Some(4));
        let rest = JournalQuery {
            limit: 2,
            after_id: next,
            ..Default::default()
        };
        let (text, next) = export_pages(&store, rest, 2).expect("export");
        assert_eq!(text.lines().count(), 1);
        assert_eq!(next, None);
    }

    #[test]
    fn a_complete_export_has_no_continuation_cursor_header() {
        let (text, next) =
            export_pages(&store_with(3), JournalQuery::default(), 1).expect("export");
        assert_eq!(text.lines().count(), 3);
        assert!(export_response(text, next)
            .headers()
            .get(NEXT_CURSOR_HEADER)
            .is_none());
        let truncated = export_response(String::new(), Some(7));
        assert_eq!(truncated.headers()[NEXT_CURSOR_HEADER], "7");
    }
}
