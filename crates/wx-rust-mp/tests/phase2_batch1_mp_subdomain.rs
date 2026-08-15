//! Phase 2 Batch 2.1: mp 子域 Service 测试
//!
//! 镜像 Java WxMpDraftServiceImplTest / WxMpFreePublishServiceImplTest /
//! WxMpSubscribeMsgServiceImplTest / WxMpUserTagServiceImplTest /
//! WxMpUserBlacklistServiceImplTest / WxMpBusyRetryTest

use wx_rust_mp::bean::draft::*;
use wx_rust_mp::bean::freepublish::*;
use wx_rust_mp::bean::result::*;
use wx_rust_mp::bean::subscribe::*;
use wx_rust_mp::bean::tag::*;

// ═══ Draft Beans ═══

#[test]
fn test_draft_info_serde() {
    let json = r#"{"news_item":[{"article_type":"news","title":"Draft Article","author":"Author","digest":"Digest","content":"Content","content_source_url":"http://src.example.com"}]}"#;
    let info: WxMpDraftInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.news_item.len(), 1);
    assert_eq!(info.news_item[0].title, "Draft Article");
    assert_eq!(info.news_item[0].author, "Author");
}

#[test]
fn test_draft_list_serde() {
    let json = r#"{"total_count":5,"item_count":2,"item":[{"media_id":"d1"},{"media_id":"d2"}]}"#;
    let list: WxMpDraftList = serde_json::from_str(json).unwrap();
    assert_eq!(list.total_count, 5);
    assert_eq!(list.item_count, 2);
    assert_eq!(list.items.len(), 2);
}

#[test]
fn test_draft_list_from_json() {
    let json = r#"{"total_count":0,"item_count":0}"#;
    let list = WxMpDraftList::from_json(json).unwrap();
    assert_eq!(list.total_count, 0);
}

#[test]
fn test_add_draft_serde() {
    let json = r#"{"articles":[{"article_type":"news","title":"New Draft","author":"Author","digest":"","content":"Body","content_source_url":""}]}"#;
    let draft: WxMpAddDraft = serde_json::from_str(json).unwrap();
    assert_eq!(draft.articles.len(), 1);
    assert_eq!(draft.articles[0].title, "New Draft");
}

#[test]
fn test_update_draft_serde() {
    let json = r#"{"media_id":"draft-001","index":0,"articles":{"article_type":"news","title":"Updated","author":"Author","digest":"","content":"Updated body","content_source_url":""}}"#;
    let update: WxMpUpdateDraft = serde_json::from_str(json).unwrap();
    assert_eq!(update.media_id, "draft-001");
}

#[test]
fn test_draft_item_serde() {
    let json = r#"{"media_id":"item-001","content":{"news_item":[{"title":"Article"}]},"update_time":1700000000}"#;
    let item: WxMpDraftItem = serde_json::from_str(json).unwrap();
    assert_eq!(item.media_id, "item-001");
}

// ═══ FreePublish Beans ═══

#[test]
fn test_free_publish_info_serde() {
    let json = r#"{"news_item":[{"title":"Published Article","author":"Author"}]}"#;
    let info: WxMpFreePublishInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.news_item.len(), 1);
}

#[test]
fn test_free_publish_list_serde() {
    let json = r#"{"total_count":3,"item_count":1,"item":[{"article_id":"art-001"}]}"#;
    let list: WxMpFreePublishList = serde_json::from_str(json).unwrap();
    assert_eq!(list.total_count, 3);
    assert_eq!(list.items.len(), 1);
}

#[test]
fn test_free_publish_status_serde() {
    let json = r#"{"publish_status":0,"publish_id":"pub-001","article_id":"art-001","article_detail":{"news_item":[]}}"#;
    let status: WxMpFreePublishStatus = serde_json::from_str(json).unwrap();
    assert_eq!(status.publish_status, 0);
}

// ═══ Subscribe Message Beans ═══

#[test]
fn test_subscribe_message_serde() {
    let json = r#"{"touser":"ox123","scene":"SCAN","title":"Welcome","content":"Hello!"}"#;
    let msg: WxMpSubscribeMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.to_user, Some("ox123".to_string()));
    assert_eq!(msg.title, Some("Welcome".to_string()));
}

// ═══ User Tag Beans ═══

#[test]
fn test_user_tag_list_from_json() {
    let json =
        r#"{"tags":[{"id":1,"name":"VIP","count":50},{"id":2,"name":"Normal","count":200}]}"#;
    let tags = WxUserTag::list_from_json(json).unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].name, "VIP");
    assert_eq!(tags[1].count, 200);
}

#[test]
fn test_tag_list_user_serde() {
    let json = r#"{"count":100,"data":{"openid":["ox1","ox2"]},"next_openid":"ox2"}"#;
    let result: WxTagListUser = serde_json::from_str(json).unwrap();
    assert_eq!(result.count, 100);
    assert_eq!(result.data.openid_list.len(), 2);
}

// ═══ Mass Send Result ═══

#[test]
fn test_mass_send_result_serde() {
    let json = r#"{"errcode":"0","errmsg":"success","msg_id":"123456","msg_data_id":"789"}"#;
    let result: WxMpMassSendResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.error_code, "0");
    assert_eq!(result.msg_id, "123456");
}

#[test]
fn test_mass_upload_result_serde() {
    let json = r#"{"type":"image","media_id":"media-001","created_at":1700000000}"#;
    let result: WxMpMassUploadResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.r#type, "image");
    assert_eq!(result.media_id, "media-001");
}

// ═══ Template Industry ═══
// ═══ Get Self Menu Info Result ═══
// ═══ VALUE_ADD ═══
#[test]
fn test_draft_list_empty() {
    let json = r#"{"total_count":0,"item_count":0}"#;
    let list: WxMpDraftList = serde_json::from_str(json).unwrap();
    assert_eq!(list.items.len(), 0);
}

#[test]
fn test_free_publish_list_empty() {
    let json = r#"{"total_count":0,"item_count":0}"#;
    let list: WxMpFreePublishList = serde_json::from_str(json).unwrap();
    assert_eq!(list.items.len(), 0);
}

#[test]
fn test_user_tag_empty_list() {
    let json = r#"{"tags":[]}"#;
    let tags = WxUserTag::list_from_json(json).unwrap();
    assert_eq!(tags.len(), 0);
}
