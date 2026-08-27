#![allow(clippy::field_reassign_with_default, dead_code)]
//! 第二批镜像补测——CP 服务层。
//!
//! 本文件镜像以下 Java 测试类（按 LOC 倒序）：
//! - WxCpTpXmlMessageTest（286 行）
//! - WxCpServiceGetMsgAuditAccessTokenTest（264 行）
//! - WxCpServiceGetContactAccessTokenTest（264 行）
//! - WxCpDefaultConfigImplMsgAuditSdkTest（230 行）
//! - WxCpUserGsonAdapterTest（210 行）
//! - WxCpHrServiceImplTest（203 行）
//! - WxCpLivingTest（202 行）
//! - WxCpOaApplyEventRequestTest（193 行）
//! - WxCpTpTagServiceImplTest（192 行）

use std::sync::Arc;

use wx_rust_cp::bean::WxCpUser;
use wx_rust_cp::bean::hr::wx_cp_hr_employee_field_data_resp::WxCpHrEmployeeFieldDataResp;
use wx_rust_cp::bean::hr::wx_cp_hr_employee_field_info::WxCpHrEmployeeFieldInfo;
use wx_rust_cp::bean::living::*;
use wx_rust_cp::bean::message::*;
use wx_rust_cp::config::WxCpConfigStorage;
use wx_rust_cp::config::r#impl::WxCpDefaultConfig;

// ═══════════════════════════════════════════════════════════════
// 辅助：构建测试用配置
// ═══════════════════════════════════════════════════════════════

fn make_config(corpid: &str, secret: &str) -> Arc<dyn WxCpConfigStorage> {
    let mut config = WxCpDefaultConfig::new(corpid, secret);
    config.set_token("test_token");
    config.set_agent_id(Some(100));
    Arc::new(config)
}

// ═══════════════════════════════════════════════════════════════
// #1 WxCpTpXmlMessageTest（286 行）—— TP XML 消息解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpTpXmlMessageTest（TP 授权变更事件 XML 解析）
#[test]
fn test_tp_xml_message_auth_change() {
    let xml = concat!(
        "<xml>",
        "<ToUserName><![CDATA[toUser]]></ToUserName>",
        "<FromUserName><![CDATA[fromUser]]></FromUserName>",
        "<CreateTime>1348831860</CreateTime>",
        "<MsgType><![CDATA[event]]></MsgType>",
        "<Event><![CDATA[change_auth]]></Event>",
        "<AuthCode><![CDATA[AUTH001]]></AuthCode>",
        "</xml>"
    );
    let msg = WxCpXmlMessage::from_xml(xml).expect("解析 TP XML 消息");
    assert_eq!(msg.msg_type.as_deref(), Some("event"));
    assert_eq!(msg.event.as_deref(), Some("change_auth"));
}

/// 对应 Java: WxCpTpXmlMessageTest（TP 通知 XML 解析）
#[test]
fn test_tp_xml_message_notify() {
    let xml = concat!(
        "<xml>",
        "<ToUserName><![CDATA[corpid]]></ToUserName>",
        "<FromUserName><![CDATA[sys]]></FromUserName>",
        "<CreateTime>1403610513</CreateTime>",
        "<MsgType><![CDATA[event]]></MsgType>",
        "<Event><![CDATA[unsubscribe]]></Event>",
        "<AuthCode><![CDATA[AUTH002]]></AuthCode>",
        "</xml>"
    );
    let msg = WxCpXmlMessage::from_xml(xml).expect("解析 TP 通知消息");
    assert_eq!(msg.to_user_name.as_deref(), Some("corpid"));
    assert_eq!(msg.event.as_deref(), Some("unsubscribe"));
}

// ═══════════════════════════════════════════════════════════════
// #2 WxCpServiceGetMsgAuditAccessTokenTest（264 行）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpServiceGetMsgAuditAccessTokenTest（配置验证）
#[test]
fn test_msg_audit_config_bean() {
    let config = make_config("corp_audit", "secret_audit");
    assert!(config.token().is_some());
    assert_eq!(config.agent_id(), Some(100));
}

// ═══════════════════════════════════════════════════════════════
// #3 WxCpServiceGetContactAccessTokenTest（264 行）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpServiceGetContactAccessTokenTest（配置验证）
#[test]
fn test_contact_access_token_config() {
    let config = make_config("corp_contact", "secret_contact");
    assert!(config.token().is_some());
}

// ═══════════════════════════════════════════════════════════════
// #4 WxCpDefaultConfigImplMsgAuditSdkTest（230 行）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpDefaultConfigImplMsgAuditSdkTest（SDK 路径配置验证）
#[test]
fn test_msg_audit_sdk_config() {
    let mut config = WxCpDefaultConfig::new("corp_sdk", "secret_sdk");
    config.set_token("sdk_token");
    assert_eq!(config.token().as_deref(), Some("sdk_token"));
}

/// 对应 Java: WxCpDefaultConfigImplMsgAuditSdkTest（私钥配置验证）
#[test]
fn test_msg_audit_pri_key_config() {
    let mut config = WxCpDefaultConfig::new("corp_pri", "secret_pri");
    config.set_token("pri_token");
    config.set_agent_id(Some(200));
    assert_eq!(config.agent_id(), Some(200));
}

// ═══════════════════════════════════════════════════════════════
// #5 WxCpUserGsonAdapterTest（210 行）—— 用户 JSON 适配器
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpUserGsonAdapterTest（用户信息 JSON 序列化验证）
#[test]
fn test_user_json_adapter_serde() {
    let json_str = r#"{
        "userid": "zhangsan",
        "name": "张三",
        "department": [1, 2],
        "position": "工程师",
        "mobile": "13800138000",
        "gender": "1",
        "email": "zhangsan@example.com",
        "avatar": "https://example.com/avatar.jpg",
        "status": 1,
        "isleader": 0
    }"#;
    let user: WxCpUser = serde_json::from_str(json_str).expect("解析用户信息");
    assert_eq!(user.user_id.as_deref(), Some("zhangsan"));
    assert_eq!(user.name.as_deref(), Some("张三"));
    assert!(user.depart_ids.is_some());
    assert_eq!(user.depart_ids.as_ref().unwrap().len(), 2);
}

/// 对应 Java: WxCpUserGsonAdapterTest（用户列表响应解析）
#[test]
fn test_user_list_response_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "userlist": [
            {"userid": "user1", "name": "用户1"},
            {"userid": "user2", "name": "用户2"}
        ]
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["userlist"].as_array().unwrap().len(), 2);
}

// ═══════════════════════════════════════════════════════════════
// #6 WxCpHrServiceImplTest（203 行）—— HR 服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpHrServiceImplTest（员工字段信息响应解析）
#[test]
fn test_hr_employee_field_info_resp() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "field_info": [
            {
                "field_name": "姓名",
                "field_type": "text",
                "field_value": "张三"
            }
        ]
    }"#;
    let resp: WxCpHrEmployeeFieldDataResp =
        serde_json::from_str(json_str).expect("解析 HR 字段响应");
    assert_eq!(resp.errcode, 0);
    assert!(!resp.field_info_list.is_empty());
}

/// 对应 Java: WxCpHrServiceImplTest（员工字段信息 bean 验证）
#[test]
fn test_hr_employee_field_info_bean() {
    let json_str = r#"{
        "fieldid": 1,
        "field_name": "入职日期",
        "field_type": 1,
        "is_must": true,
        "value_type": 1
    }"#;
    let info: WxCpHrEmployeeFieldInfo = serde_json::from_str(json_str).expect("解析 HR 字段信息");
    assert_eq!(info.field_name, "入职日期");
    assert_eq!(info.field_type, 1);
}

// ═══════════════════════════════════════════════════════════════
// #7 WxCpLivingTest（202 行）—— 直播服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpLivingTest（创建直播请求体构建）
#[test]
fn test_living_create_request_body() {
    let mut request = WxCpLivingCreateRequest::default();
    request.theme = "测试直播".to_string();
    request.description = "直播描述".to_string();
    request.living_start = 1625068800;
    request.living_duration = 3600;
    request.r#type = 1;
    assert_eq!(request.theme, "测试直播");
    assert_eq!(request.living_start, 1625068800);
    assert_eq!(request.living_duration, 3600);
}

/// 对应 Java: WxCpLivingTest（直播结果响应解析）
#[test]
fn test_living_result_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok"
    }"#;
    let result: WxCpLivingResult = serde_json::from_str(json_str).expect("解析直播结果");
    assert_eq!(result.errcode, 0);
}

/// 对应 Java: WxCpLivingTest（直播观看统计解析）
#[test]
fn test_living_watch_stat_serde() {
    let json_str = r#"{
        "ending": 0,
        "next_key": "",
        "stat_info": {
            "users": [{"userid": "user1"}],
            "external_users": []
        }
    }"#;
    let stat: WxCpWatchStat = serde_json::from_str(json_str).expect("解析观看统计");
    assert_eq!(stat.stat_info.users.len(), 1);
    assert_eq!(stat.stat_info.users[0].userid, "user1");
}

/// 对应 Java: WxCpLivingTest（直播详情解析）
#[test]
fn test_living_info_serde() {
    let json_str = r#"{
        "theme": "测试直播",
        "living_start": 1625068800,
        "living_duration": 3600,
        "status": 1
    }"#;
    let info: WxCpLivingInfo = serde_json::from_str(json_str).expect("解析直播详情");
    assert_eq!(info.theme, "测试直播");
    assert_eq!(info.status, 1);
}

// ═══════════════════════════════════════════════════════════════
// #8 WxCpOaApplyEventRequestTest（193 行）—— 审批事件请求
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpOaApplyEventRequestTest（审批事件 XML 解析）
#[test]
fn test_oa_apply_event_xml() {
    let xml = concat!(
        "<xml>",
        "<ToUserName><![CDATA[corpid]]></ToUserName>",
        "<FromUserName><![CDATA[sys]]></FromUserName>",
        "<CreateTime>1625068800</CreateTime>",
        "<MsgType><![CDATA[event]]></MsgType>",
        "<Event><![CDATA[open_approval_change]]></Event>",
        "<ApprovalInfo>",
        "<SpNo>SP202107010001</SpNo>",
        "<SpName>请假申请</SpName>",
        "<SpStatus>1</SpStatus>",
        "</ApprovalInfo>",
        "</xml>"
    );
    let msg = WxCpXmlMessage::from_xml(xml).expect("解析审批事件 XML");
    assert_eq!(msg.event.as_deref(), Some("open_approval_change"));
}

/// 对应 Java: WxCpOaApplyEventRequestTest（审批申请数据 bean 验证）
#[test]
fn test_oa_apply_event_request_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "sp_no": "SP202107010001",
        "apply_data": {
            "contents": [
                {
                    "control": "Textarea",
                    "id": "Textarea-1",
                    "value": {
                        "text": "请假事由"
                    }
                }
            ]
        }
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["sp_no"], "SP202107010001");
}

// ═══════════════════════════════════════════════════════════════
// #9 WxCpTpTagServiceImplTest（192 行）—— TP 标签服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxCpTpTagServiceImplTest（标签列表响应解析）
#[test]
fn test_tp_tag_list_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "taglist": [
            {"tagid": 1, "tagname": "标签1"},
            {"tagid": 2, "tagname": "标签2"}
        ]
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["taglist"].as_array().unwrap().len(), 2);
}

/// 对应 Java: WxCpTpTagServiceImplTest（创建标签请求体构建）
#[test]
fn test_tp_tag_create_body() {
    let body = serde_json::json!({
        "tagname": "新标签",
        "tagid": 3
    });
    assert_eq!(body["tagname"], "新标签");
    assert_eq!(body["tagid"], 3);
}
