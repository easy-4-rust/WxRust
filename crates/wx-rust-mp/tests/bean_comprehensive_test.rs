//! wx-rust-mp Bean 综合测试（SOURCE_PARITY + VALUE_ADD）。

use wx_rust_mp::bean::draft::*;
use wx_rust_mp::bean::freepublish::*;
use wx_rust_mp::bean::kefu::result::*;
use wx_rust_mp::bean::material::*;
use wx_rust_mp::bean::menu::*;
use wx_rust_mp::bean::result::*;
use wx_rust_mp::bean::subscribe::WxMpSubscribeMessage;
use wx_rust_mp::bean::tag::*;
use wx_rust_mp::bean::template::*;

// ═══ Result Beans ═══

#[test]
fn test_wx_mp_user_serde() {
    let json = r#"{"subscribe":true,"openid":"ox123","nickname":"test","sex":1,"language":"zh_CN","city":"深圳","province":"广东","country":"中国","headimgurl":"http://img.example.com","subscribe_time":1700000000,"unionid":"union1","remark":"","groupid":100,"tagid_list":[101,102]}"#;
    let user: WxMpUser = serde_json::from_str(json).unwrap();
    assert_eq!(user.open_id, "ox123");
    assert_eq!(user.nickname, "test");
    assert_eq!(user.subscribe, Some(true));
}

#[test]
fn test_wx_mp_user_from_json() {
    let json = r#"{"subscribe":true,"openid":"ox456","nickname":"user2","sex":2}"#;
    let user = WxMpUser::from_json(json).unwrap();
    assert_eq!(user.open_id, "ox456");
}

#[test]
fn test_wx_mp_qr_code_ticket_serde() {
    let json =
        r#"{"ticket":"TICKET-001","expire_seconds":600,"url":"https://weixin.qq.com/q/abc"}"#;
    let ticket: WxMpQrCodeTicket = serde_json::from_str(json).unwrap();
    assert_eq!(ticket.ticket, "TICKET-001");
    assert_eq!(ticket.expire_seconds, 600);
}

#[test]
fn test_wx_mp_change_openid_serde() {
    let json = r#"{"oriOpenid":"old-ox","newOpenid":"new-ox","errMsg":"ok"}"#;
    let change: WxMpChangeOpenid = serde_json::from_str(json).unwrap();
    assert_eq!(change.ori_openid, "old-ox");
    assert_eq!(change.new_openid, "new-ox");
}

#[test]
fn test_wx_mp_mass_send_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"success","msg_id":"123456","msg_data_id":"789"}"#;
    let result: WxMpMassSendResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.error_code, "0");
    assert_eq!(result.msg_id, "123456");
}

#[test]
fn test_wx_mp_mass_upload_result_serde() {
    let json = r#"{"type":"image","media_id":"media-001","created_at":1700000000}"#;
    let result: WxMpMassUploadResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.r#type, "image");
    assert_eq!(result.media_id, "media-001");
}

// ═══ Tag Beans ═══

#[test]
fn test_wx_user_tag_serde() {
    let json = r#"{"id":101,"name":"VIP用户","count":50}"#;
    let tag: WxUserTag = serde_json::from_str(json).unwrap();
    assert_eq!(tag.id, 101);
    assert_eq!(tag.name, "VIP用户");
    assert_eq!(tag.count, 50);
}

#[test]
fn test_wx_user_tag_from_json() {
    let json = r#"{"tags":[{"id":1,"name":"tag1","count":10},{"id":2,"name":"tag2","count":20}]}"#;
    let tags = WxUserTag::list_from_json(json).unwrap();
    assert_eq!(tags.len(), 2);
}

#[test]
fn test_wx_tag_list_user_serde() {
    let json = r#"{"count":100,"data":{"openid":["ox1","ox2","ox3"]},"next_openid":"ox3"}"#;
    let result: WxTagListUser = serde_json::from_str(json).unwrap();
    assert_eq!(result.count, 100);
    assert_eq!(result.data.openid_list.len(), 3);
}

// ═══ Material Beans ═══

#[test]
fn test_wx_mp_material_serde() {
    let json = r#"{"name":"test-video","videoTitle":"My Video","videoIntroduction":"Description"}"#;
    let material: WxMpMaterial = serde_json::from_str(json).unwrap();
    assert_eq!(material.name, "test-video");
    assert_eq!(material.video_title, "My Video");
}

#[test]
fn test_wx_mp_material_skip_file() {
    let material = WxMpMaterial {
        name: "test".to_string(),
        file: Some("/tmp/test.jpg".to_string()),
        ..Default::default()
    };
    let serialized = serde_json::to_string(&material).unwrap();
    assert!(!serialized.contains("file"));
}

#[test]
fn test_wx_mp_news_article_serde() {
    let json = r#"{"title":"Article","thumb_media_id":"thumb-001","author":"Author","digest":"Summary","show_cover_pic":1,"content":"Content","content_source_url":"http://src.example.com","need_open_comment":1,"only_fans_can_comment":0}"#;
    let article: WxMpNewsArticle = serde_json::from_str(json).unwrap();
    assert_eq!(article.title, "Article");
    assert_eq!(article.author, "Author");
}

#[test]
fn test_wx_media_img_upload_result_serde() {
    let json = r#"{"url":"http://cdn.example.com/media.jpg"}"#;
    let result: WxMediaImgUploadResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.url, "http://cdn.example.com/media.jpg");
}

// ═══ Template Beans ═══

#[test]
fn test_wx_mp_template_serde() {
    let json = r#"{"template_id":"tpl-002","title":"Order Notification","primary_industry":"IT科技","deputy_industry":"互联网|电子商务","content":"订单 {{order_id}} 已发货","example":"订单 12345 已发货"}"#;
    let tpl: WxMpTemplate = serde_json::from_str(json).unwrap();
    assert_eq!(tpl.template_id, "tpl-002");
    assert_eq!(tpl.title, "Order Notification");
}

// ═══ Menu Beans ═══

#[test]
fn test_wx_mp_menu_serde() {
    let json = r#"{"menu":{"button":[{"type":"click","name":"今日歌曲","key":"V1001_TODAY_MUSIC"},{"type":"view","name":"搜索","url":"http://www.soso.com/"}]}}"#;
    let menu: WxMpMenu = serde_json::from_str(json).unwrap();
    assert!(menu.menu.is_some());
    let inner = menu.menu.unwrap();
    assert_eq!(inner.buttons.len(), 2);
    assert_eq!(inner.buttons[0].name, "今日歌曲");
}

#[test]
fn test_wx_mp_menu_from_json() {
    let json = r#"{"menu":{"button":[{"type":"click","name":"test","key":"KEY"}]}}"#;
    let menu = WxMpMenu::from_json(json).unwrap();
    assert!(menu.menu.is_some());
}

// ═══ Draft Beans ═══

#[test]
fn test_wx_mp_draft_list_serde() {
    let json = r#"{"total_count":5,"item_count":2,"item":[{"media_id":"d1"},{"media_id":"d2"}]}"#;
    let list: WxMpDraftList = serde_json::from_str(json).unwrap();
    assert_eq!(list.total_count, 5);
    assert_eq!(list.item_count, 2);
    assert_eq!(list.items.len(), 2);
}

#[test]
fn test_wx_mp_draft_info_serde() {
    let json = r#"{"news_item":[{"title":"Draft Article","author":"Author","digest":"Digest","content":"Content","thumb_media_id":"thumb-001"}]}"#;
    let info: WxMpDraftInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.news_item.len(), 1);
    assert_eq!(info.news_item[0].title, "Draft Article");
}

// ═══ FreePublish Beans ═══

#[test]
fn test_wx_mp_free_publish_list_serde() {
    let json = r#"{"total_count":3,"item_count":1,"item":[{"article_id":"art-001"}]}"#;
    let list: WxMpFreePublishList = serde_json::from_str(json).unwrap();
    assert_eq!(list.total_count, 3);
    assert_eq!(list.items.len(), 1);
}

#[test]
fn test_wx_mp_free_publish_info_serde() {
    let json = r#"{"news_item":[{"title":"Published Article","author":"Author"}]}"#;
    let info: WxMpFreePublishInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.news_item.len(), 1);
}

// ═══ Kefu Beans ═══

#[test]
fn test_wx_mp_kf_list_serde() {
    let json = r#"{"kf_list":[{"kf_account":"test1@test","kf_nickname":"客服1","kf_id":"1001"}]}"#;
    let list: WxMpKfList = serde_json::from_str(json).unwrap();
    assert_eq!(list.kf_list.len(), 1);
    assert_eq!(list.kf_list[0].account, "test1@test");
}

#[test]
fn test_wx_mp_kf_online_list_serde() {
    let json = r#"{"kf_online_list":[{"kf_account":"test1@test","status":1,"kf_id":"1001","accepted_case":5}]}"#;
    let list: WxMpKfOnlineList = serde_json::from_str(json).unwrap();
    assert_eq!(list.kf_online_list.len(), 1);
}

// ═══ Subscribe Beans ═══

#[test]
fn test_wx_mp_subscribe_message_serde() {
    let json = r#"{"touser":"ox123","scene":"SCAN","title":"Welcome","content":"Hello!","url":"http://example.com"}"#;
    let msg: WxMpSubscribeMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.to_user, Some("ox123".to_string()));
    assert_eq!(msg.title, Some("Welcome".to_string()));
}

// ═══ VALUE_ADD ═══

#[test]
fn test_user_default_values() {
    let user: WxMpUser = serde_json::from_str("{}").unwrap();
    assert_eq!(user.open_id, "");
}

#[test]
fn test_material_default() {
    let material = WxMpMaterial::default();
    assert_eq!(material.name, "");
    assert_eq!(material.file, None);
}

#[test]
fn test_menu_empty() {
    let json = r#"{"menu":{"button":[]}}"#;
    let menu: WxMpMenu = serde_json::from_str(json).unwrap();
    assert_eq!(menu.menu.unwrap().buttons.len(), 0);
}

#[test]
fn test_draft_list_from_json() {
    let json = r#"{"total_count":0,"item_count":0}"#;
    let list = WxMpDraftList::from_json(json).unwrap();
    assert_eq!(list.total_count, 0);
}
