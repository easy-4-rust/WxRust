//! Phase 2 Batch 2.2: pay 子域 Service 测试
//!
//! 镜像 Java WxPayRefundQueryResultTest / WxPayRedpackQueryResultTest /
//! WxPayOrderQueryResultTest / WxPayBillResultTest

use wx_rust_pay::bean::notify::*;
use wx_rust_pay::bean::order::*;

// ═══ Order Notify Result ═══

#[test]
fn test_order_notify_result_full() {
    let json = r#"{"return_code":"SUCCESS","return_msg":"OK","result_code":"SUCCESS","err_code":"","err_code_des":"","appid":"wx1234","mch_id":"mch123","nonce_str":"nonce123","sign":"ABC","trade_type":"NATIVE","bank_type":"CMB","total_fee":100,"settlement_total_fee":100,"cash_fee":0,"out_trade_no":"ORDER-001","transaction_id":"4200001234","time_end":"20240101120000","openid":"ox123"}"#;
    let result: WxPayOrderNotifyResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.return_code, Some("SUCCESS".to_string()));
    assert_eq!(result.appid, Some("wx1234".to_string()));
    assert_eq!(result.out_trade_no, Some("ORDER-001".to_string()));
    assert_eq!(result.transaction_id, Some("4200001234".to_string()));
}

#[test]
fn test_order_notify_result_error() {
    let json = r#"{"return_code":"FAIL","return_msg":"签名错误"}"#;
    let result: WxPayOrderNotifyResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.return_code, Some("FAIL".to_string()));
    assert_eq!(result.return_msg, Some("签名错误".to_string()));
}

// ═══ Refund Notify Result ═══

#[test]
fn test_refund_notify_result_serde() {
    let json = r#"{"return_code":"SUCCESS","appid":"wx1234","mch_id":"mch123","out_trade_no":"ORDER-001","out_refund_no":"REFUND-001","refund_id":"RF-001","refund_fee":50,"total_fee":100}"#;
    let result: WxPayRefundNotifyResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.appid, Some("wx1234".to_string()));
    assert_eq!(result.mch_id, Some("mch123".to_string()));
}

// ═══ Refund Notify V3 Result ═══

#[test]
fn test_refund_notify_v3_full() {
    let json = r#"{"rawData":{"id":"ev-001","createTime":"2024-01-01","resourceType":"encrypt-resource"},"result":{"mchid":"mch123","out_trade_no":"ORDER-001","transaction_id":"4200001234","out_refund_no":"REFUND-001","refund_id":"RF-001","refund_status":"SUCCESS","amount":{"total":100,"refund":50,"payer_total":100,"payer_refund":50}}}"#;
    let result: WxPayRefundNotifyV3Result = serde_json::from_str(json).unwrap();
    assert!(result.raw_data.is_some());
    assert!(result.result.is_some());
    let inner = result.result.unwrap();
    assert_eq!(inner.out_trade_no, Some("ORDER-001".to_string()));
    assert_eq!(inner.out_refund_no, Some("REFUND-001".to_string()));
    assert_eq!(inner.refund_id, Some("RF-001".to_string()));
}

// ═══ Notify V3 Result ═══

#[test]
fn test_notify_v3_full() {
    let json = r#"{"rawData":{"id":"ev-002","createTime":"2024-01-01"},"result":{"appid":"wx1234","mchid":"mch123","out_trade_no":"ORDER-002","transaction_id":"4200005678","trade_state":"SUCCESS","trade_type":"NATIVE","amount":{"total":100,"payer_total":100,"currency":"CNY"},"payer":{"openid":"ox123"}}}"#;
    let result: WxPayNotifyV3Result = serde_json::from_str(json).unwrap();
    let inner = result.result.unwrap();
    assert_eq!(inner.appid, Some("wx1234".to_string()));
    assert_eq!(inner.out_trade_no, Some("ORDER-002".to_string()));
    assert_eq!(inner.trade_state, Some("SUCCESS".to_string()));
}

// ═══ Origin Notify Response ═══

#[test]
fn test_origin_notify_response_serde() {
    let json = r#"{"id":"ev-003","create_time":"2024-01-01","event_type":"TRANSACTION.SUCCESS","summary":"支付成功","resource_type":"encrypt-resource","resource":{"algorithm":"AEAD_AES_256_GCM","ciphertext":"encrypted","nonce":"nonce123"}}"#;
    let resp: OriginNotifyResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.id, Some("ev-003".to_string()));
    assert_eq!(resp.event_type, Some("TRANSACTION.SUCCESS".to_string()));
    assert!(resp.resource.is_some());
}

// ═══ Mweb Order Result (XML) ═══

#[test]
fn test_mweb_order_result_roundtrip() {
    let result = WxPayMwebOrderResult {
        mweb_url: Some("https://wx.tenpay.com/test".to_string()),
    };
    let xml = result.to_xml().unwrap();
    let deserialized = WxPayMwebOrderResult::from_xml(&xml).unwrap();
    assert_eq!(deserialized.mweb_url, result.mweb_url);
}

// ═══ VALUE_ADD ═══

#[test]
fn test_order_notify_empty() {
    let result: WxPayOrderNotifyResult = serde_json::from_str("{}").unwrap();
    assert_eq!(result.return_code, None);
    assert_eq!(result.appid, None);
}

#[test]
fn test_refund_notify_v3_empty() {
    let result: WxPayRefundNotifyV3Result = serde_json::from_str("{}").unwrap();
    assert_eq!(result.raw_data, None);
    assert_eq!(result.result, None);
}

#[test]
fn test_origin_notify_response_empty() {
    let result: OriginNotifyResponse = serde_json::from_str("{}").unwrap();
    assert_eq!(result.id, None);
    assert_eq!(result.resource, None);
}
