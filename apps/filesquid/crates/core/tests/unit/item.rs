//! Unit tests for the canonical item type.

use filesquid_core::ItemType;

#[test]
fn item_type_is_file() {
    assert_eq!(ItemType::File.as_str(), "file");
    let json = serde_json::to_string(&ItemType::File).expect("json");
    assert_eq!(json, "\"file\"");
}

#[test]
fn photo_image_video_are_not_item_types() {
    for label in ["photo", "image", "video", "Photo", "Image", "Video"] {
        let json = format!("\"{label}\"");
        let parsed: Result<ItemType, _> = serde_json::from_str(&json);
        assert!(parsed.is_err(), "{label} must not deserialize as ItemType");
    }
}
