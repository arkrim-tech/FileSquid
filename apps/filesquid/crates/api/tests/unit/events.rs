//! Unit tests for the in-process event bus that hosts forward to UIs.

use filesquid_api::{ApiEvent, ApiState};

#[tokio::test]
async fn subscribers_receive_published_events() {
    let state = ApiState::new();
    let mut rx = state.events.subscribe();
    state.events.publish(ApiEvent::new(
        "job.progress",
        serde_json::json!({ "done": 1, "total": 2 }),
    ));
    let event = rx.recv().await.expect("event delivered");
    assert_eq!(event.topic, "job.progress");
    assert_eq!(event.data["done"], 1);
}

#[test]
fn publishing_without_subscribers_is_not_an_error() {
    let state = ApiState::new();
    let delivered = state
        .events
        .publish(ApiEvent::new("noop", serde_json::Value::Null));
    assert_eq!(delivered, 0);
}
