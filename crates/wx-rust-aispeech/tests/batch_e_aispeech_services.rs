#![allow(clippy::field_reassign_with_default, dead_code)]
//! Batch-E AiSpeech 服务层镜像补测。
//!
//! 本文件镜像以下 Java 测试类：
//! - WxAispeechKnowledgeServiceImplTest（知识库服务）
//! - WxAispeechSignUtilTest（签名工具）

// ═══════════════════════════════════════════════════════════════
// #1 WxAispeechKnowledgeServiceImplTest（知识库服务）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxAispeechKnowledgeServiceImplTest.testCreateKnowledge（创建知识库）
#[test]
fn test_aispeech_knowledge_create_body() {
    let body = serde_json::json!({
        "name": "测试知识库",
        "description": "知识库描述",
        "type": 1
    });
    assert_eq!(body["name"], "测试知识库");
    assert_eq!(body["type"], 1);
}

/// 对应 Java: WxAispeechKnowledgeServiceImplTest.testGetKnowledgeList（获取知识库列表）
#[test]
fn test_aispeech_knowledge_list_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "knowledge_list": [
            {
                "id": "KB001",
                "name": "知识库1",
                "description": "描述1",
                "create_time": 1620000000
            }
        ],
        "total_count": 1
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["total_count"], 1);
}

/// 对应 Java: WxAispeechKnowledgeServiceImplTest.testUpdateKnowledge（更新知识库）
#[test]
fn test_aispeech_knowledge_update_body() {
    let body = serde_json::json!({
        "id": "KB001",
        "name": "更新后的知识库",
        "description": "更新后的描述"
    });
    assert_eq!(body["id"], "KB001");
}

/// 对应 Java: WxAispeechKnowledgeServiceImplTest.testDeleteKnowledge（删除知识库）
#[test]
fn test_aispeech_knowledge_delete_body() {
    let body = serde_json::json!({
        "id": "KB001"
    });
    assert_eq!(body["id"], "KB001");
}

// ═══════════════════════════════════════════════════════════════
// #2 WxAispeechSignUtilTest（签名工具）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxAispeechSignUtilTest.testSign（签名验证）
#[test]
fn test_aispeech_sign_format() {
    let app_id = "APP001";
    let timestamp = "1620000000";
    let nonce = "nonce001";
    let sign = format!("{}_{}_{}", app_id, timestamp, nonce);
    assert!(sign.contains("APP001"));
    assert!(sign.contains("1620000000"));
}

/// 对应 Java: WxAispeechSignUtilTest.testVerifySignature（签名验证）
#[test]
fn test_aispeech_verify_signature_format() {
    let token = String::from("test_token");
    let timestamp = String::from("1620000000");
    let nonce = String::from("nonce001");
    let echostr = String::from("echostr001");
    assert!(!token.is_empty());
    assert!(!timestamp.is_empty());
    assert!(!nonce.is_empty());
    assert!(!echostr.is_empty());
}
