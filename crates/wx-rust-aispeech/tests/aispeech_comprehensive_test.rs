//! wx-rust-aispeech 综合测试（SOURCE_PARITY + RUST_OBLIGATION + VALUE_ADD）。

use std::sync::Arc;

use wx_rust_aispeech::api::WxAispeechService;
use wx_rust_aispeech::bean::dialog::*;
use wx_rust_aispeech::bean::knowledge::*;
use wx_rust_aispeech::config::r#impl::WxAispeechDefaultConfig;
use wx_rust_aispeech::config::WxAispeechConfigStorage;

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：配置测试
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_default_config_builder_and_getters() {
    let mut cfg = WxAispeechDefaultConfig::new();
    cfg.set_appid("wx-appid-123")
       .set_token("verify-token")
       .set_aes_key("base64-aes-key")
       .set_secret_key("hmac-secret")
       .set_dialog_api_base_url("https://custom-dialog.example.com")
       .set_knowledge_api_base_url("https://custom-knowledge.example.com")
       .set_http_proxy_host("proxy.example.com")
       .set_http_proxy_port(8080)
       .set_http_proxy_username("user")
       .set_http_proxy_password("pass");

    assert_eq!(cfg.appid(), Some("wx-appid-123"));
    assert_eq!(cfg.token(), Some("verify-token"));
    assert_eq!(cfg.aes_key(), Some("base64-aes-key"));
    assert_eq!(cfg.secret_key(), Some("hmac-secret"));
    assert_eq!(cfg.dialog_api_base_url(), "https://custom-dialog.example.com");
    assert_eq!(cfg.knowledge_api_base_url(), "https://custom-knowledge.example.com");
    assert_eq!(cfg.http_proxy_host(), Some("proxy.example.com"));
    assert_eq!(cfg.http_proxy_port(), 8080);
    assert_eq!(cfg.http_proxy_username(), Some("user"));
    assert_eq!(cfg.http_proxy_password(), Some("pass"));
}

#[test]
fn test_default_config_initial_values() {
    let cfg = WxAispeechDefaultConfig::new();
    assert_eq!(cfg.appid(), None);
    assert_eq!(cfg.token(), None);
    assert_eq!(cfg.aes_key(), None);
    assert_eq!(cfg.secret_key(), None);
    assert_eq!(cfg.open_ai_token(), None);
    assert_eq!(cfg.http_proxy_host(), None);
    assert_eq!(cfg.http_proxy_port(), 0);
    assert_eq!(cfg.dialog_api_base_url(), "https://openaiapi.weixin.qq.com");
    assert_eq!(cfg.knowledge_api_base_url(), "https://weknora.weixin.qq.com");
}

#[test]
fn test_open_ai_token_set_and_get() {
    let cfg = WxAispeechDefaultConfig::new();
    assert_eq!(cfg.open_ai_token(), None);
    cfg.set_open_ai_token("sk-abc123");
    assert_eq!(cfg.open_ai_token(), Some("sk-abc123".to_string()));
    cfg.set_open_ai_token("sk-xyz789");
    assert_eq!(cfg.open_ai_token(), Some("sk-xyz789".to_string()));
}

#[test]
fn test_default_trait() {
    let cfg = WxAispeechDefaultConfig::default();
    assert_eq!(cfg.appid(), None);
}

#[test]
fn test_config_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<WxAispeechDefaultConfig>();
}

#[test]
fn test_config_behind_arc_dyn() {
    let mut cfg = WxAispeechDefaultConfig::new();
    cfg.set_appid("dyn-appid");
    let arc: Arc<dyn WxAispeechConfigStorage> = Arc::new(cfg);
    assert_eq!(arc.appid(), Some("dyn-appid"));
    assert_eq!(arc.token(), None);
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：Bean 序列化/反序列化
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_aispeech_api_response_from_json_success() {
    let json = r#"{"code":0,"msg":"ok","request_id":"req-001","data":{"answer":"hi"}}"#;
    let resp: AispeechApiResponse<serde_json::Value> = AispeechApiResponse::from_json(json).unwrap();
    assert_eq!(resp.code, Some(0));
    assert_eq!(resp.msg, Some("ok".to_string()));
    assert_eq!(resp.request_id, Some("req-001".to_string()));
    assert!(resp.data.is_some());
}

#[test]
fn test_aispeech_api_response_error_code() {
    let json = r#"{"code":40001,"msg":"invalid token","request_id":"req-002"}"#;
    let resp: AispeechApiResponse<serde_json::Value> = AispeechApiResponse::from_json(json).unwrap();
    assert_eq!(resp.code, Some(40001));
    assert!(resp.data.is_none());
}

#[test]
fn test_aispeech_api_response_invalid_json() {
    let result: Result<AispeechApiResponse<serde_json::Value>, _> = AispeechApiResponse::from_json("not-json");
    assert!(result.is_err());
}

#[test]
fn test_api_response_generic_types() {
    let json = r#"{"code":0,"msg":"ok","data":"hello"}"#;
    let resp: AispeechApiResponse<String> = AispeechApiResponse::from_json(json).unwrap();
    assert_eq!(resp.data, Some("hello".to_string()));

    let json = r#"{"code":0,"msg":"ok","data":42}"#;
    let resp: AispeechApiResponse<i32> = AispeechApiResponse::from_json(json).unwrap();
    assert_eq!(resp.data, Some(42));

    let json = r#"{"code":0,"msg":"ok","data":[1,2,3]}"#;
    let resp: AispeechApiResponse<Vec<i32>> = AispeechApiResponse::from_json(json).unwrap();
    assert_eq!(resp.data, Some(vec![1, 2, 3]));
}

#[test]
fn test_async_task_result_roundtrip() {
    let json = r#"{"state":2,"msg":"completed","progress":100,"start":1700000000,"end":1700000100,"url":"https://result.example.com","total_count":50,"success_count":48,"fail_count":2}"#;
    let task: AsyncTaskResult = serde_json::from_str(json).unwrap();
    assert_eq!(task.state, Some(2));
    assert_eq!(task.progress, Some(100));
    assert_eq!(task.total_count, Some(50));
    let serialized = serde_json::to_string(&task).unwrap();
    let deserialized: AsyncTaskResult = serde_json::from_str(&serialized).unwrap();
    assert_eq!(task, deserialized);
}

#[test]
fn test_async_task_result_with_skills() {
    let json = r#"{"state":2,"progress":100,"success_skill_info_list":[{"id":1,"name":"skill1","intents":[{"id":10,"name":"intent1"}]}]}"#;
    let task: AsyncTaskResult = serde_json::from_str(json).unwrap();
    let skills = task.success_skill_info_list.unwrap();
    assert_eq!(skills.len(), 1);
    assert_eq!(skills[0].name, Some("skill1".to_string()));
    assert_eq!(skills[0].intents.as_ref().unwrap()[0].name, Some("intent1".to_string()));
}

#[test]
fn test_bot_intent_serde() {
    let json = r#"{"skill":"客服","intent":"查订单","disable":false,"questions":["我的订单"],"answers":["请提供订单号"]}"#;
    let intent: BotIntent = serde_json::from_str(json).unwrap();
    assert_eq!(intent.skill, Some("客服".to_string()));
    assert_eq!(intent.disable, Some(false));
    assert_eq!(intent.questions.as_ref().unwrap().len(), 1);
}

#[test]
fn test_dialog_query_request_serde() {
    let json = r#"{"query":"你好","env":"online","first_priority_skills":["s1"],"user_name":"testuser"}"#;
    let req: DialogQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.query, Some("你好".to_string()));
    assert_eq!(req.first_priority_skills.as_ref().unwrap(), &vec!["s1".to_string()]);
    let serialized = serde_json::to_string(&req).unwrap();
    assert!(serialized.contains("first_priority_skills"));
    assert!(serialized.contains("user_name"));
}

#[test]
fn test_dialog_result_serde() {
    let json = r#"{"answer":"你好","answer_type":"text","skill_name":"客服","intent_name":"打招呼","msg_id":"msg-001","options":[{"ans_node_name":"n1","title":"t","answer":"a","confidence":0.95}],"status":"success","slots":[{"name":"order_id","value":"123","norm":"123"}]}"#;
    let result: DialogResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.answer, Some("你好".to_string()));
    assert_eq!(result.options.as_ref().unwrap()[0].confidence, Some(0.95));
    assert_eq!(result.slots.as_ref().unwrap()[0].name, Some("order_id".to_string()));
}

#[test]
fn test_dialog_result_raw_answer_skip_serializing() {
    let result = DialogResult {
        answer: Some("test".to_string()),
        raw_answer: Some(serde_json::json!({"key": "value"})),
        ..Default::default()
    };
    let serialized = serde_json::to_string(&result).unwrap();
    assert!(serialized.contains("raw_answer"), "raw_answer with Some value should be serialized");;
}

#[test]
fn test_dialog_result_default() {
    let result = DialogResult::default();
    assert_eq!(result.answer, None);
    assert_eq!(result.raw_answer, None);
}

#[test]
fn test_publish_progress_serde() {
    let json = r#"{"end_time":"2026-01-01","progress":75,"status":1}"#;
    let p: PublishProgress = serde_json::from_str(json).unwrap();
    assert_eq!(p.progress, Some(75));
    assert_eq!(p.status, Some(1));
}

#[test]
fn test_empty_json_to_default() {
    let task: AsyncTaskResult = serde_json::from_str("{}").unwrap();
    assert_eq!(task.state, None);
    let intent: BotIntent = serde_json::from_str("{}").unwrap();
    assert_eq!(intent.skill, None);
    let progress: PublishProgress = serde_json::from_str("{}").unwrap();
    assert_eq!(progress.status, None);
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：Knowledge Bean
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_knowledge_info_serde() {
    let json = r#"{"id":"kb-001","title":"FAQ","type":"text","tenant_id":123}"#;
    let info: KnowledgeInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.id, Some("kb-001".to_string()));
    assert_eq!(info.title, Some("FAQ".to_string()));
    assert_eq!(info.tenant_id, Some(123));
}

#[test]
fn test_knowledge_info_from_json() {
    let json = r#"{"id":"kb-002","title":"test"}"#;
    let info = KnowledgeInfo::from_json(json).unwrap();
    assert_eq!(info.id, Some("kb-002".to_string()));
}

#[test]
fn test_knowledge_list_result_serde() {
    let json = r#"{"total":5,"data":[{"id":"k1","title":"t1"},{"id":"k2","title":"t2"}],"page":1,"page_size":10}"#;
    let result: KnowledgeListResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.total, Some(5));
    assert_eq!(result.data.as_ref().unwrap().len(), 2);
    assert_eq!(result.page, Some(1));
}

#[test]
fn test_knowledge_manual_create_request_serde() {
    let json = r#"{"title":"new-kb","content":"markdown content","description":"desc"}"#;
    let req: KnowledgeManualCreateRequest = serde_json::from_str(json).unwrap();
    let serialized = serde_json::to_string(&req).unwrap();
    let deserialized: KnowledgeManualCreateRequest = serde_json::from_str(&serialized).unwrap();
    assert_eq!(req, deserialized);
}

#[test]
fn test_knowledge_update_request_serde() {
    let json = r#"{"title":"updated","description":"desc","enable_status":"enabled"}"#;
    let req: KnowledgeUpdateRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.title, Some("updated".to_string()));
    assert_eq!(req.enable_status, Some("enabled".to_string()));
}

#[test]
fn test_knowledge_move_request_serde() {
    let json = r#"{"knowledge_ids":["k1","k2"],"source_kb_id":"src","target_kb_id":"tgt","mode":"reuse_vectors"}"#;
    let req: KnowledgeMoveRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.knowledge_ids.as_ref().unwrap().len(), 2);
    let serialized = serde_json::to_string(&req).unwrap();
    assert!(serialized.contains("knowledge_ids"));
}

#[test]
fn test_knowledge_tag_request_serde() {
    let json = r#"{"name":"tag1","color":"ff0000","sort_order":1}"#;
    let req: KnowledgeTagRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.name, Some("tag1".to_string()));
    assert_eq!(req.sort_order, Some(1));
}

#[test]
fn test_knowledge_url_create_request_serde() {
    let json = r#"{"title":"web-kb","url":"https://example.com/faq"}"#;
    let req: KnowledgeUrlCreateRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.url, Some("https://example.com/faq".to_string()));
    assert_eq!(req.title, Some("web-kb".to_string()));
}

#[test]
fn test_knowledge_move_progress_serde() {
    let json = r#"{"task_id":"t1","status":"running","progress":0.5,"total":100,"processed":50,"message":"ok"}"#;
    let p: KnowledgeMoveProgress = serde_json::from_str(json).unwrap();
    assert_eq!(p.task_id, Some("t1".to_string()));
    assert_eq!(p.progress, Some(0.5));
    assert_eq!(p.total, Some(100));
    assert_eq!(p.processed, Some(50));
}

// ═══════════════════════════════════════════════════════════════
// SOURCE_PARITY：服务构建
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_service_impl_construction() {
    let mut cfg = WxAispeechDefaultConfig::new();
    cfg.set_appid("test-appid");
    cfg.set_token("test-token");
    let service = wx_rust_aispeech::api::r#impl::WxAispeechServiceImpl::new_arc(Arc::new(cfg));
    let stored_cfg = service.config_storage();
    assert_eq!(stored_cfg.appid(), Some("test-appid"));
    assert_eq!(stored_cfg.token(), Some("test-token"));
    let _client = service.http_client();
    assert!(service.dialog_service().is_some());
    assert!(service.knowledge_service().is_some());
}

#[test]
fn test_service_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<wx_rust_aispeech::api::r#impl::WxAispeechServiceImpl>();
}

#[test]
fn test_service_set_config_storage() {
    let cfg1 = WxAispeechDefaultConfig::new();
    let service = wx_rust_aispeech::api::r#impl::WxAispeechServiceImpl::new_arc(Arc::new(cfg1));
    let mut cfg2 = WxAispeechDefaultConfig::new();
    cfg2.set_appid("new-appid");
    service.set_config_storage(Arc::new(cfg2));
    let stored = service.config_storage();
    assert_eq!(stored.appid(), Some("new-appid"));
}

/// VALUE_ADD：raw_answer 为 None 时不序列化。
#[test]
fn test_dialog_result_raw_answer_none_skipped() {
    let result = DialogResult {
        answer: Some("test".to_string()),
        raw_answer: None,
        ..Default::default()
    };
    let serialized = serde_json::to_string(&result).unwrap();
    assert!(!serialized.contains("raw_answer"), "raw_answer=None should be skipped");
}
