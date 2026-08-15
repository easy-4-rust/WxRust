//! Phase 1 Batch 1.2: wx-rust-mp 核心 Service 测试
//!
//! 镜像 Java WxMpUserServiceImplTest / WxMpQrcodeServiceImplTest /
//! WxMpMenuServiceImplTest / WxMpKefuServiceImplTest

use wx_rust_mp::bean::kefu::*;
use wx_rust_mp::bean::material::*;
use wx_rust_mp::bean::menu::*;
use wx_rust_mp::bean::result::*;
use wx_rust_mp::bean::tag::*;
use wx_rust_mp::bean::template::*;

// ═══ WxMpUser ═══

#[test]
fn test_user_info_serde() {
    let json = r#"{"subscribe":true,"openid":"ox123","nickname":"test","language":"zh_CN","headimgurl":"http://img.example.com","subscribe_time":1700000000,"unionid":"union1","remark":"测试备注","groupid":100,"tagid_list":[101,102],"subscribe_scene":"ADD_SCENE_SEARCH","qr_scene":"0","qr_scene_str":""}"#;
    let user: WxMpUser = serde_json::from_str(json).unwrap();
    assert_eq!(user.open_id, "ox123");
    assert_eq!(user.nickname, "test");
    assert_eq!(user.subscribe, Some(true));
    assert_eq!(user.remark, "测试备注");
    assert_eq!(user.subscribe_scene, "ADD_SCENE_SEARCH");
    assert_eq!(user.group_id, Some(100));
}

#[test]
fn test_user_info_from_json() {
    let json = r#"{"subscribe":true,"openid":"ox456","nickname":"user2"}"#;
    let user = WxMpUser::from_json(json).unwrap();
    assert_eq!(user.open_id, "ox456");
    assert_eq!(user.nickname, "user2");
}

#[test]
fn test_change_openid_serde() {
    let json = r#"{"oriOpenid":"old-ox","newOpenid":"new-ox","errMsg":"ok"}"#;
    let change: WxMpChangeOpenid = serde_json::from_str(json).unwrap();
    assert_eq!(change.ori_openid, "old-ox");
    assert_eq!(change.new_openid, "new-ox");
}

// ═══ WxMpQrCodeTicket ═══

#[test]
fn test_qrcode_ticket_from_json() {
    let json =
        r#"{"ticket":"TICKET-001","expire_seconds":600,"url":"https://weixin.qq.com/q/abc"}"#;
    let ticket = WxMpQrCodeTicket::from_json(json).unwrap();
    assert_eq!(ticket.ticket, "TICKET-001");
    assert_eq!(ticket.expire_seconds, 600);
    assert!(ticket.url.contains("weixin.qq.com"));
}

#[test]
fn test_qrcode_ticket_permanent() {
    let json =
        r#"{"ticket":"PERM-TICKET","expire_seconds":-1,"url":"https://weixin.qq.com/q/perm"}"#;
    let ticket = WxMpQrCodeTicket::from_json(json).unwrap();
    assert_eq!(ticket.expire_seconds, -1);
}

#[test]
fn test_qrcode_ticket_roundtrip() {
    let json = r#"{"ticket":"T1","expire_seconds":300,"url":"http://t.example.com"}"#;
    let ticket = WxMpQrCodeTicket::from_json(json).unwrap();
    let serialized = serde_json::to_string(&ticket).unwrap();
    let deserialized = WxMpQrCodeTicket::from_json(&serialized).unwrap();
    assert_eq!(ticket.ticket, deserialized.ticket);
    assert_eq!(ticket.expire_seconds, deserialized.expire_seconds);
}

// ═══ WxMpKefuMessage ═══

#[test]
fn test_kefu_text_message_serde() {
    let json = r#"{"touser":"ox123","msgtype":"text","content":"Hello World"}"#;
    let msg: WxMpKefuMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.to_user, Some("ox123".to_string()));
    assert_eq!(msg.msg_type, Some("text".to_string()));
    assert_eq!(msg.content, Some("Hello World".to_string()));
}

#[test]
fn test_kefu_image_message_serde() {
    let json = r#"{"touser":"ox123","msgtype":"image","media_id":"media-001"}"#;
    let msg: WxMpKefuMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.to_user, Some("ox123".to_string()));
    assert_eq!(msg.msg_type, Some("image".to_string()));
    assert_eq!(msg.media_id, Some("media-001".to_string()));
}

#[test]
fn test_kefu_mpnews_message_serde() {
    let json = r#"{"touser":"ox123","msgtype":"mpnews","mp_news_media_id":"news-001"}"#;
    let msg: WxMpKefuMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.mp_news_media_id, Some("news-001".to_string()));
}

#[test]
fn test_kefu_miniprogram_message_serde() {
    let json = r#"{"touser":"ox123","msgtype":"miniprogrampage","mini_program_app_id":"wx-app","mini_program_page_path":"pages/index"}"#;
    let msg: WxMpKefuMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.mini_program_app_id, Some("wx-app".to_string()));
}

// ═══ WxMpKfList ═══

#[test]
fn test_kf_list_serde() {
    let json = r#"{"kf_list":[{"kf_account":"test1@test","kf_nick":"客服1","kf_id":"1001","kf_headimgurl":"http://img.example.com/1.jpg"}]}"#;
    let list: WxMpKfList = serde_json::from_str(json).unwrap();
    assert_eq!(list.kf_list.len(), 1);
    assert_eq!(list.kf_list[0].account, "test1@test");
    assert_eq!(list.kf_list[0].nick, "客服1");
}

#[test]
fn test_kf_online_list_serde() {
    let json = r#"{"kf_online_list":[{"kf_account":"test1@test","status":1,"kf_id":"1001","accepted_case":5}]}"#;
    let list: WxMpKfOnlineList = serde_json::from_str(json).unwrap();
    assert_eq!(list.kf_online_list.len(), 1);
    assert_eq!(list.kf_online_list[0].account, "test1@test");
}

// ═══ WxMpMenu ═══

#[test]
fn test_mp_menu_serde() {
    let json = r#"{"menu":{"button":[{"type":"click","name":"今日歌曲","key":"V1001_TODAY_MUSIC"},{"type":"view","name":"搜索","url":"http://www.soso.com/"}]}}"#;
    let menu: WxMpMenu = serde_json::from_str(json).unwrap();
    assert!(menu.menu.is_some());
    let inner = menu.menu.unwrap();
    assert_eq!(inner.buttons.len(), 2);
    assert_eq!(inner.buttons[0].name, "今日歌曲");
    assert_eq!(inner.buttons[1].url, "http://www.soso.com/");
}

#[test]
fn test_mp_menu_with_conditional() {
    let json = r#"{"menu":{"button":[]},"conditionalmenu":[{"button":[{"type":"click","name":"VIP专属","key":"VIP_KEY"}],"matchrule":{"sex":"1","country":"中国","province":"广东"}}]}"#;
    let menu: WxMpMenu = serde_json::from_str(json).unwrap();
    assert_eq!(menu.conditional_menu.len(), 1);
}

#[test]
fn test_mp_menu_from_json() {
    let json = r#"{"menu":{"button":[{"type":"click","name":"test","key":"KEY"}]}}"#;
    let menu = WxMpMenu::from_json(json).unwrap();
    assert!(menu.menu.is_some());
}

// ═══ WxMpTemplateMessage ═══

#[test]
fn test_template_message_serde() {
    let json = r#"{"touser":"ox123","template_id":"tpl-001","url":"http://example.com","data":[{"name":"first","value":"Hello"},{"name":"keyword1","value":"World"}]}"#;
    let msg: WxMpTemplateMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.to_user, Some("ox123".to_string()));
    assert_eq!(msg.template_id, Some("tpl-001".to_string()));
}

#[test]
fn test_template_serde() {
    let json = r#"{"template_id":"tpl-002","title":"Order Notification","primary_industry":"IT科技","deputy_industry":"互联网|电子商务","content":"订单 {{order_id}} 已发货","example":"订单 12345 已发货"}"#;
    let tpl: WxMpTemplate = serde_json::from_str(json).unwrap();
    assert_eq!(tpl.template_id, "tpl-002");
    assert_eq!(tpl.title, "Order Notification");
}

// ═══ WxUserTag ═══

#[test]
fn test_user_tag_serde() {
    let json = r#"{"id":101,"name":"VIP用户","count":50}"#;
    let tag: WxUserTag = serde_json::from_str(json).unwrap();
    assert_eq!(tag.id, 101);
    assert_eq!(tag.name, "VIP用户");
}

#[test]
fn test_tag_list_user_serde() {
    let json = r#"{"count":100,"data":{"openid":["ox1","ox2","ox3"]},"next_openid":"ox3"}"#;
    let result: WxTagListUser = serde_json::from_str(json).unwrap();
    assert_eq!(result.count, 100);
    assert_eq!(result.data.openid_list.len(), 3);
}

// ═══ Material Bean ═══

#[test]
fn test_material_serde() {
    let json = r#"{"name":"test-video","videoTitle":"My Video","videoIntroduction":"Description"}"#;
    let material: WxMpMaterial = serde_json::from_str(json).unwrap();
    assert_eq!(material.name, "test-video");
    assert_eq!(material.video_title, "My Video");
}

#[test]
fn test_news_article_serde() {
    let json = r#"{"title":"Article","thumb_media_id":"thumb-001","author":"Author","digest":"Summary","show_cover_pic":1,"content":"Content","content_source_url":"http://src.example.com","need_open_comment":1,"only_fans_can_comment":0}"#;
    let article: WxMpNewsArticle = serde_json::from_str(json).unwrap();
    assert_eq!(article.title, "Article");
    assert_eq!(article.author, "Author");
}

// ═══ VALUE_ADD ═══

#[test]
fn test_user_empty_json() {
    let user: WxMpUser = serde_json::from_str("{}").unwrap();
    assert_eq!(user.open_id, "");
    assert_eq!(user.subscribe, None);
}

#[test]
fn test_qrcode_ticket_empty() {
    let ticket: WxMpQrCodeTicket = serde_json::from_str("{}").unwrap();
    assert_eq!(ticket.ticket, "");
    assert_eq!(ticket.expire_seconds, -1);
}

#[test]
fn test_kefu_message_empty() {
    let msg: WxMpKefuMessage = serde_json::from_str("{}").unwrap();
    assert_eq!(msg.to_user, None);
    assert_eq!(msg.msg_type, None);
}

#[test]
fn test_kf_list_empty() {
    let list: WxMpKfList = serde_json::from_str(r#"{"kf_list":[]}"#).unwrap();
    assert_eq!(list.kf_list.len(), 0);
}
