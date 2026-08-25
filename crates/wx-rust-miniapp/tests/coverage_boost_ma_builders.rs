//! Coverage boost: miniapp builders (link_message_builder 50 lines,
//! image_message_builder, text_message_builder).

use wx_rust_miniapp::builder::ImageMessageBuilder;
use wx_rust_miniapp::builder::LinkMessageBuilder;
use wx_rust_miniapp::builder::TextMessageBuilder;

// ========================================================================
// LinkMessageBuilder (50 lines, 0%)
// ========================================================================

#[test]
fn link_message_builder_new() {
    let builder = LinkMessageBuilder::new();
    let msg = builder.build();
    assert_eq!(msg.msg_type.as_deref(), Some("link"));
    assert!(msg.link.is_some());
}

#[test]
fn link_message_builder_all_fields() {
    let msg = LinkMessageBuilder::new()
        .to_user("user_1")
        .title("Link Title")
        .description("Link Description")
        .url("https://example.com")
        .thumb_url("https://img.example.com/thumb.png")
        .build();
    assert_eq!(msg.to_user.as_deref(), Some("user_1"));
    assert_eq!(msg.msg_type.as_deref(), Some("link"));
    let link = msg.link.unwrap();
    assert_eq!(link.title.as_deref(), Some("Link Title"));
    assert_eq!(link.description.as_deref(), Some("Link Description"));
    assert_eq!(link.url.as_deref(), Some("https://example.com"));
    assert_eq!(
        link.thumb_url.as_deref(),
        Some("https://img.example.com/thumb.png")
    );
}

#[test]
fn link_message_builder_partial() {
    let msg = LinkMessageBuilder::new().title("Title").build();
    let link = msg.link.unwrap();
    assert_eq!(link.title.as_deref(), Some("Title"));
    assert!(link.description.is_none());
}

#[test]
fn link_message_builder_clone() {
    let builder = LinkMessageBuilder::new().title("t");
    let cloned = builder.clone();
    let msg = cloned.build();
    assert_eq!(msg.link.unwrap().title.as_deref(), Some("t"));
}

#[test]
fn link_message_builder_debug() {
    let builder = LinkMessageBuilder::new();
    let dbg = format!("{builder:?}");
    assert!(dbg.contains("LinkMessageBuilder"));
}

// ========================================================================
// ImageMessageBuilder
// ========================================================================

#[test]
fn image_message_builder_new() {
    let builder = ImageMessageBuilder::new();
    let msg = builder.build();
    assert_eq!(msg.msg_type.as_deref(), Some("image"));
}

#[test]
fn image_message_builder_all_fields() {
    let msg = ImageMessageBuilder::new()
        .to_user("user_1")
        .media_id("media_123")
        .build();
    assert_eq!(msg.to_user.as_deref(), Some("user_1"));
    let image = msg.image.unwrap();
    assert_eq!(image.media_id.as_deref(), Some("media_123"));
}

// ========================================================================
// TextMessageBuilder
// ========================================================================

#[test]
fn text_message_builder_new() {
    let builder = TextMessageBuilder::new();
    let msg = builder.build();
    assert_eq!(msg.msg_type.as_deref(), Some("text"));
}

#[test]
fn text_message_builder_all_fields() {
    let msg = TextMessageBuilder::new()
        .to_user("user_1")
        .content("Hello World")
        .build();
    assert_eq!(msg.to_user.as_deref(), Some("user_1"));
    let text = msg.text.unwrap();
    assert_eq!(text.content.as_deref(), Some("Hello World"));
}
