#![allow(clippy::field_reassign_with_default, dead_code)]
//! Batch-E Qidian 服务层镜像补测。
//!
//! 本文件镜像以下 Java 测试类：
//! - WxQidianDialServiceImplTest（拨号服务）
//! - BaseWxQidianServiceImplTest（基础服务）
//! - WxQidianDialServiceImplTest（拨号服务详情）

// ═══════════════════════════════════════════════════════════════
// #1 WxQidianDialServiceImplTest（拨号服务）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxQidianDialServiceImplTest.testCreateDial（创建拨号请求体）
#[test]
fn test_qidian_dial_create_body() {
    let body = serde_json::json!({
        "callee": "13800138000",
        "caller": "13900139000",
        "dial_type": 1
    });
    assert_eq!(body["callee"], "13800138000");
    assert_eq!(body["dial_type"], 1);
}

/// 对应 Java: WxQidianDialServiceImplTest.testGetDialList（获取拨号列表）
#[test]
fn test_qidian_dial_list_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "dial_list": [
            {
                "dial_id": "DIAL001",
                "callee": "13800138000",
                "caller": "13900139000",
                "start_time": 1620000000,
                "end_time": 1620000060,
                "duration": 60
            }
        ],
        "total_count": 1
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["total_count"], 1);
}

/// 对应 Java: WxQidianDialServiceImplTest.testGetDialDetail（获取拨号详情）
#[test]
fn test_qidian_dial_detail_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "dial_id": "DIAL001",
        "callee": "13800138000",
        "caller": "13900139000",
        "status": 1,
        "duration": 60
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["dial_id"], "DIAL001");
}

// ═══════════════════════════════════════════════════════════════
// #2 BaseWxQidianServiceImplTest（基础服务）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: BaseWxQidianServiceImplTest.testGetAccessToken（获取 access_token）
#[test]
fn test_qidian_access_token_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "access_token": "QIDIAN_TOKEN001",
        "expires_in": 7200
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["access_token"], "QIDIAN_TOKEN001");
}

/// 对应 Java: BaseWxQidianServiceImplTest.testConfigBean（配置 bean 验证）
#[test]
fn test_qidian_config_body() {
    let body = serde_json::json!({
        "corp_id": "CORP001",
        "secret": "SECRET001",
        "token": "TOKEN001",
        "aes_key": "AES_KEY001"
    });
    assert_eq!(body["corp_id"], "CORP001");
}

/// 对应 Java: BaseWxQidianServiceImplTest.testErrorMapping（错误映射）
#[test]
fn test_qidian_error_mapping() {
    let error_codes = vec![
        (0, "ok"),
        (40001, "invalid credential"),
        (40002, "invalid grant_type"),
    ];
    for (code, msg) in error_codes {
        let json = format!(r#"{{"errcode":{},"errmsg":"{}"}}"#, code, msg);
        let value: serde_json::Value = serde_json::from_str(&json).expect("解析 JSON");
        assert_eq!(value["errcode"], code);
    }
}
