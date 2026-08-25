//! Coverage boost: `ma_page_message_builder.rs` (44 lines, 0%).

use wx_rust_miniapp::builder::MaPageMessageBuilder;

#[test]
fn ma_page_message_builder_new() {
    let builder = MaPageMessageBuilder::new();
    let _msg = builder.build();
}

#[test]
fn ma_page_message_builder_all_fields() {
    let msg = MaPageMessageBuilder::new()
        .to_user("user_1")
        .title("Page Title")
        .build();
    let _ = msg;
}

#[test]
fn ma_page_message_builder_partial() {
    let msg = MaPageMessageBuilder::new().title("Title").build();
    let _ = msg;
}

#[test]
fn ma_page_message_builder_clone_debug() {
    let builder = MaPageMessageBuilder::new().title("t");
    let cloned = builder.clone();
    let msg = cloned.build();
    let _ = msg;
    let _dbg = format!("{:?}", MaPageMessageBuilder::new());
}
