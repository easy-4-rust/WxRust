//! Coverage boost: mp outxml builders (music_builder 47 lines, etc.).

use wx_rust_mp::builder::outxml::{
    DeviceBuilder, ImageBuilder, MusicBuilder, NewsBuilder, TextBuilder,
    TransferCustomerServiceBuilder, VideoBuilder, VoiceBuilder,
};

// ========================================================================
// MusicBuilder (47 lines, 0%)
// ========================================================================

#[test]
fn music_builder_new() {
    let builder = MusicBuilder::new();
    let _msg = builder.build();
}

#[test]
fn music_builder_all_fields() {
    let msg = MusicBuilder::new()
        .to_user("to_user")
        .from_user("from_user")
        .title("Music Title")
        .description("Music Description")
        .thumb_media_id("thumb_123")
        .music_url("https://music.example.com/song.mp3")
        .hq_music_url("https://music.example.com/song_hq.mp3")
        .build();
    assert!(msg.music.is_some());
    let music = msg.music.unwrap();
    assert_eq!(music.title.as_deref(), Some("Music Title"));
    assert_eq!(music.description.as_deref(), Some("Music Description"));
    assert_eq!(music.thumb_media_id.as_deref(), Some("thumb_123"));
    assert_eq!(
        music.music_url.as_deref(),
        Some("https://music.example.com/song.mp3")
    );
    assert_eq!(
        music.hq_music_url.as_deref(),
        Some("https://music.example.com/song_hq.mp3")
    );
}

#[test]
fn music_builder_partial() {
    let msg = MusicBuilder::new()
        .title("Title")
        .music_url("https://music.example.com/song.mp3")
        .build();
    let music = msg.music.unwrap();
    assert_eq!(music.title.as_deref(), Some("Title"));
    assert!(music.description.is_none());
}

#[test]
fn music_builder_clone_debug() {
    let builder = MusicBuilder::new().title("t");
    let cloned = builder.clone();
    let msg = cloned.build();
    assert_eq!(msg.music.unwrap().title.as_deref(), Some("t"));
    let _dbg = format!("{:?}", MusicBuilder::new());
}

// ========================================================================
// ImageBuilder
// ========================================================================

#[test]
fn image_builder_new() {
    let _msg = ImageBuilder::new().build();
}

#[test]
fn image_builder_all_fields() {
    let msg = ImageBuilder::new()
        .to_user("to_user")
        .from_user("from_user")
        .media_id("media_123")
        .build();
    let _ = msg;
}

// ========================================================================
// VoiceBuilder
// ========================================================================

#[test]
fn voice_builder_new() {
    let _msg = VoiceBuilder::new().build();
}

#[test]
fn voice_builder_all_fields() {
    let msg = VoiceBuilder::new()
        .to_user("to_user")
        .media_id("voice_123")
        .build();
    let _ = msg;
}

// ========================================================================
// VideoBuilder
// ========================================================================

#[test]
fn video_builder_new() {
    let _msg = VideoBuilder::new().build();
}

#[test]
fn video_builder_all_fields() {
    let msg = VideoBuilder::new()
        .to_user("to_user")
        .media_id("video_123")
        .title("Video Title")
        .description("Video Desc")
        .build();
    let _ = msg;
}

// ========================================================================
// TextBuilder
// ========================================================================

#[test]
fn text_builder_new() {
    let _msg = TextBuilder::new().build();
}

#[test]
fn text_builder_all_fields() {
    let msg = TextBuilder::new()
        .to_user("to_user")
        .content("Hello World")
        .build();
    assert_eq!(msg.content.as_deref(), Some("Hello World"));
}

// ========================================================================
// NewsBuilder
// ========================================================================

#[test]
fn news_builder_new() {
    let _msg = NewsBuilder::new().build();
}

// ========================================================================
// TransferCustomerServiceBuilder
// ========================================================================

#[test]
fn transfer_customer_service_builder_new() {
    let msg = TransferCustomerServiceBuilder::new().build();
    let _ = msg;
}

#[test]
fn transfer_customer_service_builder_with_kf_account() {
    let msg = TransferCustomerServiceBuilder::new()
        .kf_account("kf_account_1")
        .build();
    let _ = msg;
}

// ========================================================================
// DeviceBuilder
// ========================================================================

#[test]
fn device_builder_new() {
    let msg = DeviceBuilder::new().build();
    let _ = msg;
}
