//! Coverage boost: `event_builder.rs` (96 lines, 0%).
//!
//! Exercises all builder methods and build() output.

use wx_rust_cp::message::outxmlbuilder::EventBuilder;

#[test]
fn event_builder_new() {
    let builder = EventBuilder::new();
    let msg = builder.build();
    assert!(msg.event.is_none());
}

#[test]
fn event_builder_all_fields() {
    let msg = EventBuilder::new()
        .event("change_contact")
        .chat_id("chat_1")
        .change_type("create_user")
        .update_detail("detail_1")
        .join_scene("scene_1")
        .quit_scene("scene_2")
        .mem_change_cnt("5")
        .tag_type("tag_1")
        .strategy_id("strat_1")
        .user_id("user_1")
        .external_user_id("ext_user_1")
        .state("state_1")
        .source("source_1")
        .welcome_code("wc_1")
        .fail_reason("reason_1")
        .id("id_1")
        .to_user("to_user")
        .from_user("from_user")
        .build();
    assert_eq!(msg.event.as_deref(), Some("change_contact"));
    assert_eq!(msg.chat_id.as_deref(), Some("chat_1"));
    assert_eq!(msg.change_type.as_deref(), Some("create_user"));
    assert_eq!(msg.update_detail.as_deref(), Some("detail_1"));
    assert_eq!(msg.join_scene.as_deref(), Some("scene_1"));
    assert_eq!(msg.quit_scene.as_deref(), Some("scene_2"));
    assert_eq!(msg.mem_change_cnt.as_deref(), Some("5"));
    assert_eq!(msg.tag_type.as_deref(), Some("tag_1"));
    assert_eq!(msg.strategy_id.as_deref(), Some("strat_1"));
    assert_eq!(msg.user_id.as_deref(), Some("user_1"));
    assert_eq!(msg.external_user_id.as_deref(), Some("ext_user_1"));
    assert_eq!(msg.state.as_deref(), Some("state_1"));
    assert_eq!(msg.source.as_deref(), Some("source_1"));
    assert_eq!(msg.welcome_code.as_deref(), Some("wc_1"));
    assert_eq!(msg.fail_reason.as_deref(), Some("reason_1"));
    assert_eq!(msg.id.as_deref(), Some("id_1"));
}

#[test]
fn event_builder_partial_fields() {
    let msg = EventBuilder::new().event("subscribe").user_id("u1").build();
    assert_eq!(msg.event.as_deref(), Some("subscribe"));
    assert_eq!(msg.user_id.as_deref(), Some("u1"));
    assert!(msg.chat_id.is_none());
}

#[test]
fn event_builder_clone() {
    let builder = EventBuilder::new().event("e");
    let cloned = builder.clone();
    let msg = cloned.build();
    assert_eq!(msg.event.as_deref(), Some("e"));
}

#[test]
fn event_builder_debug() {
    let builder = EventBuilder::new();
    let dbg = format!("{builder:?}");
    assert!(dbg.contains("EventBuilder"));
}
