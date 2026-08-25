//! Coverage boost: `wx_cp_linked_corp_message.rs` (154 lines, 0%) and
//! `wx_cp_school_contact_message.rs` (136 lines, 0%).
//!
//! Exercises all `handle_msg_type` branches for both message types
//! (text/markdown/textcard/image/file/video/news/mpnews/miniprogram_notice
//! for linked_corp; text/image/file/voice/video/news/mpnews/miniprogram
//! for school_contact), plus top-level field serialization.

use std::collections::HashMap;

use wx_rust_cp::bean::article::{MpnewsArticle, NewArticle};
use wx_rust_cp::bean::message::WxCpLinkedCorpMessage;
use wx_rust_cp::bean::message::WxCpSchoolContactMessage;

// ========================================================================
// WxCpLinkedCorpMessage
// ========================================================================

#[test]
fn linked_corp_default_is_empty() {
    let msg = WxCpLinkedCorpMessage::default();
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["msgtype"], serde_json::Value::Null);
}

#[test]
fn linked_corp_text_message() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.msg_type = Some("text".into());
    msg.content = Some("hello".into());
    msg.to_users = vec!["u1".into(), "u2".into()];
    msg.to_parties = vec!["p1".into()];
    msg.to_tags = vec!["t1".into()];
    msg.is_to_all = Some(false);
    msg.agent_id = Some(1000002);
    msg.is_safe = Some(true);
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["touser"], serde_json::json!(["u1", "u2"]));
    assert_eq!(v["toparty"], serde_json::json!(["p1"]));
    assert_eq!(v["totag"], serde_json::json!(["t1"]));
    assert_eq!(v["toall"], 0);
    assert_eq!(v["msgtype"], "text");
    assert_eq!(v["agentid"], 1000002);
    assert_eq!(v["text"]["content"], "hello");
    assert_eq!(v["safe"], 1);
}

#[test]
fn linked_corp_to_all_true() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.is_to_all = Some(true);
    msg.msg_type = Some("text".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["toall"], 1);
}

#[test]
fn linked_corp_markdown_message() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.msg_type = Some("markdown".into());
    msg.content = Some("# title".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["markdown"]["content"], "# title");
}

#[test]
fn linked_corp_textcard_message() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.msg_type = Some("textcard".into());
    msg.title = Some("Card Title".into());
    msg.description = Some("Card Desc".into());
    msg.url = Some("https://example.com".into());
    msg.btn_txt = Some("Click".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["textcard"]["title"], "Card Title");
    assert_eq!(v["textcard"]["description"], "Card Desc");
    assert_eq!(v["textcard"]["url"], "https://example.com");
    assert_eq!(v["textcard"]["btntxt"], "Click");
}

#[test]
fn linked_corp_image_message() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.msg_type = Some("image".into());
    msg.media_id = Some("media_123".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["image"]["media_id"], "media_123");
}

#[test]
fn linked_corp_file_message() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.msg_type = Some("file".into());
    msg.media_id = Some("file_123".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["file"]["media_id"], "file_123");
}

#[test]
fn linked_corp_video_message() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.msg_type = Some("video".into());
    msg.media_id = Some("vid_1".into());
    msg.title = Some("Video Title".into());
    msg.description = Some("Video Desc".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["video"]["media_id"], "vid_1");
    assert_eq!(v["video"]["title"], "Video Title");
    assert_eq!(v["video"]["description"], "Video Desc");
}

#[test]
fn linked_corp_news_message() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.msg_type = Some("news".into());
    msg.articles = vec![NewArticle {
        title: "t1".into(),
        description: "d1".into(),
        url: "u1".into(),
        pic_url: "p1".into(),
        btn_text: "btn".into(),
        ..Default::default()
    }];
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["news"]["articles"][0]["title"], "t1");
    assert_eq!(v["news"]["articles"][0]["btntxt"], "btn");
}

#[test]
fn linked_corp_mpnews_with_media_id() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.msg_type = Some("mpnews".into());
    msg.media_id = Some("mp_media".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["mpnews"]["media_id"], "mp_media");
}

#[test]
fn linked_corp_mpnews_with_articles() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.msg_type = Some("mpnews".into());
    msg.mp_news_articles = vec![MpnewsArticle {
        title: "at1".into(),
        thumb_media_id: "thumb".into(),
        author: "auth".into(),
        content_source_url: "csu".into(),
        content: "body".into(),
        digest: "dig".into(),
        show_cover_pic: "1".into(),
    }];
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["mpnews"]["articles"][0]["title"], "at1");
    assert_eq!(v["mpnews"]["articles"][0]["thumb_media_id"], "thumb");
}

#[test]
fn linked_corp_miniprogram_notice() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.msg_type = Some("miniprogram_notice".into());
    msg.app_id = Some("wx_app".into());
    msg.page = Some("/pages/index".into());
    msg.title = Some("Notice Title".into());
    msg.description = Some("Notice Desc".into());
    msg.emphasis_first_item = Some(true);
    let mut items = HashMap::new();
    items.insert("k1".into(), "v1".into());
    msg.content_items = items;
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["miniprogram_notice"]["appid"], "wx_app");
    assert_eq!(v["miniprogram_notice"]["page"], "/pages/index");
    assert_eq!(v["miniprogram_notice"]["emphasis_first_item"], true);
    assert_eq!(v["miniprogram_notice"]["content_item"][0]["key"], "k1");
}

#[test]
fn linked_corp_unknown_msg_type() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.msg_type = Some("unknown_type".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["msgtype"], "unknown_type");
    // No message body for unknown type
    assert!(v.get("unknown_type").is_none());
}

#[test]
fn linked_corp_safe_false() {
    let mut msg = WxCpLinkedCorpMessage::default();
    msg.msg_type = Some("text".into());
    msg.is_safe = Some(false);
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["safe"], 0);
}

// ========================================================================
// WxCpSchoolContactMessage
// ========================================================================

#[test]
fn school_contact_default() {
    let msg = WxCpSchoolContactMessage::default();
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["msgtype"], serde_json::Value::Null);
}

#[test]
fn school_contact_text() {
    let mut msg = WxCpSchoolContactMessage::default();
    msg.msg_type = Some("text".into());
    msg.content = Some("hello parents".into());
    msg.recv_scope = Some(1);
    msg.to_parent_user_id = vec!["parent1".into()];
    msg.to_student_user_id = vec!["student1".into()];
    msg.to_party = vec!["dept1".into()];
    msg.to_all = Some(false);
    msg.agent_id = Some(1000002);
    msg.enable_id_trans = Some(true);
    msg.enable_duplicate_check = Some(true);
    msg.duplicate_check_interval = Some(1800);
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["recv_scope"], 1);
    assert_eq!(v["to_parent_userid"], serde_json::json!(["parent1"]));
    assert_eq!(v["to_student_userid"], serde_json::json!(["student1"]));
    assert_eq!(v["to_party"], serde_json::json!(["dept1"]));
    assert_eq!(v["toall"], 0);
    assert_eq!(v["msgtype"], "text");
    assert_eq!(v["agentid"], 1000002);
    assert_eq!(v["text"]["content"], "hello parents");
    assert_eq!(v["enable_id_trans"], 1);
    assert_eq!(v["enable_duplicate_check"], 1);
    assert_eq!(v["duplicate_check_interval"], 1800);
}

#[test]
fn school_contact_to_all_true() {
    let mut msg = WxCpSchoolContactMessage::default();
    msg.to_all = Some(true);
    msg.msg_type = Some("text".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["toall"], 1);
}

#[test]
fn school_contact_image() {
    let mut msg = WxCpSchoolContactMessage::default();
    msg.msg_type = Some("image".into());
    msg.media_id = Some("img_123".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["image"]["media_id"], "img_123");
}

#[test]
fn school_contact_file() {
    let mut msg = WxCpSchoolContactMessage::default();
    msg.msg_type = Some("file".into());
    msg.media_id = Some("file_123".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["file"]["media_id"], "file_123");
}

#[test]
fn school_contact_voice() {
    let mut msg = WxCpSchoolContactMessage::default();
    msg.msg_type = Some("voice".into());
    msg.media_id = Some("voice_123".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["voice"]["media_id"], "voice_123");
}

#[test]
fn school_contact_video() {
    let mut msg = WxCpSchoolContactMessage::default();
    msg.msg_type = Some("video".into());
    msg.media_id = Some("vid_1".into());
    msg.title = Some("Video Title".into());
    msg.description = Some("Video Desc".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["video"]["media_id"], "vid_1");
    assert_eq!(v["video"]["title"], "Video Title");
    assert_eq!(v["video"]["description"], "Video Desc");
}

#[test]
fn school_contact_news() {
    let mut msg = WxCpSchoolContactMessage::default();
    msg.msg_type = Some("news".into());
    msg.articles = vec![NewArticle {
        title: "News Title".into(),
        description: "News Desc".into(),
        url: "https://example.com".into(),
        pic_url: "https://img.example.com".into(),
        ..Default::default()
    }];
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["news"]["articles"][0]["title"], "News Title");
}

#[test]
fn school_contact_mpnews() {
    let mut msg = WxCpSchoolContactMessage::default();
    msg.msg_type = Some("mpnews".into());
    msg.mp_news_articles = vec![MpnewsArticle {
        title: "MP Title".into(),
        thumb_media_id: "thumb".into(),
        author: "auth".into(),
        content_source_url: "url".into(),
        content: "body".into(),
        digest: "dig".into(),
        show_cover_pic: "1".into(),
    }];
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["mpnews"]["articles"][0]["title"], "MP Title");
}

#[test]
fn school_contact_miniprogram() {
    let mut msg = WxCpSchoolContactMessage::default();
    msg.msg_type = Some("miniprogram".into());
    msg.app_id = Some("wx_app".into());
    msg.page_path = Some("/pages/index".into());
    msg.title = Some("Mini Title".into());
    msg.thumb_media_id = Some("mini_thumb".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["miniprogram"]["appid"], "wx_app");
    assert_eq!(v["miniprogram"]["pagepath"], "/pages/index");
    assert_eq!(v["miniprogram"]["title"], "Mini Title");
    assert_eq!(v["miniprogram"]["thumb_media_id"], "mini_thumb");
}

#[test]
fn school_contact_unknown_type() {
    let mut msg = WxCpSchoolContactMessage::default();
    msg.msg_type = Some("unknown".into());
    let json = msg.to_json();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["msgtype"], "unknown");
}
