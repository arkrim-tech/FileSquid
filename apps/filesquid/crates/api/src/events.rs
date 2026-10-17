//! In-process event bus. Hosts subscribe and forward events to their UIs
//! (WebSocket on `filesquid-server`, Tauri events on `filesquid-desktop`).

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

/// Number of events buffered per subscriber before the oldest are dropped.
/// Bounded so a slow UI cannot grow memory without limit.
const EVENT_BUFFER: usize = 256;

/// One push event. `topic` is a dotted name (for example `job.progress`); `data` is its payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiEvent {
    /// Dotted event name.
    pub topic: String,
    /// Event payload; its shape is defined per topic in the API contract.
    pub data: serde_json::Value,
}

impl ApiEvent {
    /// Build an event from a topic and a JSON payload.
    pub fn new(topic: impl Into<String>, data: serde_json::Value) -> Self {
        Self {
            topic: topic.into(),
            data,
        }
    }
}

/// Broadcast bus for [`ApiEvent`]s.
///
/// The bus is process-wide and unscoped: every subscriber (each WebSocket session on
/// `filesquid-server`, each desktop webview) receives every event. Publishers MUST NOT emit
/// account-, permission- or collection-specific payloads until hosts filter events per
/// authorized session; no host has accounts yet, and nothing outside tests publishes.
#[derive(Clone, Debug)]
pub struct EventBus {
    sender: broadcast::Sender<ApiEvent>,
}

impl EventBus {
    /// Create an empty bus.
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(EVENT_BUFFER);
        Self { sender }
    }

    /// Publish an event to every current subscriber.
    ///
    /// Returns the number of subscribers that received it. Zero subscribers is not an error.
    pub fn publish(&self, event: ApiEvent) -> usize {
        self.sender.send(event).unwrap_or(0)
    }

    /// Number of live subscribers (for example, connected WebSocket sessions).
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }

    /// Subscribe to events published after this call.
    pub fn subscribe(&self) -> broadcast::Receiver<ApiEvent> {
        self.sender.subscribe()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
