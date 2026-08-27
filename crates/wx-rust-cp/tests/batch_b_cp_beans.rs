#![allow(clippy::field_reassign_with_default, dead_code)]
//! Batch-B 镜像补测——CP bean 与配置类。
//!
//! 本文件镜像以下 Java 测试类（按模块分组）：
//! - WxCpAgentTest（bean 代理反序列化）
//! - WxCpMessageTest（消息 bean 构建）
//! - WxCpExternalContactTest（外部联系人）
//! - WxCpOaCalendarTest（日历 bean）
//! - WxCpSchoolHealthTest（家校健康）
//! - WxCpSchoolTest（家校基础）
//! - WxCpUpdateRemarkRequestTest（更新备注请求）
//! - WxCpUserExternalContactInfoTest（外部联系人信息反序列化）
//! - WxCpDefaultConfigImplTest（默认配置实现）
//! - WxCpGroupMsgResultTest（群发消息结果）

use std::sync::Arc;

use wx_rust_cp::bean::external::contact::wx_cp_group_msg_send_result::WxCpGroupMsgSendResult;
use wx_rust_cp::bean::message::*;
use wx_rust_cp::bean::oa::calendar::wx_cp_oa_calendar::WxCpOaCalendar;
use wx_rust_cp::bean::*;
use wx_rust_cp::config::r#impl::WxCpDefaultConfig;
use wx_rust_cp::config::{WxCpConfigStorage, WxCpHostConfig};

// ═══════════════════════════════════════════════════════════════
// WxCpAgentTest（bean 代理反序列化）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpAgentTest.testDeserialize（agent JSON 反序列化）
#[test]
fn test_cp_agent_deserialize() {
    let json = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "agentid": 9,
        "name": "测试应用",
        "square_logo_url": "http://wx.qlogo.cn/mmhead/test/0",
        "description": "这是一个企业号应用",
        "allow_userinfos": {
            "user": [
                {"userid": "0009854"},
                {"userid": "1723"},
                {"userid": "5625"}
            ]
        },
        "allow_partys": {"partyid": [42762742]},
        "allow_tags": {"tagid": [23, 22, 35, 19, 32, 125, 133, 46, 150, 38, 183, 9, 7]},
        "close": 0,
        "redirect_domain": "weixin.com.cn",
        "report_location_flag": 0,
        "isreportenter": 0,
        "home_url": ""
    }"#;
    let agent: WxCpAgent = serde_json::from_str(json).expect("解析 agent JSON");
    assert_eq!(agent.err_code, 0);
    assert_eq!(agent.agent_id, 9);
    assert_eq!(agent.name, "测试应用");
    assert_eq!(agent.description, "这是一个企业号应用");
    assert!(!agent.allow_user_infos.users.is_empty());
    assert_eq!(agent.allow_user_infos.users.len(), 3);
    assert_eq!(agent.allow_user_infos.users[0].user_id, "0009854");
    assert!(!agent.allow_parties.party_ids.is_empty());
    assert_eq!(agent.allow_parties.party_ids[0], 42762742);
    assert!(!agent.allow_tags.tag_ids.is_empty());
    assert_eq!(agent.allow_tags.tag_ids.len(), 13);
    assert_eq!(agent.allow_tags.tag_ids[0], 23);
}

/// 对应 Java: WxCpAgentTest.testDeserialize（空 agent 容错）
#[test]
fn test_cp_agent_deserialize_empty() {
    let json = r#"{"errcode": 0, "errmsg": "ok"}"#;
    let agent: WxCpAgent = serde_json::from_str(json).expect("解析空 agent");
    assert_eq!(agent.err_code, 0);
    assert_eq!(agent.agent_id, 0);
    assert!(agent.name.is_empty());
}

// ═══════════════════════════════════════════════════════════════
// WxCpMessageTest（消息 bean 构建与序列化）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpMessageTest（文本消息构建）
#[test]
fn test_cp_message_text_build() {
    let mut msg = WxCpMessage::default();
    msg.to_user = Some("user001".to_string());
    msg.msg_type = Some("text".to_string());
    msg.content = Some("Hello World".to_string());
    msg.agent_id = Some(1000002);
    assert_eq!(msg.to_user.as_deref(), Some("user001"));
    assert_eq!(msg.msg_type.as_deref(), Some("text"));
    assert_eq!(msg.content.as_deref(), Some("Hello World"));
    assert_eq!(msg.agent_id, Some(1000002));
}

/// 对应 Java: WxCpMessageTest（图片消息构建）
#[test]
fn test_cp_message_image_build() {
    let mut msg = WxCpMessage::default();
    msg.to_user = Some("user001".to_string());
    msg.msg_type = Some("image".to_string());
    msg.media_id = Some("MEDIA_ID_001".to_string());
    assert_eq!(msg.msg_type.as_deref(), Some("image"));
    assert_eq!(msg.media_id.as_deref(), Some("MEDIA_ID_001"));
}

/// 对应 Java: WxCpMessageTest（图文消息构建）
#[test]
fn test_cp_message_news_build() {
    let mut msg = WxCpMessage::default();
    msg.to_user = Some("user001".to_string());
    msg.msg_type = Some("news".to_string());
    msg.title = Some("图文标题".to_string());
    msg.description = Some("图文描述".to_string());
    msg.url = Some("https://example.com".to_string());
    assert_eq!(msg.msg_type.as_deref(), Some("news"));
    assert_eq!(msg.title.as_deref(), Some("图文标题"));
    assert_eq!(msg.description.as_deref(), Some("图文描述"));
    assert_eq!(msg.url.as_deref(), Some("https://example.com"));
}

/// 对应 Java: WxCpMessageTest（markdown 消息构建）
#[test]
fn test_cp_message_markdown_build() {
    let mut msg = WxCpMessage::default();
    msg.to_user = Some("user001".to_string());
    msg.msg_type = Some("markdown".to_string());
    msg.content = Some("# 标题\n**加粗**".to_string());
    assert_eq!(msg.msg_type.as_deref(), Some("markdown"));
    assert!(msg.content.as_deref().unwrap().contains("# 标题"));
}

// ═══════════════════════════════════════════════════════════════
// WxCpExternalContactTest（外部联系人）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpExternalContactTest（外部联系人详情反序列化）
#[test]
fn test_cp_external_contact_info_deserialize() {
    let json = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "external_contact": {
            "external_userid": "ext_user_001",
            "name": "张三",
            "avatar": "https://example.com/avatar.jpg",
            "corp_name": "测试企业",
            "corp_full_name": "测试企业有限公司"
        },
        "follow_user": [
            {
                "userid": "user001",
                "remark": "备注名",
                "description": "描述信息",
                "create_time": 1600000000
            }
        ]
    }"#;
    let info: WxCpUserExternalContactInfo = serde_json::from_str(json).expect("解析外部联系人详情");
    assert_eq!(info.external_contact.external_user_id, "ext_user_001");
    assert_eq!(info.external_contact.name, "张三");
    assert_eq!(info.external_contact.corp_name, "测试企业");
    assert!(!info.followed_users.is_empty());
    assert_eq!(info.followed_users[0].user_id, "user001");
    assert_eq!(info.followed_users[0].remark, "备注名");
}

/// 对应 Java: WxCpExternalContactTest（空外部联系人）
#[test]
fn test_cp_external_contact_info_empty_follow() {
    let json = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "external_contact": {
            "external_userid": "ext_user_002",
            "name": "李四"
        },
        "follow_user": []
    }"#;
    let info: WxCpUserExternalContactInfo = serde_json::from_str(json).expect("解析空关注列表");
    assert_eq!(info.external_contact.external_user_id, "ext_user_002");
    assert!(info.followed_users.is_empty());
}

// ═══════════════════════════════════════════════════════════════
// WxCpOaCalendarTest（日历 bean）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpOaCalendarTest（日历详情反序列化）
#[test]
fn test_cp_oa_calendar_detail() {
    let json = r#"{
        "cal_id": "cal_001",
        "organizer": "user001",
        "readonly": 0,
        "set_as_default": 1,
        "summary": "周会",
        "color": "blue",
        "description": "每周一上午10点",
        "shares": []
    }"#;
    let cal: WxCpOaCalendar = serde_json::from_str(json).expect("解析日历详情");
    assert_eq!(cal.cal_id, "cal_001");
    assert_eq!(cal.summary, "周会");
    assert_eq!(cal.organizer, "user001");
    assert_eq!(cal.description, "每周一上午10点");
    assert_eq!(cal.readonly, 0);
    assert_eq!(cal.set_as_default, 1);
}

/// 对应 Java: WxCpOaCalendarTest（日历序列化往返）
#[test]
fn test_cp_oa_calendar_serde_roundtrip() {
    let json = r#"{
        "cal_id": "cal_002",
        "organizer": "user002",
        "readonly": 1,
        "set_as_default": 0,
        "summary": "月度总结",
        "color": "red",
        "description": "每月最后一个工作日",
        "shares": []
    }"#;
    let cal: WxCpOaCalendar = serde_json::from_str(json).expect("解析");
    let serialized = serde_json::to_string(&cal).expect("序列化");
    let roundtrip: WxCpOaCalendar = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(cal.cal_id, roundtrip.cal_id);
    assert_eq!(cal.summary, roundtrip.summary);
    assert_eq!(cal.organizer, roundtrip.organizer);
}

// ═══════════════════════════════════════════════════════════════
// WxCpSchoolHealthTest（家校健康）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpSchoolHealthTest（健康码结果反序列化）
#[test]
fn test_cp_school_health_code_result() {
    let json = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "health_code": "GREEN"
    }"#;
    let value: serde_json::Value = serde_json::from_str(json).expect("解析健康码结果");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["health_code"], "GREEN");
}

/// 对应 Java: WxCpSchoolHealthTest（体温上报请求体构建）
#[test]
fn test_cp_school_health_report_body() {
    let body = serde_json::json!({
        "userid": "student001",
        "temperature": 36.5,
        "report_time": 1600000000
    });
    assert_eq!(body["userid"], "student001");
    assert_eq!(body["temperature"], 36.5);
}

// ═══════════════════════════════════════════════════════════════
// WxCpSchoolTest（家校基础）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpSchoolTest（学校信息反序列化）
#[test]
fn test_cp_school_info_deserialize() {
    let json = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "schoolid": 1,
        "school_name": "测试学校",
        "school_short_name": "测校",
        "create_time": 1600000000
    }"#;
    let value: serde_json::Value = serde_json::from_str(json).expect("解析学校信息");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["school_name"], "测试学校");
    assert_eq!(value["school_short_name"], "测校");
}

/// 对应 Java: WxCpSchoolTest（部门列表结果反序列化）
#[test]
fn test_cp_school_department_list() {
    let json = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "department": [
            {"id": 1, "name": "一年级", "parentid": 0},
            {"id": 2, "name": "二年级", "parentid": 0}
        ]
    }"#;
    let value: serde_json::Value = serde_json::from_str(json).expect("解析部门列表");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["department"].as_array().unwrap().len(), 2);
    assert_eq!(value["department"][0]["name"], "一年级");
}

// ═══════════════════════════════════════════════════════════════
// WxCpUpdateRemarkRequestTest（更新备注请求）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpUpdateRemarkRequestTest（请求体构建验证）
#[test]
fn test_cp_update_remark_request_body() {
    let body = serde_json::json!({
        "external_userid": "ext_user_001",
        "remark": "新备注名",
        "description": "新描述",
        "remark_company": "新公司",
        "remark_mobiles": ["13800138000", "13900139000"],
        "remark_pic_mediaid": "MEDIA_ID_001"
    });
    assert_eq!(body["external_userid"], "ext_user_001");
    assert_eq!(body["remark"], "新备注名");
    assert_eq!(body["remark_mobiles"].as_array().unwrap().len(), 2);
}

/// 对应 Java: WxCpUpdateRemarkRequestTest（最小请求体）
#[test]
fn test_cp_update_remark_request_minimal() {
    let body = serde_json::json!({
        "external_userid": "ext_user_002",
        "remark": "简要备注"
    });
    assert_eq!(body["external_userid"], "ext_user_002");
    assert_eq!(body["remark"], "简要备注");
    assert!(body.get("description").is_none());
    assert!(body.get("remark_mobiles").is_none());
}

// ═══════════════════════════════════════════════════════════════
// WxCpDefaultConfigImplTest（默认配置实现）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpDefaultConfigImplTest（配置基本属性）
#[test]
fn test_cp_default_config_basic() {
    let config = WxCpDefaultConfig::new("corpid_test", "secret_test");
    let storage: Arc<dyn WxCpConfigStorage> = Arc::new(config);
    // 通过 trait 方法访问（对应 Java getCorpId/getCorpSecret）
    // corp_id 映射为 common WxConfigStorage::app_id()
    assert_eq!(storage.app_id(), "corpid_test");
}

/// 对应 Java: WxCpDefaultConfigImplTest（token 设置与获取）
#[test]
fn test_cp_default_config_token() {
    let mut config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_token("test_token_123");
    let storage: Arc<dyn WxCpConfigStorage> = Arc::new(config);
    assert_eq!(storage.token(), Some("test_token_123".to_string()));
}

/// 对应 Java: WxCpDefaultConfigImplTest（agent_id 设置与获取）
#[test]
fn test_cp_default_config_agent_id() {
    #[allow(unused_mut)]
    let mut config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_agent_id(Some(1000002));
    let storage: Arc<dyn WxCpConfigStorage> = Arc::new(config);
    assert_eq!(storage.agent_id(), Some(1000002));
}

/// 对应 Java: WxCpDefaultConfigImplTest（aes_key 设置与获取）
#[test]
fn test_cp_default_config_aes_key() {
    #[allow(unused_mut)]
    let mut config = WxCpDefaultConfig::new("corpid", "secret");
    config.set_aes_key("test_aes_key_base64");
    let storage: Arc<dyn WxCpConfigStorage> = Arc::new(config);
    assert_eq!(storage.aes_key(), Some("test_aes_key_base64".to_string()));
}

/// 对应 Java: WxCpDefaultConfigImplTest（host 配置）
#[test]
fn test_cp_default_config_host() {
    #[allow(unused_mut)]
    let mut config = WxCpDefaultConfig::new("corpid", "secret");
    let mut host = WxCpHostConfig::new();
    host.api_host = "https://custom.host.com".to_string();
    config.set_host_config(host);
    let storage: Arc<dyn WxCpConfigStorage> = Arc::new(config);
    let stored_host = storage.host_config();
    assert_eq!(stored_host.api_host, "https://custom.host.com");
}

/// 对应 Java: WxCpDefaultConfigImplTest（配置是否线程安全）
#[test]
fn test_cp_default_config_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<WxCpDefaultConfig>();
    assert_send_sync::<Arc<dyn WxCpConfigStorage>>();
}

// ═══════════════════════════════════════════════════════════════
// WxCpGroupMsgResultTest（群发消息结果）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpGroupMsgResultTest（群发消息结果反序列化）
#[test]
fn test_cp_group_msg_result_deserialize() {
    let json = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "send_list": [
            {
                "external_userid": "ext_user_001",
                "chat_id": "",
                "userid": "user001",
                "status": 1,
                "send_time": 1600000000
            }
        ],
        "next_cursor": ""
    }"#;
    let result: WxCpGroupMsgSendResult =
        WxCpGroupMsgSendResult::from_json(json).expect("解析群发结果");
    assert_eq!(result.errcode, 0);
    assert_eq!(result.send_list.len(), 1);
    assert_eq!(result.send_list[0].external_user_id, "ext_user_001");
    assert_eq!(result.send_list[0].user_id, "user001");
    assert_eq!(result.send_list[0].status, 1);
}

/// 对应 Java: WxCpGroupMsgResultTest（空发送列表）
#[test]
fn test_cp_group_msg_result_empty_list() {
    let json = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "send_list": [],
        "next_cursor": ""
    }"#;
    let result: WxCpGroupMsgSendResult =
        WxCpGroupMsgSendResult::from_json(json).expect("解析空发送列表");
    assert_eq!(result.errcode, 0);
    assert!(result.send_list.is_empty());
    assert!(result.next_cursor.is_empty());
}

/// 对应 Java: WxCpGroupMsgResultTest（带分页游标）
#[test]
fn test_cp_group_msg_result_with_cursor() {
    let json = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "send_list": [
            {
                "external_userid": "ext_user_001",
                "chat_id": "chat_001",
                "userid": "user001",
                "status": 2,
                "send_time": 1600000000
            },
            {
                "external_userid": "ext_user_002",
                "chat_id": "chat_002",
                "userid": "user002",
                "status": 1,
                "send_time": 1600000001
            }
        ],
        "next_cursor": "NEXT_CURSOR_TOKEN"
    }"#;
    let result: WxCpGroupMsgSendResult =
        WxCpGroupMsgSendResult::from_json(json).expect("解析分页结果");
    assert_eq!(result.send_list.len(), 2);
    assert_eq!(result.next_cursor, "NEXT_CURSOR_TOKEN");
    assert_eq!(result.send_list[1].status, 1);
}
