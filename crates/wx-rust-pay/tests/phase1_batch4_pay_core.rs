//! Phase 1 Batch 1.4: wx-rust-pay 核心测试

use std::collections::HashMap;
use wx_rust_pay::bean::WxPayApiData;
use wx_rust_pay::bean::notify::*;
use wx_rust_pay::bean::order::*;
use wx_rust_pay::util::sign_utils::SignUtils;

// ═══ Order Results ═══

#[test]
fn test_native_order_result_serde() {
    let json = r#"{"codeUrl":"weixin://wxpay/bizpayurl?pr=xxx"}"#;
    let result: WxPayNativeOrderResult = serde_json::from_str(json).unwrap();
    assert_eq!(
        result.code_url,
        Some("weixin://wxpay/bizpayurl?pr=xxx".to_string())
    );
}

#[test]
fn test_mp_order_result_serde() {
    let json = r#"{"appId":"wx1234","timeStamp":"1700000000","nonceStr":"abc","package":"prepay_id=wx2024","signType":"RSA","paySign":"sign123"}"#;
    let result: WxPayMpOrderResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.app_id, Some("wx1234".to_string()));
    assert_eq!(result.package_value, Some("prepay_id=wx2024".to_string()));
}

#[test]
fn test_app_order_result_serde() {
    let json = r#"{"sign":"sign123","prepayId":"wx2024","partnerId":"mch123","appId":"wx1234","packageValue":"Sign=WXPay","timeStamp":"1700000000","nonceStr":"nonce"}"#;
    let result: WxPayAppOrderResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.prepay_id, Some("wx2024".to_string()));
}

// ═══ Notify Results ═══

#[test]
fn test_order_notify_result_serde() {
    let json = r#"{"return_code":"SUCCESS","return_msg":"OK","result_code":"SUCCESS","appid":"wx1234","mch_id":"mch123","out_trade_no":"ORDER-001","total_fee":100}"#;
    let result: WxPayOrderNotifyResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.return_code, Some("SUCCESS".to_string()));
    assert_eq!(result.out_trade_no, Some("ORDER-001".to_string()));
}

#[test]
fn test_refund_notify_v3_result_serde() {
    let json = r#"{"rawData":{"id":"ev-001"},"result":{"mchid":"mch123","out_trade_no":"ORDER-001","out_refund_no":"REFUND-001"}}"#;
    let result: WxPayRefundNotifyV3Result = serde_json::from_str(json).unwrap();
    assert!(result.result.is_some());
    assert_eq!(
        result.result.unwrap().out_trade_no,
        Some("ORDER-001".to_string())
    );
}

#[test]
fn test_notify_v3_result_serde() {
    let json = r#"{"rawData":{"id":"ev-002"},"result":{"appid":"wx1234","out_trade_no":"ORDER-002","transaction_id":"4200005678","trade_state":"SUCCESS"}}"#;
    let result: WxPayNotifyV3Result = serde_json::from_str(json).unwrap();
    assert_eq!(result.result.unwrap().appid, Some("wx1234".to_string()));
}

// ═══ SignUtils ═══

#[test]
fn test_sign_utils_create_sign_md5() {
    // 镜像 Java SignUtils.createSign with MD5
    let mut params = HashMap::new();
    params.insert("appid".to_string(), "wx1234".to_string());
    params.insert("mch_id".to_string(), "mch123".to_string());
    params.insert("body".to_string(), "test".to_string());
    let result = SignUtils::create_sign(&params, Some("MD5"), "test_key", &[]);
    assert!(result.is_ok());
    let sign = result.unwrap();
    assert!(!sign.is_empty());
    assert_eq!(sign.len(), 32); // MD5 hex is 32 chars, uppercase
}

#[test]
fn test_sign_utils_create_sign_hmac_sha256() {
    let mut params = HashMap::new();
    params.insert("appid".to_string(), "wx1234".to_string());
    params.insert("body".to_string(), "test".to_string());
    let result = SignUtils::create_sign(&params, Some("HMAC-SHA256"), "test_key", &[]);
    assert!(result.is_ok());
    let sign = result.unwrap();
    assert_eq!(sign.len(), 64); // SHA256 hex is 64 chars
}

#[test]
fn test_sign_utils_create_sign_default_md5() {
    // 空 signType 默认 MD5
    let mut params = HashMap::new();
    params.insert("test".to_string(), "value".to_string());
    let result = SignUtils::create_sign(&params, None, "key", &[]);
    assert!(result.is_ok());
}

#[test]
fn test_sign_utils_create_sign_skips_ignored() {
    let mut params = HashMap::new();
    params.insert("appid".to_string(), "wx1234".to_string());
    params.insert("secret".to_string(), "should_be_ignored".to_string());
    let result1 = SignUtils::create_sign(&params, Some("MD5"), "key", &["secret"]);
    params.remove("secret");
    let result2 = SignUtils::create_sign(&params, Some("MD5"), "key", &[]);
    assert_eq!(result1.unwrap(), result2.unwrap());
}

// ═══ API Data ═══

#[test]
fn test_api_data_serde() {
    let data = WxPayApiData::new(
        Some("https://api.mch.weixin.qq.com/pay/unifiedorder".to_string()),
        Some("<xml>...</xml>".to_string()),
        Some("<xml>SUCCESS</xml>".to_string()),
        None,
    );
    assert_eq!(
        data.url,
        Some("https://api.mch.weixin.qq.com/pay/unifiedorder".to_string())
    );
}

// ═══ VALUE_ADD ═══

#[test]
fn test_order_results_empty() {
    let native: WxPayNativeOrderResult = serde_json::from_str("{}").unwrap();
    assert_eq!(native.code_url, None);
    let mp: WxPayMpOrderResult = serde_json::from_str("{}").unwrap();
    assert_eq!(mp.app_id, None);
}
