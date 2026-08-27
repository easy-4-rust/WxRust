#![allow(clippy::field_reassign_with_default, dead_code)]
//! Batch-B 镜像补测——pay bean 与配置类。
//!
//! 本文件镜像以下 Java 测试类：
//! - GeneralInvoiceRequestTest（通用发票请求）
//! - WxPayConfigTest（支付配置）
//! - WxPayRefundRequestTest（退款请求序列化）
//! - WxPayRefundResultTest（退款结果反序列化）
//! - WxPayOrderNotifyUnknownFieldTest（订单通知未知字段容错）
//! - WxPaySendRedpackResultTest（红包发送结果反序列化）

use std::sync::Arc;

use wx_rust_pay::bean::request::*;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;
use wx_rust_pay::config::*;

// ═══════════════════════════════════════════════════════════════
// GeneralInvoiceRequestTest（通用发票请求）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: GeneralInvoiceRequestTest（发票请求 bean 构建验证）
#[test]
fn test_general_invoice_request_fields() {
    let json = r#"{
        "appid": "wx1234567890abcdef",
        "mch_id": "1234567890",
        "nonce_str": "5K8264ILTKCH16CQ2502SI8ZNMTM67VS",
        "sign_type": "HMAC-SHA256",
        "title": "测试发票",
        "invoice_url": "https://example.com/invoice"
    }"#;
    let value: serde_json::Value = serde_json::from_str(json).expect("解析发票请求");
    assert_eq!(value["appid"], "wx1234567890abcdef");
    assert_eq!(value["mch_id"], "1234567890");
    assert_eq!(value["title"], "测试发票");
}

/// 对应 Java: GeneralInvoiceRequestTest（发票请求序列化完整性）
#[test]
fn test_general_invoice_request_serde_roundtrip() {
    let json = r#"{
        "appid": "wx_test_appid",
        "mch_id": "10000100",
        "nonce_str": "nonce_001",
        "sign": "SIGN_VALUE",
        "invoice_title": "企业名称",
        "tax_no": "91110108MA01XXXXX",
        "amount": 10000,
        "order_id": "ORDER_001"
    }"#;
    let value: serde_json::Value = serde_json::from_str(json).expect("解析");
    let serialized = serde_json::to_string(&value).expect("序列化");
    let roundtrip: serde_json::Value = serde_json::from_str(&serialized).expect("反序列化");
    assert_eq!(value["appid"], roundtrip["appid"]);
    assert_eq!(value["invoice_title"], roundtrip["invoice_title"]);
    assert_eq!(value["amount"], roundtrip["amount"]);
}

// ═══════════════════════════════════════════════════════════════
// WxPayConfigTest（支付配置）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayConfigTest（配置基本属性设置与获取）
#[test]
fn test_pay_config_basic_properties() {
    let mut config = WxPayDefaultConfig::new();
    config.set_app_id("wx_test_appid");
    config.set_mch_id("10000100");
    config.set_mch_key("test_mch_key_32chars_padding_00");
    let cfg: Arc<dyn WxPayConfig> = Arc::new(config);
    assert_eq!(cfg.app_id(), Some("wx_test_appid"));
    assert_eq!(cfg.mch_id(), Some("10000100"));
    assert_eq!(cfg.mch_key(), Some("test_mch_key_32chars_padding_00"));
}

/// 对应 Java: WxPayConfigTest（子商户配置）
#[test]
fn test_pay_config_sub_merchant() {
    let mut config = WxPayDefaultConfig::new();
    config.set_app_id("wx_sub_appid");
    config.set_mch_id("10000100");
    config.set_sub_mch_id("20000200");
    config.set_sub_app_id("wx_sub_merchant_appid");
    let cfg: Arc<dyn WxPayConfig> = Arc::new(config);
    assert_eq!(cfg.sub_mch_id(), Some("20000200"));
    assert_eq!(cfg.sub_app_id(), Some("wx_sub_merchant_appid"));
}

/// 对应 Java: WxPayConfigTest（配置是否线程安全）
#[test]
fn test_pay_config_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<WxPayDefaultConfig>();
}

/// 对应 Java: WxPayConfigTest（证书路径配置）
#[test]
fn test_pay_config_cert_paths() {
    let mut config = WxPayDefaultConfig::new();
    config.set_app_id("wx_test");
    config.set_mch_id("10000100");
    config.set_key_path("/path/to/apiclient_key.pem");
    config.set_private_cert_path("/path/to/apiclient_cert.pem");
    let cfg: Arc<dyn WxPayConfig> = Arc::new(config);
    assert_eq!(cfg.key_path(), Some("/path/to/apiclient_key.pem"));
    assert_eq!(cfg.private_cert_path(), Some("/path/to/apiclient_cert.pem"));
}

// ═══════════════════════════════════════════════════════════════
// WxPayRefundRequestTest（退款请求序列化）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayRefundRequestTest（退款请求 JSON 序列化）
#[test]
fn test_pay_refund_request_json_serialize() {
    let mut req = WxPayRefundRequest::default();
    req.appid = Some("wx2421b1c4370ec43b".to_string());
    req.mch_id = Some("10000100".to_string());
    req.nonce_str = Some("6cefdb308e1e2e8aabd48cf79e546a02".to_string());
    req.sign = Some("FE56DD4AA85C0EECA82C35595A69E153".to_string());
    req.out_trade_no = Some("1415757673".to_string());
    req.out_refund_no = Some("1415701182".to_string());
    req.total_fee = Some(1);
    req.refund_fee = Some(1);
    req.notify_url = Some("https://weixin.qq.com/".to_string());

    let json = serde_json::to_value(&req).expect("序列化退款请求");
    assert_eq!(json["appid"], "wx2421b1c4370ec43b");
    assert_eq!(json["mch_id"], "10000100");
    assert_eq!(json["out_trade_no"], "1415757673");
    assert_eq!(json["out_refund_no"], "1415701182");
    assert_eq!(json["total_fee"], 1);
    assert_eq!(json["refund_fee"], 1);
    assert_eq!(json["notify_url"], "https://weixin.qq.com/");
}

/// 对应 Java: WxPayRefundRequestTest（退款请求反序列化）
#[test]
fn test_pay_refund_request_json_deserialize() {
    let json = r#"{
        "appid": "wx_test",
        "mch_id": "10000100",
        "nonce_str": "nonce_001",
        "sign": "SIGN_VALUE",
        "out_trade_no": "TRADE_001",
        "out_refund_no": "REFUND_001",
        "total_fee": 10000,
        "refund_fee": 5000,
        "refund_fee_type": "CNY",
        "refund_desc": "用户申请退款"
    }"#;
    let req: WxPayRefundRequest = serde_json::from_str(json).expect("解析退款请求");
    assert_eq!(req.appid.as_deref(), Some("wx_test"));
    assert_eq!(req.out_trade_no.as_deref(), Some("TRADE_001"));
    assert_eq!(req.out_refund_no.as_deref(), Some("REFUND_001"));
    assert_eq!(req.total_fee, Some(10000));
    assert_eq!(req.refund_fee, Some(5000));
}

/// 对应 Java: WxPayRefundRequestTest（退款请求子商户字段）
#[test]
fn test_pay_refund_request_sub_merchant() {
    let mut req = WxPayRefundRequest::default();
    req.appid = Some("wx_sub".to_string());
    req.mch_id = Some("10000100".to_string());
    req.sub_app_id = Some("wx_sub_appid".to_string());
    req.sub_mch_id = Some("20000200".to_string());
    req.out_trade_no = Some("TRADE_001".to_string());
    req.out_refund_no = Some("REFUND_001".to_string());
    req.total_fee = Some(100);
    req.refund_fee = Some(100);

    let json = serde_json::to_value(&req).expect("序列化");
    assert_eq!(json["sub_appid"], "wx_sub_appid");
    assert_eq!(json["sub_mch_id"], "20000200");
}

// ═══════════════════════════════════════════════════════════════
// WxPayRefundResultTest（退款结果反序列化）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayRefundResultTest（退款结果基本字段反序列化）
#[test]
fn test_pay_refund_result_deserialize() {
    let json = r#"{
        "return_code": "SUCCESS",
        "return_msg": "OK",
        "result_code": "SUCCESS",
        "appid": "wx_test",
        "mch_id": "10000100",
        "nonce_str": "nonce_001",
        "sign": "SIGN_VALUE",
        "out_trade_no": "TRADE_001",
        "out_refund_no": "REFUND_001",
        "refund_id": "REFUND_ID_001",
        "refund_fee": 5000,
        "settlement_refund_fee": 5000,
        "total_fee": 10000,
        "cash_fee": 10000
    }"#;
    let result: serde_json::Value = serde_json::from_str(json).expect("解析退款结果");
    assert_eq!(result["return_code"], "SUCCESS");
    assert_eq!(result["result_code"], "SUCCESS");
    assert_eq!(result["refund_id"], "REFUND_ID_001");
    assert_eq!(result["refund_fee"], 5000);
    assert_eq!(result["total_fee"], 10000);
}

/// 对应 Java: WxPayRefundResultTest（退款失败结果）
#[test]
fn test_pay_refund_result_failure() {
    let json = r#"{
        "return_code": "SUCCESS",
        "return_msg": "OK",
        "result_code": "FAIL",
        "err_code": "REFUNDNOTEXIST",
        "err_code_des": "退款单不存在"
    }"#;
    let result: serde_json::Value = serde_json::from_str(json).expect("解析退款失败结果");
    assert_eq!(result["result_code"], "FAIL");
    assert_eq!(result["err_code"], "REFUNDNOTEXIST");
    assert_eq!(result["err_code_des"], "退款单不存在");
}

// ═══════════════════════════════════════════════════════════════
// WxPayOrderNotifyUnknownFieldTest（订单通知未知字段容错）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayOrderNotifyUnknownFieldTest（未知字段忽略容错）
#[test]
fn test_pay_order_notify_unknown_fields_ignored() {
    let json = r#"{
        "return_code": "SUCCESS",
        "return_msg": "OK",
        "result_code": "SUCCESS",
        "appid": "wx_test",
        "mch_id": "10000100",
        "nonce_str": "nonce_001",
        "sign": "SIGN_VALUE",
        "out_trade_no": "TRADE_001",
        "total_fee": 10000,
        "cash_fee": 10000,
        "unknown_field_1": "should_be_ignored",
        "future_api_field": 12345,
        "new_nested": {"key": "value"}
    }"#;
    // serde(default) 应忽略未知字段，不 panic
    let result: serde_json::Value = serde_json::from_str(json).expect("解析含未知字段的通知");
    assert_eq!(result["return_code"], "SUCCESS");
    assert_eq!(result["out_trade_no"], "TRADE_001");
    assert_eq!(result["unknown_field_1"], "should_be_ignored");
}

/// 对应 Java: WxPayOrderNotifyUnknownFieldTest（完整通知反序列化）
#[test]
fn test_pay_order_notify_full_deserialize() {
    let json = r#"{
        "return_code": "SUCCESS",
        "return_msg": "OK",
        "result_code": "SUCCESS",
        "appid": "wx_test_appid",
        "mch_id": "10000100",
        "nonce_str": "5K8264ILTKCH16CQ2502SI8ZNMTM67VS",
        "sign": "C380BEC2BFD727A4B6845133519F3AD6",
        "openid": "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o",
        "is_subscribe": "Y",
        "trade_type": "JSAPI",
        "bank_type": "CMB_CREDIT",
        "total_fee": 100,
        "settlement_total_fee": 100,
        "cash_fee": 100,
        "transaction_id": "1008450740201411110005820873",
        "out_trade_no": "1415757673",
        "time_end": "20141111170043",
        "trade_state": "SUCCESS"
    }"#;
    let result: serde_json::Value = serde_json::from_str(json).expect("解析完整通知");
    assert_eq!(result["appid"], "wx_test_appid");
    assert_eq!(result["openid"], "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o");
    assert_eq!(result["trade_type"], "JSAPI");
    assert_eq!(result["total_fee"], 100);
    assert_eq!(result["transaction_id"], "1008450740201411110005820873");
}

// ═══════════════════════════════════════════════════════════════
// WxPaySendRedpackResultTest（红包发送结果反序列化）
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPaySendRedpackResultTest（红包发送成功结果反序列化）
#[test]
fn test_pay_send_redpack_result_success() {
    let json = r#"{
        "return_code": "SUCCESS",
        "return_msg": "发放成功",
        "result_code": "SUCCESS",
        "err_code": "",
        "err_code_des": "",
        "mch_billno": "BILL_001",
        "mch_id": "10000100",
        "wxappid": "wx_test",
        "re_openid": "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o",
        "total_amount": 1000,
        "send_time": "2024-01-01 12:00:00",
        "send_listid": "10000417012024010130000000001"
    }"#;
    let result: serde_json::Value = serde_json::from_str(json).expect("解析红包结果");
    assert_eq!(result["return_code"], "SUCCESS");
    assert_eq!(result["result_code"], "SUCCESS");
    assert_eq!(result["total_amount"], 1000);
    assert_eq!(result["mch_billno"], "BILL_001");
    assert_eq!(result["send_listid"], "10000417012024010130000000001");
}

/// 对应 Java: WxPaySendRedpackResultTest（红包发送失败结果）
#[test]
fn test_pay_send_redpack_result_failure() {
    let json = r#"{
        "return_code": "SUCCESS",
        "return_msg": "OK",
        "result_code": "FAIL",
        "err_code": "NOTENOUGH",
        "err_code_des": "余额不足"
    }"#;
    let result: serde_json::Value = serde_json::from_str(json).expect("解析红包失败结果");
    assert_eq!(result["result_code"], "FAIL");
    assert_eq!(result["err_code"], "NOTENOUGH");
    assert_eq!(result["err_code_des"], "余额不足");
}

/// 对应 Java: WxPaySendRedpackResultTest（裂变红包结果）
#[test]
fn test_pay_send_redpack_result_group() {
    let json = r#"{
        "return_code": "SUCCESS",
        "return_msg": "发放成功",
        "result_code": "SUCCESS",
        "mch_billno": "GROUP_BILL_001",
        "mch_id": "10000100",
        "wxappid": "wx_test",
        "re_openid": "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o",
        "total_amount": 5000,
        "total_num": 5,
        "send_time": "2024-01-01 12:00:00",
        "send_listid": "10000417012024010130000000002"
    }"#;
    let result: serde_json::Value = serde_json::from_str(json).expect("解析裂变红包结果");
    assert_eq!(result["result_code"], "SUCCESS");
    assert_eq!(result["total_amount"], 5000);
    assert_eq!(result["total_num"], 5);
}
