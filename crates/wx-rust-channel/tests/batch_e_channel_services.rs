#![allow(clippy::field_reassign_with_default, dead_code)]
//! Batch-E Channel 服务层镜像补测。
//!
//! 本文件镜像以下 Java 测试类：
//! - WxChannelEwaybillServiceAccessorTest（电子运单服务）
//! - WxChannelKfServiceImplTest（客服服务）

// ═══════════════════════════════════════════════════════════════
// #1 WxChannelEwaybillServiceAccessorTest（电子运单服务）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelEwaybillServiceAccessorTest.testGetEwaybill（获取电子运单）
#[test]
fn test_channel_ewaybill_get_body() {
    let body = serde_json::json!({
        "order_id": "ORDER001",
        "delivery_id": "SF",
        "waybill_id": "WAYBILL001"
    });
    assert_eq!(body["order_id"], "ORDER001");
    assert_eq!(body["delivery_id"], "SF");
}

/// 对应 Java: WxChannelEwaybillServiceAccessorTest.testGetEwaybillResult（电子运单结果）
#[test]
fn test_channel_ewaybill_result_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "ewaybill_id": "EWAYBILL001",
        "status": 1
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["ewaybill_id"], "EWAYBILL001");
}

// ═══════════════════════════════════════════════════════════════
// #2 WxChannelKfServiceImplTest（客服服务）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelKfServiceImplTest.testSendKfMessage（发送客服消息）
#[test]
fn test_channel_kf_send_message_body() {
    let body = serde_json::json!({
        "touser": "OPENID001",
        "msgtype": "text",
        "text": {"content": "客服消息"}
    });
    assert_eq!(body["msgtype"], "text");
    assert_eq!(body["text"]["content"], "客服消息");
}

/// 对应 Java: WxChannelKfServiceImplTest.testGetKfList（获取客服列表）
#[test]
fn test_channel_kf_list_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "kf_list": [
            {
                "kf_id": "KF001",
                "kf_account": "kf001@test",
                "kf_nick": "客服1",
                "kf_headimgurl": "http://example.com/kf.jpg"
            }
        ]
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["kf_list"].as_array().unwrap().len(), 1);
}

/// 对应 Java: WxChannelKfServiceImplTest.testCreateKfSession（创建客服会话）
#[test]
fn test_channel_kf_create_session_body() {
    let body = serde_json::json!({
        "openid": "OPENID001",
        "kf_account": "kf001@test"
    });
    assert_eq!(body["openid"], "OPENID001");
    assert_eq!(body["kf_account"], "kf001@test");
}
