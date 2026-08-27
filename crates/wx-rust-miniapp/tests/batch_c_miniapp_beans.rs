#![allow(clippy::field_reassign_with_default, dead_code)]
//! Batch-C 镜像补测——Miniapp bean 层。
//!
//! 本文件镜像以下 Java 测试类（按 LOC 倒序）：
//! - WxMaCode2VerifyInfoResultTest（code2 验证信息结果解析）
//! - WxMaAuditMediaUploadResultTest（审核媒体上传结果解析）
//! - WxMaMediaAsyncCheckResultTest（媒体异步检查结果解析）
//! - WxMaApiResponseTest（API 响应解析）

use wx_rust_miniapp::bean::*;

// ═══════════════════════════════════════════════════════════════
// #1 WxMaCode2VerifyInfoResultTest —— code2 验证信息结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaCode2VerifyInfoResultTest（code2 验证信息 JSON 解析）
#[test]
fn test_code2_verify_info_result_from_json() {
    let json_str = r#"{
        "session_key": "session_key_abc123",
        "openid": "openid_001",
        "unionid": "unionid_001",
        "is_limit": false
    }"#;
    let result = WxMaCode2VerifyInfoResult::from_json(json_str).expect("解析 code2 验证信息");
    assert_eq!(result.session_key, "session_key_abc123");
    assert_eq!(result.openid, "openid_001");
    assert_eq!(result.unionid, "unionid_001");
    assert!(!result.is_limit);
}

/// 对应 Java: WxMaCode2VerifyInfoResultTest（code2 验证信息受限用户）
#[test]
fn test_code2_verify_info_result_is_limit() {
    let json_str = r#"{
        "session_key": "session_key_def456",
        "openid": "openid_002",
        "unionid": "",
        "is_limit": true
    }"#;
    let result = WxMaCode2VerifyInfoResult::from_json(json_str).expect("解析受限用户");
    assert_eq!(result.session_key, "session_key_def456");
    assert_eq!(result.openid, "openid_002");
    assert_eq!(result.unionid, "");
    assert!(result.is_limit);
}

/// 对应 Java: WxMaCode2VerifyInfoResultTest（code2 验证信息序列化往返验证）
#[test]
fn test_code2_verify_info_result_roundtrip() {
    let json_str = r#"{
        "session_key": "session_key_xyz",
        "openid": "openid_003",
        "unionid": "unionid_003",
        "is_limit": false
    }"#;
    let result = WxMaCode2VerifyInfoResult::from_json(json_str).expect("解析");
    let serialized = serde_json::to_string(&result).expect("序列化");
    let result2: WxMaCode2VerifyInfoResult = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(result, result2);
}

/// 对应 Java: WxMaCode2VerifyInfoResultTest（code2 验证信息默认值验证）
#[test]
fn test_code2_verify_info_result_default() {
    let json_str = r#"{}"#;
    let result = WxMaCode2VerifyInfoResult::from_json(json_str).expect("解析空 JSON");
    assert_eq!(result.session_key, "");
    assert_eq!(result.openid, "");
    assert_eq!(result.unionid, "");
    assert!(!result.is_limit);
}

// ═══════════════════════════════════════════════════════════════
// #2 WxMaAuditMediaUploadResultTest —— 审核媒体上传结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaAuditMediaUploadResultTest（审核媒体上传结果 JSON 解析）
#[test]
fn test_audit_media_upload_result_from_json() {
    let json_str = r#"{
        "type": "image",
        "mediaid": "media_id_001"
    }"#;
    let result = WxMaAuditMediaUploadResult::from_json(json_str).expect("解析审核媒体上传结果");
    assert_eq!(result.r#type, "image");
    assert_eq!(result.media_id, "media_id_001");
}

/// 对应 Java: WxMaAuditMediaUploadResultTest（审核媒体上传结果视频类型）
#[test]
fn test_audit_media_upload_result_video() {
    let json_str = r#"{
        "type": "video",
        "mediaid": "media_id_002"
    }"#;
    let result = WxMaAuditMediaUploadResult::from_json(json_str).expect("解析视频上传结果");
    assert_eq!(result.r#type, "video");
    assert_eq!(result.media_id, "media_id_002");
}

/// 对应 Java: WxMaAuditMediaUploadResultTest（审核媒体上传结果序列化往返验证）
#[test]
fn test_audit_media_upload_result_roundtrip() {
    let json_str = r#"{
        "type": "image",
        "mediaid": "media_id_003"
    }"#;
    let result = WxMaAuditMediaUploadResult::from_json(json_str).expect("解析");
    let serialized = serde_json::to_string(&result).expect("序列化");
    let result2: WxMaAuditMediaUploadResult = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(result, result2);
}

// ═══════════════════════════════════════════════════════════════
// #3 WxMaMediaAsyncCheckResultTest —— 媒体异步检查结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaMediaAsyncCheckResultTest（媒体异步检查结果 JSON 解析）
#[test]
fn test_media_async_check_result_from_json() {
    let json_str = r#"{
        "trace_id": "trace_001",
        "result": {
            "suggest": "pass",
            "label": "normal"
        },
        "detail": [
            {
                "strategy": "content_security",
                "errcode": 0,
                "suggest": "pass",
                "label": "normal",
                "prob": 90
            }
        ]
    }"#;
    let result = WxMaMediaAsyncCheckResult::from_json(json_str).expect("解析媒体异步检查结果");
    assert_eq!(result.trace_id, "trace_001");
    assert_eq!(result.result.suggest, "pass");
    assert_eq!(result.result.label, "normal");
    assert_eq!(result.detail.len(), 1);
    assert_eq!(result.detail[0].strategy, "content_security");
    assert_eq!(result.detail[0].errcode, 0);
    assert_eq!(result.detail[0].prob, 90);
}

/// 对应 Java: WxMaMediaAsyncCheckResultTest（媒体异步检查结果违规内容）
#[test]
fn test_media_async_check_result_risky() {
    let json_str = r#"{
        "trace_id": "trace_002",
        "result": {
            "suggest": "risky",
            "label": "spam"
        },
        "detail": [
            {
                "strategy": "content_security",
                "errcode": 0,
                "suggest": "risky",
                "label": "spam",
                "prob": 95
            },
            {
                "strategy": "anti_spam",
                "errcode": 0,
                "suggest": "risky",
                "label": "ad",
                "prob": 80
            }
        ]
    }"#;
    let result = WxMaMediaAsyncCheckResult::from_json(json_str).expect("解析违规内容结果");
    assert_eq!(result.result.suggest, "risky");
    assert_eq!(result.result.label, "spam");
    assert_eq!(result.detail.len(), 2);
    assert_eq!(result.detail[1].strategy, "anti_spam");
}

/// 对应 Java: WxMaMediaAsyncCheckResultTest（媒体异步检查结果序列化往返验证）
#[test]
fn test_media_async_check_result_roundtrip() {
    let json_str = r#"{
        "trace_id": "trace_003",
        "result": {
            "suggest": "pass",
            "label": "normal"
        },
        "detail": []
    }"#;
    let result = WxMaMediaAsyncCheckResult::from_json(json_str).expect("解析");
    let serialized = result.to_json().expect("序列化");
    let result2 = WxMaMediaAsyncCheckResult::from_json(&serialized).expect("反序列化");
    assert_eq!(result, result2);
}

/// 对应 Java: WxMaMediaAsyncCheckResultTest（媒体异步检查结果空详情）
#[test]
fn test_media_async_check_result_empty_detail() {
    let json_str = r#"{
        "trace_id": "trace_004",
        "result": {
            "suggest": "pass",
            "label": "normal"
        },
        "detail": []
    }"#;
    let result = WxMaMediaAsyncCheckResult::from_json(json_str).expect("解析空详情");
    assert_eq!(result.trace_id, "trace_004");
    assert!(result.detail.is_empty());
}

// ═══════════════════════════════════════════════════════════════
// #4 WxMaApiResponseTest —— API 响应解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaApiResponseTest（API 响应 JSON 解析）
#[test]
fn test_api_response_from_json() {
    let json_str = r#"{
        "content": "response content",
        "headers": {
            "Content-Type": "application/json",
            "X-Request-Id": "req_001"
        }
    }"#;
    let resp: WxMaApiResponse = serde_json::from_str(json_str).expect("解析 API 响应");
    assert_eq!(resp.content, "response content");
    assert_eq!(resp.headers.len(), 2);
    assert_eq!(
        resp.headers.get("Content-Type").unwrap(),
        "application/json"
    );
    assert_eq!(resp.headers.get("X-Request-Id").unwrap(), "req_001");
}

/// 对应 Java: WxMaApiResponseTest（API 响应空 headers）
#[test]
fn test_api_response_empty_headers() {
    let json_str = r#"{
        "content": "ok",
        "headers": {}
    }"#;
    let resp: WxMaApiResponse = serde_json::from_str(json_str).expect("解析空 headers 响应");
    assert_eq!(resp.content, "ok");
    assert!(resp.headers.is_empty());
}

/// 对应 Java: WxMaApiResponseTest（API 响应序列化往返验证）
#[test]
fn test_api_response_roundtrip() {
    let json_str = r#"{
        "content": "test content",
        "headers": {
            "X-Test": "test_value"
        }
    }"#;
    let resp: WxMaApiResponse = serde_json::from_str(json_str).expect("解析");
    let serialized = serde_json::to_string(&resp).expect("序列化");
    let resp2: WxMaApiResponse = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(resp, resp2);
}
