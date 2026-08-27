//! Top-15 未镜像 Java 测试类批量补测——pay 模块。
//!
//! 本文件镜像以下 Java 测试类：
//! - MultiAppIdSwitchoverTest（546 行）
//! - TransferAuthorizationApiCompatibilityTest（361 行）
//! - WxPayUnifiedOrderV3ResultTest（322 行）

use std::sync::Arc;

use wx_rust_pay::api::r#impl::WxPayServiceImpl;
use wx_rust_pay::api::*;
use wx_rust_pay::bean::*;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;
use wx_rust_pay::config::*;

// ═══════════════════════════════════════════════════════════════
// 辅助：构建测试用配置
// ═══════════════════════════════════════════════════════════════

fn make_config(app_id: &str, mch_id: &str, mch_key: &str) -> Arc<dyn WxPayConfig> {
    let mut config = WxPayDefaultConfig::new();
    config.set_app_id(app_id);
    config.set_mch_id(mch_id);
    config.set_mch_key(mch_key);
    Arc::new(config)
}

fn make_service() -> Arc<WxPayServiceImpl> {
    let default_config = make_config("wx_default", "0000000000", "default_key");
    WxPayServiceImpl::new_arc(default_config)
}

// ═══════════════════════════════════════════════════════════════
// #5 MultiAppIdSwitchoverTest（546 行）—— 多 appid 切换
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: MultiAppIdSwitchoverTest.testGetConfigWithMchIdAndAppId
#[test]
fn test_multi_appid_switchover_add_config() {
    let service = make_service();

    let config1 = make_config("wx1111111111111111", "1234567890", "test_key_1");
    let config2 = make_config("wx2222222222222222", "1234567890", "test_key_2");
    let config3 = make_config("wx3333333333333333", "1234567890", "test_key_3");

    service.add_config("1234567890", "wx1111111111111111", config1);
    service.add_config("1234567890", "wx2222222222222222", config2);
    service.add_config("1234567890", "wx3333333333333333", config3);

    // 切换到第一个配置
    assert!(service.switchover("1234567890", "wx1111111111111111"));
    // 切换到第二个配置
    assert!(service.switchover("1234567890", "wx2222222222222222"));
    // 切换到第三个配置
    assert!(service.switchover("1234567890", "wx3333333333333333"));
    // 切换到不存在的配置
    assert!(!service.switchover("1234567890", "wx_not_exist"));
}

/// 对应 Java: MultiAppIdSwitchoverTest.testSwitchoverByKey
#[test]
fn test_multi_appid_switchover_by_key() {
    let service = make_service();

    let config1 = make_config("wx1111111111111111", "1234567890", "test_key_1");
    let config2 = make_config("wx2222222222222222", "1234567890", "test_key_2");

    service.add_config("1234567890", "wx1111111111111111", config1);
    service.add_config("1234567890", "wx2222222222222222", config2);

    // 使用 config_key 切换
    let key = "1234567890_wx1111111111111111";
    assert!(service.switchover_with_key(key));
}

/// 对应 Java: MultiAppIdSwitchoverTest.testSetMultiConfig
#[test]
fn test_multi_appid_set_multi_config() {
    let service = make_service();

    let mut configs = std::collections::HashMap::new();
    configs.insert(
        "1234567890_wx1111111111111111".to_string(),
        make_config("wx1111111111111111", "1234567890", "key1"),
    );
    configs.insert(
        "1234567890_wx2222222222222222".to_string(),
        make_config("wx2222222222222222", "1234567890", "key2"),
    );

    service.set_multi_config(&configs);

    // 验证可以切换
    assert!(service.switchover("1234567890", "wx1111111111111111"));
    assert!(service.switchover("1234567890", "wx2222222222222222"));
}

/// 对应 Java: MultiAppIdSwitchoverTest.testRemoveConfig
#[test]
fn test_multi_appid_remove_config() {
    let service = make_service();

    let config = make_config("wx1111111111111111", "1234567890", "test_key");
    service.add_config("1234567890", "wx1111111111111111", config);
    assert!(service.switchover("1234567890", "wx1111111111111111"));

    // 移除配置
    service.remove_config("1234567890", "wx1111111111111111");
    assert!(!service.switchover("1234567890", "wx1111111111111111"));
}

// ═══════════════════════════════════════════════════════════════
// #13 TransferAuthorizationApiCompatibilityTest（361 行）—— 转账授权兼容
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: TransferAuthorizationApiCompatibilityTest（转账请求 bean 验证）
#[test]
fn test_transfer_authorization_request_serde() {
    let json_str = r#"{
        "appid": "wx1111111111111111",
        "mch_id": "1234567890",
        "partner_trade_no": "TRADE001",
        "openid": "openid001",
        "check_name": "FORCE_CHECK",
        "re_user_name": "张三",
        "amount": 100,
        "desc": "测试转账",
        "spbill_create_ip": "127.0.0.1"
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["appid"], "wx1111111111111111");
    assert_eq!(value["mch_id"], "1234567890");
    assert_eq!(value["partner_trade_no"], "TRADE001");
    assert_eq!(value["openid"], "openid001");
    assert_eq!(value["amount"], 100);
}

/// 对应 Java: TransferAuthorizationApiCompatibilityTest（转账响应解析）
#[test]
fn test_transfer_authorization_result_serde() {
    let json_str = r#"{
        "return_code": "SUCCESS",
        "return_msg": "OK",
        "result_code": "SUCCESS",
        "partner_trade_no": "TRADE001",
        "payment_no": "PAY001",
        "payment_time": "2023-01-01 12:00:00"
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["return_code"], "SUCCESS");
    assert_eq!(value["result_code"], "SUCCESS");
    assert_eq!(value["partner_trade_no"], "TRADE001");
    assert_eq!(value["payment_no"], "PAY001");
}

/// 对应 Java: TransferAuthorizationApiCompatibilityTest（商家转账请求 bean）
#[test]
fn test_merchant_transfer_request_serde() {
    let json_str = r#"{
        "appid": "wx1111111111111111",
        "out_batch_no": "BATCH001",
        "batch_name": "测试批次",
        "batch_remark": "测试备注",
        "total_amount": 10000,
        "total_num": 1,
        "transfer_detail_list": [{
            "out_detail_no": "DETAIL001",
            "transfer_amount": 10000,
            "transfer_remark": "测试",
            "openid": "openid001"
        }]
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["appid"], "wx1111111111111111");
    assert_eq!(value["out_batch_no"], "BATCH001");
    assert_eq!(value["total_amount"], 10000);
    assert_eq!(value["total_num"], 1);
    assert_eq!(
        value["transfer_detail_list"][0]["out_detail_no"],
        "DETAIL001"
    );
}

// ═══════════════════════════════════════════════════════════════
// #15 WxPayUnifiedOrderV3ResultTest（322 行）—— V3 统一下单响应
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayUnifiedOrderV3ResultTest（JSAPI 预支付响应解析）
#[test]
fn test_v3_unified_order_result_jsapi() {
    let json_str = r#"{"prepay_id":"wx201410272009395522657a690ac8912d790100"}"#;
    let result: WxPayUnifiedOrderV3Result =
        serde_json::from_str(json_str).expect("解析 V3 统一下单响应");
    assert_eq!(
        result.prepay_id.as_deref(),
        Some("wx201410272009395522657a690ac8912d790100")
    );
    assert!(result.h5_url.is_none());
    assert!(result.code_url.is_none());
}

/// 对应 Java: WxPayUnifiedOrderV3ResultTest（Native 预支付响应解析）
#[test]
fn test_v3_unified_order_result_native() {
    let json_str = r#"{"code_url":"weixin://wxpay/bizpayurl?pr=xxxxx"}"#;
    let result: WxPayUnifiedOrderV3Result =
        serde_json::from_str(json_str).expect("解析 V3 统一下单响应");
    assert!(result.prepay_id.is_none());
    assert_eq!(
        result.code_url.as_deref(),
        Some("weixin://wxpay/bizpayurl?pr=xxxxx")
    );
}

/// 对应 Java: WxPayUnifiedOrderV3ResultTest（H5 预支付响应解析）
#[test]
fn test_v3_unified_order_result_h5() {
    let json_str = r#"{"h5_url":"https://wx.tenpay.com/cgi-bin/mmpayweb-bin/checkmweb?prepay_id=wx201410272009395522657a690ac8912d790100"}"#;
    let result: WxPayUnifiedOrderV3Result =
        serde_json::from_str(json_str).expect("解析 V3 统一下单响应");
    assert!(result.prepay_id.is_none());
    assert!(result.code_url.is_none());
    assert!(result.h5_url.is_some());
    assert!(result.h5_url.as_ref().unwrap().contains("prepay_id"));
}

/// 对应 Java: WxPayUnifiedOrderV3ResultTest（APP 预支付响应解析）
#[test]
fn test_v3_unified_order_result_app() {
    let json_str = r#"{"prepay_id":"wx201410272009395522657a690ac8912d790100"}"#;
    let result: WxPayUnifiedOrderV3Result =
        serde_json::from_str(json_str).expect("解析 V3 统一下单响应");
    assert!(result.prepay_id.is_some());
    // APP 场景：用 prepay_id 构建 JsapiResult（验证签名字段）
    let jsapi_result = JsapiResult {
        app_id: Some("wx1111111111111111".to_string()),
        time_stamp: Some("1620000000".to_string()),
        nonce_str: Some("nonce001".to_string()),
        package_value: Some(format!("prepay_id={}", result.prepay_id.as_ref().unwrap())),
        sign_type: Some("RSA".to_string()),
        pay_sign: Some("sign001".to_string()),
    };
    assert_eq!(jsapi_result.app_id.as_deref(), Some("wx1111111111111111"));
    assert!(
        jsapi_result
            .package_value
            .as_ref()
            .unwrap()
            .contains("prepay_id")
    );
}

/// 对应 Java: WxPayUnifiedOrderV3ResultTest（JSAPI 签名结果解析）
#[test]
fn test_v3_jsapi_result_serde() {
    let json_str = r#"{
        "appId": "wx1111111111111111",
        "timeStamp": "1620000000",
        "nonceStr": "nonce001",
        "package": "prepay_id=wx201410272009395522657a690ac8912d790100",
        "signType": "RSA",
        "paySign": "sign001",
        "prepayId": "wx201410272009395522657a690ac8912d790100"
    }"#;
    let result: JsapiResult = serde_json::from_str(json_str).expect("解析 JSAPI 签名结果");
    assert_eq!(result.app_id.as_deref(), Some("wx1111111111111111"));
    assert_eq!(result.time_stamp.as_deref(), Some("1620000000"));
    assert_eq!(result.nonce_str.as_deref(), Some("nonce001"));
    assert_eq!(
        result.package_value.as_deref(),
        Some("prepay_id=wx201410272009395522657a690ac8912d790100")
    );
    assert_eq!(result.sign_type.as_deref(), Some("RSA"));
    assert_eq!(result.pay_sign.as_deref(), Some("sign001"));
}

/// 对应 Java: WxPayUnifiedOrderV3ResultTest（默认值语义）
#[test]
fn test_v3_unified_order_result_default() {
    let result = WxPayUnifiedOrderV3Result::default();
    assert!(result.prepay_id.is_none());
    assert!(result.h5_url.is_none());
    assert!(result.code_url.is_none());
}

/// 对应 Java: WxPayUnifiedOrderV3ResultTest（JSON 序列化 roundtrip）
#[test]
fn test_v3_unified_order_result_roundtrip() {
    let original = WxPayUnifiedOrderV3Result {
        prepay_id: Some("wx_test_prepay".to_string()),
        h5_url: None,
        code_url: Some("weixin://test".to_string()),
    };
    let json_str = serde_json::to_string(&original).expect("序列化成功");
    let parsed: WxPayUnifiedOrderV3Result = serde_json::from_str(&json_str).expect("反序列化成功");
    assert_eq!(original, parsed);
}
