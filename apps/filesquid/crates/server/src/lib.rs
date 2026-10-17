//! `filesquid-server`: the FileSquid server host.
//!
//! Serves the single FileSquid API ([`filesquid_api::router`]) over HTTP, pushes API events
//! to WebSocket clients at `GET /api/v1/events`, exposes Prometheus metrics, and serves one
//! distribution's built UI as static files ([ADR 030](../../../../../resources/architecture/030-filesquid-api-and-transports.md),
//! [ADR 032](../../../../../resources/architecture/032-hosting-and-privacy-model.md)).
//!
//! This host is for servers that the owner or their organisation operates: its operator
//! can read the files it stores. It MUST NOT be used as the ciphertext-only hosted service.

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::Response;
use axum::{routing::get, Router};
use filesquid_api::admin::{load_or_create_token, AdminAuth};
use filesquid_api::monitor::HostMonitor;
use filesquid_api::trace_layer::JournalLayer;
use filesquid_api::{ApiState, API_PREFIX};
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::sync::broadcast::error::RecvError;
use tower_http::services::{ServeDir, ServeFile};

/// Default listen address: loopback only, so a fresh install is not exposed on the network.
pub const DEFAULT_BIND: &str = "127.0.0.1:8083";

/// Default data directory (operational store and admin token), relative to the working directory.
pub const DEFAULT_DATA_DIR: &str = "filesquid-data";

/// Runtime configuration for the server host. `Debug` output redacts the admin token.
#[derive(Clone)]
pub struct ServerConfig {
    /// Address to listen on (`FILESQUID_BIND`).
    pub bind: SocketAddr,
    /// Directory of a built distribution UI to serve at `/` (`FILESQUID_UI_DIR`). `None` serves the API only.
    pub ui_dir: Option<PathBuf>,
    /// Directory for the operational store and the admin token (`FILESQUID_DATA_DIR`).
    pub data_dir: PathBuf,
    /// Admin token for the admin routes (`FILESQUID_ADMIN_TOKEN`); generated into the data dir when absent.
    pub admin_token: Option<String>,
}

impl std::fmt::Debug for ServerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServerConfig")
            .field("bind", &self.bind)
            .field("ui_dir", &self.ui_dir)
            .field("data_dir", &self.data_dir)
            .field(
                "admin_token",
                &self.admin_token.as_ref().map(|_| "[redacted]"),
            )
            .finish()
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind: DEFAULT_BIND
                .parse()
                .expect("DEFAULT_BIND is a valid socket address"),
            ui_dir: None,
            data_dir: PathBuf::from(DEFAULT_DATA_DIR),
            admin_token: None,
        }
    }
}

impl ServerConfig {
    /// Build configuration from a variable lookup (for example `std::env::var`).
    ///
    /// Returns an error message naming the variable when `FILESQUID_BIND` is not `host:port`.
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let bind_text = lookup("FILESQUID_BIND").unwrap_or_else(|| DEFAULT_BIND.to_string());
        let bind = bind_text
            .parse()
            .map_err(|err| format!("FILESQUID_BIND must be host:port, got {bind_text:?}: {err}"))?;
        let ui_dir = lookup("FILESQUID_UI_DIR").map(PathBuf::from);
        let data_dir = lookup("FILESQUID_DATA_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_DATA_DIR));
        let admin_token = lookup("FILESQUID_ADMIN_TOKEN");
        Ok(Self {
            bind,
            ui_dir,
            data_dir,
            admin_token,
        })
    }
}

/// Start the activity journal, process registry and admin routes for this server (FileSquid
/// F-061): opens `<data_dir>/filesquid.sqlite`, resolves the admin token (configured, stored,
/// or generated into `<data_dir>/admin-token`), and installs the monitor in `state`.
/// Performs filesystem and database I/O. The token itself is never logged.
pub fn start_host_monitor(
    config: &ServerConfig,
    state: &ApiState,
    layer: Option<&JournalLayer>,
) -> Result<HostMonitor, String> {
    let (token, created) = load_or_create_token(&config.data_dir, config.admin_token.as_deref())?;
    if created {
        tracing::info!(path = %config.data_dir.join("admin-token").display(), "generated admin token for the Monitor dashboard");
    }
    HostMonitor::open(
        &config.data_dir,
        "filesquid-server",
        AdminAuth::bearer(token),
        state,
        layer,
    )
}

/// Compose the server router: API, events, metrics, then the optional static UI.
pub fn app(config: &ServerConfig, state: ApiState) -> Router {
    let events = Router::new()
        .route(&format!("{API_PREFIX}/events"), get(events_handler))
        .route("/metrics", get(metrics_handler))
        .with_state(state.clone());
    let router = filesquid_api::router(state).merge(events);
    match &config.ui_dir {
        Some(dir) => router.fallback_service(static_ui(dir)),
        None => router,
    }
}

/// Serve a built UI directory; unknown paths fall back to `index.html` for client-side routing.
fn static_ui(dir: &std::path::Path) -> ServeDir<ServeFile> {
    ServeDir::new(dir).fallback(ServeFile::new(dir.join("index.html")))
}

/// `GET /api/v1/events`: upgrade to WebSocket and forward every API event as a JSON text frame.
async fn events_handler(ws: WebSocketUpgrade, State(state): State<ApiState>) -> Response {
    ws.on_upgrade(move |socket| forward_events(socket, state))
}

/// Forward bus events to one WebSocket until the client disconnects or the bus closes.
///
/// The socket is polled alongside the bus, so a client that leaves while no events flow
/// still ends this task and drops its bus subscription.
async fn forward_events(mut socket: WebSocket, state: ApiState) {
    let mut events = state.events.subscribe();
    loop {
        let next = tokio::select! {
            incoming = socket.recv() => {
                if client_gone(incoming) {
                    tracing::debug!("event client disconnected");
                    return;
                }
                continue;
            }
            next = events.recv() => next,
        };
        let event = match next {
            Ok(event) => event,
            Err(RecvError::Lagged(skipped)) => {
                tracing::warn!(skipped, "event subscriber lagged; events dropped");
                continue;
            }
            Err(RecvError::Closed) => return,
        };
        let text = match serde_json::to_string(&event) {
            Ok(text) => text,
            Err(err) => {
                tracing::error!(%err, "event serialization failed");
                continue;
            }
        };
        if socket.send(Message::Text(text.into())).await.is_err() {
            tracing::debug!("event client disconnected");
            return;
        }
    }
}

/// True when a WebSocket read shows the client has left: stream ended, read error, or a Close frame.
/// Other client frames are ignored; the event channel is push-only.
fn client_gone(incoming: Option<Result<Message, axum::Error>>) -> bool {
    matches!(incoming, None | Some(Err(_)) | Some(Ok(Message::Close(_))))
}

/// `GET /metrics`: Prometheus text from the same counters as `GET /api/v1/admin/metrics`.
async fn metrics_handler(State(state): State<ApiState>) -> String {
    match state.monitor() {
        Some(monitor) => monitor.metrics().to_prometheus(),
        None => "# HELP filesquid_up 1 while the FileSquid host is running\n# TYPE filesquid_up gauge\nfilesquid_up 1\n".to_string(),
    }
}
