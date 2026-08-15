//! wx-rust-pay Bean 综合测试（SOURCE_PARITY + VALUE_ADD）。

use wx_rust_pay::bean::WxPayApiData;
use wx_rust_pay::bean::notify::*;
use wx_rust_pay::bean::order::*;

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
fn test_native_order_result_none() {
    let result: WxPayNativeOrderResult = serde_json::from_str("{}").unwrap();
    assert_eq!(result.code_url, None);
}

#[test]
fn test_mp_order_result_serde() {
    let json = r#"{"appId":"wx1234","timeStamp":"1700000000","nonceStr":"abc","package":"prepay_id=wx2024","signType":"RSA","paySign":"sign123"}"#;
    let result: WxPayMpOrderResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.app_id, Some("wx1234".to_string()));
    assert_eq!(result.package_value, Some("prepay_id=wx2024".to_string()));
}

#[test]
fn test_mp_order_result_roundtrip() {
    let result = WxPayMpOrderResult {
        app_id: Some("wx1234".to_string()),
        time_stamp: Some("1234567890".to_string()),
        nonce_str: Some("nonce".to_string()),
        package_value: Some("prepay_id=wx".to_string()),
        sign_type: Some("RSA".to_string()),
        pay_sign: Some("sign".to_string()),
    };
    let serialized = serde_json::to_string(&result).unwrap();
    let deserialized: WxPayMpOrderResult = serde_json::from_str(&serialized).unwrap();
    assert_eq!(result, deserialized);
}

#[test]
fn test_app_order_result_serde() {
    let json = r#"{"sign":"sign123","prepayId":"wx2024","partnerId":"mch123","appId":"wx1234","packageValue":"Sign=WXPay","timeStamp":"1700000000","nonceStr":"nonce"}"#;
    let result: WxPayAppOrderResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.prepay_id, Some("wx2024".to_string()));
    assert_eq!(result.partner_id, Some("mch123".to_string()));
}

#[test]
fn test_mweb_order_result_from_xml() {
    let result = WxPayMwebOrderResult {
        mweb_url: Some(
            "https://wx.tenpay.com/cgi-bin/mmpayweb-bin/checkmweb?prepay_id=wx2024".to_string(),
        ),
    };
    let xml = result.to_xml().unwrap();
    let deserialized = WxPayMwebOrderResult::from_xml(&xml).unwrap();
    assert_eq!(deserialized.mweb_url, result.mweb_url);
}

#[test]
fn test_mweb_order_result_roundtrip() {
    let result = WxPayMwebOrderResult {
        mweb_url: Some("https://wx.tenpay.com/test".to_string()),
    };
    let xml = result.to_xml().unwrap();
    let deserialized = WxPayMwebOrderResult::from_xml(&xml).unwrap();
    assert_eq!(result, deserialized);
}

// ═══ Notify Results ═══

#[test]
fn test_order_notify_result_serde() {
    let json = r#"{"return_code":"SUCCESS","return_msg":"OK","result_code":"SUCCESS","appid":"wx1234","mch_id":"mch123","nonce_str":"nonce","sign":"sign123","trade_type":"NATIVE","total_fee":100,"out_trade_no":"ORDER-001","transaction_id":"4200001234"}"#;
    let result: WxPayOrderNotifyResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.return_code, Some("SUCCESS".to_string()));
    assert_eq!(result.out_trade_no, Some("ORDER-001".to_string()));
}

#[test]
fn test_refund_notify_result_serde() {
    let json = r#"{"return_code":"SUCCESS","appid":"wx1234","mch_id":"mch123","out_trade_no":"ORDER-001","out_refund_no":"REFUND-001","refund_id":"RF-001"}"#;
    let result: WxPayRefundNotifyResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.appid, Some("wx1234".to_string()));
    assert_eq!(result.mch_id, Some("mch123".to_string()));
}

#[test]
fn test_refund_notify_v3_result_serde() {
    let json = r#"{"rawData":{"id":"ev-001","createTime":"2024-01-01","resourceType":"encrypt-resource"},"result":{"mchid":"mch123","out_trade_no":"ORDER-001","transaction_id":"4200001234","out_refund_no":"REFUND-001","refund_id":"RF-001"}}"#;
    let result: WxPayRefundNotifyV3Result = serde_json::from_str(json).unwrap();
    assert!(result.raw_data.is_some());
    assert!(result.result.is_some());
    let inner = result.result.unwrap();
    assert_eq!(inner.out_trade_no, Some("ORDER-001".to_string()));
    assert_eq!(inner.out_refund_no, Some("REFUND-001".to_string()));
}

#[test]
fn test_notify_v3_result_serde() {
    let json = r#"{"rawData":{"id":"ev-002","createTime":"2024-01-01"},"result":{"appid":"wx1234","mchid":"mch123","out_trade_no":"ORDER-002","transaction_id":"4200005678","trade_state":"SUCCESS","trade_type":"NATIVE","amount":{"total":100,"payer_total":100,"currency":"CNY"}}}"#;
    let result: WxPayNotifyV3Result = serde_json::from_str(json).unwrap();
    assert!(result.result.is_some());
    let inner = result.result.unwrap();
    assert_eq!(inner.appid, Some("wx1234".to_string()));
    assert_eq!(inner.out_trade_no, Some("ORDER-002".to_string()));
}

#[test]
fn test_origin_notify_response_serde() {
    let json = r#"{"id":"ev-003","create_time":"2024-01-01","event_type":"TRANSACTION.SUCCESS","summary":"支付成功"}"#;
    let resp: OriginNotifyResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.id, Some("ev-003".to_string()));
    assert_eq!(resp.event_type, Some("TRANSACTION.SUCCESS".to_string()));
}

// ═══ API Data ═══

#[test]
fn test_wx_pay_api_data_serde() {
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
    assert_eq!(data.request_data, Some("<xml>...</xml>".to_string()));
    assert_eq!(data.error_msg, None);
}

// ═══ VALUE_ADD ═══

#[test]
fn test_order_results_empty_json() {
    let native: WxPayNativeOrderResult = serde_json::from_str("{}").unwrap();
    assert_eq!(native.code_url, None);

    let mp: WxPayMpOrderResult = serde_json::from_str("{}").unwrap();
    assert_eq!(mp.app_id, None);

    let app: WxPayAppOrderResult = serde_json::from_str("{}").unwrap();
    assert_eq!(app.prepay_id, None);
}

#[test]
fn test_mp_order_result_none_skipping() {
    let result = WxPayMpOrderResult {
        app_id: Some("wx1234".to_string()),
        ..Default::default()
    };
    let serialized = serde_json::to_string(&result).unwrap();
    assert!(!serialized.contains("nonceStr"));
    assert!(serialized.contains("appId"));
}

#[test]
fn test_refund_notify_v3_empty() {
    let result: WxPayRefundNotifyV3Result = serde_json::from_str("{}").unwrap();
    assert_eq!(result.raw_data, None);
    assert_eq!(result.result, None);
}
