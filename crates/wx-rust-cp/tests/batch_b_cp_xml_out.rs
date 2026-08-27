#![allow(clippy::field_reassign_with_default, dead_code)]
//! Batch-B 镜像补测——CP XML 输出消息类。
//!
//! 本文件镜像以下 Java 测试类：
//! - WxCpXmlOutTextMessageTest（文本回复消息）
//! - WxCpXmlOutImageMessageTest（图片回复消息）
//! - WxCpXmlOutNewsMessageTest（图文回复消息）
//! - WxCpXmlOutTaskCardMessageTest（任务卡片回复消息）
//! - WxCpXmlOutVideoMessageTest（视频回复消息）
//! - WxCpXmlOutVoiceMessageTest（语音回复消息）

use wx_rust_cp::bean::message::wx_cp_xml_out_news_message::Item as NewsItem;
use wx_rust_cp::bean::message::*;

// ═══════════════════════════════════════════════════════════════
// WxCpXmlOutTextMessageTest（文本回复消息）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpXmlOutTextMessageTest（文本消息构建与 XML 序列化）
#[test]
fn test_cp_xml_out_text_message_build() {
    let mut msg = WxCpXmlOutTextMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.base.create_time = Some(1348831860);
    msg.content = Some("Hello World".to_string());
    let xml = msg.to_xml();
    assert!(xml.contains("<ToUserName><![CDATA[user001]]></ToUserName>"));
    assert!(xml.contains("<FromUserName><![CDATA[corp001]]></FromUserName>"));
    assert!(xml.contains("<MsgType><![CDATA[text]]></MsgType>"));
    assert!(xml.contains("<Content><![CDATA[Hello World]]></Content>"));
    assert!(xml.contains("<CreateTime>1348831860</CreateTime>"));
}

/// 对应 Java: WxCpXmlOutTextMessageTest（空内容文本消息）
#[test]
fn test_cp_xml_out_text_message_empty_content() {
    let mut msg = WxCpXmlOutTextMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.content = None;
    let xml = msg.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[text]]></MsgType>"));
    // 空内容不应包含 Content 标签
    assert!(!xml.contains("<Content>"));
}

/// 对应 Java: WxCpXmlOutTextMessageTest（特殊字符转义）
#[test]
fn test_cp_xml_out_text_message_special_chars() {
    let mut msg = WxCpXmlOutTextMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.content = Some("包含<特殊>&字符\"测试".to_string());
    let xml = msg.to_xml();
    assert!(xml.contains("<Content>"));
    // 内容应该被 CDATA 包裹，不需要 XML 转义
    assert!(xml.contains("包含<特殊>&字符\"测试"));
}

// ═══════════════════════════════════════════════════════════════
// WxCpXmlOutImageMessageTest（图片回复消息）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpXmlOutImageMessageTest（图片消息构建与 XML 序列化）
#[test]
fn test_cp_xml_out_image_message_build() {
    let mut msg = WxCpXmlOutImageMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.base.create_time = Some(1348831860);
    msg.media_id = Some("MEDIA_ID_001".to_string());
    let xml = msg.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[image]]></MsgType>"));
    assert!(xml.contains("<Image>"));
    assert!(xml.contains("<MediaId><![CDATA[MEDIA_ID_001]]></MediaId>"));
    assert!(xml.contains("</Image>"));
}

/// 对应 Java: WxCpXmlOutImageMessageTest（图片消息空 media_id）
#[test]
fn test_cp_xml_out_image_message_no_media() {
    let mut msg = WxCpXmlOutImageMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.media_id = None;
    let xml = msg.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[image]]></MsgType>"));
    // 空 media_id 不应生成 Image 标签
    assert!(!xml.contains("<Image>"));
}

// ═══════════════════════════════════════════════════════════════
// WxCpXmlOutNewsMessageTest（图文回复消息）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpXmlOutNewsMessageTest（单条图文消息构建）
#[test]
fn test_cp_xml_out_news_message_single() {
    let mut msg = WxCpXmlOutNewsMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.add_article(NewsItem {
        title: Some("图文标题".to_string()),
        description: Some("图文描述".to_string()),
        pic_url: Some("https://example.com/pic.jpg".to_string()),
        url: Some("https://example.com/article".to_string()),
    });
    let xml = msg.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[news]]></MsgType>"));
    assert!(xml.contains("<ArticleCount>1</ArticleCount>"));
    assert!(xml.contains("<Articles>"));
    assert!(xml.contains("<Title><![CDATA[图文标题]]></Title>"));
    assert!(xml.contains("<Description><![CDATA[图文描述]]></Description>"));
    assert!(xml.contains("<PicUrl><![CDATA[https://example.com/pic.jpg]]></PicUrl>"));
    assert!(xml.contains("<Url><![CDATA[https://example.com/article]]></Url>"));
}

/// 对应 Java: WxCpXmlOutNewsMessageTest（多条图文消息构建）
#[test]
fn test_cp_xml_out_news_message_multiple() {
    let mut msg = WxCpXmlOutNewsMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.add_article(NewsItem {
        title: Some("第一条".to_string()),
        description: Some("描述1".to_string()),
        pic_url: Some("https://example.com/pic1.jpg".to_string()),
        url: Some("https://example.com/1".to_string()),
    });
    msg.add_article(NewsItem {
        title: Some("第二条".to_string()),
        description: Some("描述2".to_string()),
        pic_url: Some("https://example.com/pic2.jpg".to_string()),
        url: Some("https://example.com/2".to_string()),
    });
    let xml = msg.to_xml();
    assert!(xml.contains("<ArticleCount>2</ArticleCount>"));
    assert!(xml.contains("第一条"));
    assert!(xml.contains("第二条"));
}

/// 对应 Java: WxCpXmlOutNewsMessageTest（空图文消息）
#[test]
fn test_cp_xml_out_news_message_empty() {
    let mut msg = WxCpXmlOutNewsMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    let xml = msg.to_xml();
    assert!(xml.contains("<ArticleCount>0</ArticleCount>"));
}

// ═══════════════════════════════════════════════════════════════
// WxCpXmlOutTaskCardMessageTest（任务卡片回复消息）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpXmlOutTaskCardMessageTest（任务卡片消息构建）
#[test]
fn test_cp_xml_out_task_card_message_build() {
    let mut msg = WxCpXmlOutTaskCardMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.replace_name = Some("同意".to_string());
    let xml = msg.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[update_taskcard]]></MsgType>"));
    assert!(xml.contains("<TaskCard><![CDATA[同意]]></TaskCard>"));
}

/// 对应 Java: WxCpXmlOutTaskCardMessageTest（任务卡片空替换名）
#[test]
fn test_cp_xml_out_task_card_message_empty_replace() {
    let mut msg = WxCpXmlOutTaskCardMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.replace_name = None;
    let xml = msg.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[update_taskcard]]></MsgType>"));
    // 空替换名不应生成 TaskCard 标签
    assert!(!xml.contains("<TaskCard>"));
}

// ═══════════════════════════════════════════════════════════════
// WxCpXmlOutVideoMessageTest（视频回复消息）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpXmlOutVideoMessageTest（视频消息构建与 XML 序列化）
#[test]
fn test_cp_xml_out_video_message_build() {
    let mut msg = WxCpXmlOutVideoMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.base.create_time = Some(1348831860);
    msg.video.media_id = Some("VIDEO_MEDIA_ID".to_string());
    msg.video.title = Some("视频标题".to_string());
    msg.video.description = Some("视频描述".to_string());
    let xml = msg.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[video]]></MsgType>"));
    assert!(xml.contains("<Video>"));
    assert!(xml.contains("<MediaId><![CDATA[VIDEO_MEDIA_ID]]></MediaId>"));
    assert!(xml.contains("<Title><![CDATA[视频标题]]></Title>"));
    assert!(xml.contains("<Description><![CDATA[视频描述]]></Description>"));
}

/// 对应 Java: WxCpXmlOutVideoMessageTest（视频消息最小字段）
#[test]
fn test_cp_xml_out_video_message_minimal() {
    let mut msg = WxCpXmlOutVideoMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.video.media_id = Some("VIDEO_ID".to_string());
    let xml = msg.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[video]]></MsgType>"));
    assert!(xml.contains("<MediaId><![CDATA[VIDEO_ID]]></MediaId>"));
}

// ═══════════════════════════════════════════════════════════════
// WxCpXmlOutVoiceMessageTest（语音回复消息）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpXmlOutVoiceMessageTest（语音消息构建与 XML 序列化）
#[test]
fn test_cp_xml_out_voice_message_build() {
    let mut msg = WxCpXmlOutVoiceMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.base.create_time = Some(1348831860);
    msg.media_id = Some("VOICE_MEDIA_ID".to_string());
    let xml = msg.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[voice]]></MsgType>"));
    assert!(xml.contains("<Voice>"));
    assert!(xml.contains("<MediaId><![CDATA[VOICE_MEDIA_ID]]></MediaId>"));
    assert!(xml.contains("</Voice>"));
}

/// 对应 Java: WxCpXmlOutVoiceMessageTest（语音消息空 media_id）
#[test]
fn test_cp_xml_out_voice_message_no_media() {
    let mut msg = WxCpXmlOutVoiceMessage::new();
    msg.base.to_user_name = Some("user001".to_string());
    msg.base.from_user_name = Some("corp001".to_string());
    msg.media_id = None;
    let xml = msg.to_xml();
    assert!(xml.contains("<MsgType><![CDATA[voice]]></MsgType>"));
    // 空 media_id 不应生成 Voice 标签
    assert!(!xml.contains("<Voice>"));
}
