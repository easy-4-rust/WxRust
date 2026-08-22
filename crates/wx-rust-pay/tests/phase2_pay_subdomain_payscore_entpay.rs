#![allow(clippy::field_reassign_with_default)]
//! Phase 2 补齐: PayScore / EntPay（企业付款）子域 Bean 序列化测试。
//!
//! 镜像 Java:
//! - `PayScoreServiceImplTest`（支付分请求/结果/通知解析）
//! - `EntPayServiceImplTest`（企业付款/企业付款到银行/查询/红包）
//! - `WxPayException` 异常映射（check_result 语义）
//! - 对账单解析边界（bill result 空行/汇总段解析）
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java @Test 的序列化/反序列化断言
//! - RUST_OBLIGATION: serde skip 语义、XML roundtrip
//! - VALUE_ADD: 空值/边界/异常映射路径

use wx_rust_pay::bean::entpay::*;
use wx_rust_pay::bean::payscore::*;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;
use wx_rust_pay::util::wx_pay_service_impl_utils::check_result;

// ═══ WxPayScoreRequest（SOURCE_PARITY:
//     Java PayScoreServiceImplTest.testCreateScoreService）═══

/// 支付分创建订单请求 serde（对应 Java `WxPayScoreRequest`：`out_order_no`/
/// `service_id`/`service_introduction`/`time_range`/`location`/`risk_fund`/
/// `openid`/`need_user_confirm`）。
/// 对应 Java: PayScoreServiceImplTest.testCreateScoreService
#[test]
fn test_pay_score_request_serde() {
    let json = r#"{
        "out_order_no":"SCORE-001",
        "appid":"wx1234",
        "service_id":"500001",
        "service_introduction":"借用服务",
        "risk_fund":{"name":"DEPOSIT","amount":10000},
        "time_range":{"start_time":"2024-01-01T00:00:00+08:00"},
        "openid":"ox123",
        "notify_url":"https://example.com/notify",
        "need_user_confirm":true,
        "attach":"自定义数据"
    }"#;
    let request: WxPayScoreRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.out_order_no.as_deref(), Some("SCORE-001"));
    assert_eq!(request.service_id.as_deref(), Some("500001"));
    assert_eq!(request.openid.as_deref(), Some("ox123"));
    assert_eq!(request.need_user_confirm, Some(true));
    assert_eq!(request.attach.as_deref(), Some("自定义数据"));
    let risk = request.risk_fund.as_ref().unwrap();
    assert_eq!(risk.amount, 10000);
}

// ═══ WxPayScoreResult（SOURCE_PARITY:
//     Java PayScoreServiceImplTest (response)）═══

/// 支付分结果 serde（对应 Java `WxPayScoreResult`）。
/// 对应 Java: PayScoreServiceImplTest (result)
#[test]
fn test_pay_score_result_serde() {
    let json = r#"{
        "appid":"wx1234",
        "mchid":"mch123",
        "out_order_no":"SCORE-001",
        "service_id":"500001",
        "service_introduction":"借用服务",
        "state":"CREATED",
        "state_description":"订单已创建",
        "risk_fund":{"name":"DEPOSIT","amount":10000},
        "openid":"ox123"
    }"#;
    let result: WxPayScoreResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.out_order_no.as_deref(), Some("SCORE-001"));
    assert_eq!(result.state.as_deref(), Some("CREATED"));
    assert_eq!(result.state_description.as_deref(), Some("订单已创建"));
}

// ═══ RiskFund ═══

#[test]
fn test_risk_fund_serde() {
    let json = r#"{"name":"DEPOSIT","amount":10000,"description":"押金"}"#;
    let fund: RiskFund = serde_json::from_str(json).unwrap();
    assert_eq!(fund.name.as_deref(), Some("DEPOSIT"));
    assert_eq!(fund.amount, 10000);
}

// ═══ TimeRange ═══

#[test]
fn test_time_range_serde() {
    let json =
        r#"{"start_time":"2024-01-01T00:00:00+08:00","end_time":"2024-01-02T00:00:00+08:00"}"#;
    let range: TimeRange = serde_json::from_str(json).unwrap();
    assert_eq!(
        range.start_time.as_deref(),
        Some("2024-01-01T00:00:00+08:00")
    );
}

// ═══ Location ═══

/// Location serde（对应 Java `Location`：`start_location`/`end_location` 为字符串字段）。
#[test]
fn test_location_serde() {
    let json = r#"{"start_location":"上海市浦东新区","end_location":"北京市朝阳区"}"#;
    let location: Location = serde_json::from_str(json).unwrap();
    assert_eq!(location.start_location.as_deref(), Some("上海市浦东新区"));
    assert_eq!(location.end_location.as_deref(), Some("北京市朝阳区"));
}

// ═══ PostPayment ═══

#[test]
fn test_post_payment_serde() {
    let json = r#"{"name":"DEPOSIT","amount":10000,"description":"押金扣除","count":1}"#;
    let payment: PostPayment = serde_json::from_str(json).unwrap();
    assert_eq!(payment.amount, Some(10000));
}

// ═══ PayScoreNotifyData ═══

/// 支付分通知数据 serde（对应 Java `PayScoreNotifyData`）。
#[test]
fn test_pay_score_notify_data_serde() {
    let json = r#"{
        "id":"ev-001",
        "create_time":"2024-01-01",
        "resource_type":"encrypt-resource",
        "event_type":"PAYSCORE.USER_PAID",
        "resource":{"algorithm":"AEAD_AES_256_GCM","ciphertext":"encrypted","nonce":"nonce123"}
    }"#;
    let data: PayScoreNotifyData = serde_json::from_str(json).unwrap();
    assert_eq!(data.event_type.as_deref(), Some("PAYSCORE.USER_PAID"));
    assert!(data.resource.is_some());
}

// ═══ WxPartnerPayScoreRequest（SOURCE_PARITY:
//     Java PayScoreServiceImplTest (partner)）═══

/// 服务商支付分请求 serde。
#[test]
fn test_partner_pay_score_request_serde() {
    let json = r#"{
        "out_order_no":"PSCORE-001",
        "service_id":"500001",
        "appid":"wx12334",
        "sub_mchid":"sub123"
    }"#;
    let request: WxPartnerPayScoreRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.out_order_no.as_deref(), Some("PSCORE-001"));
    assert_eq!(request.sub_mchid.as_deref(), Some("sub123"));
}

/// 服务商支付分结果 serde。
#[test]
fn test_partner_pay_score_result_serde() {
    let json = r#"{
        "out_order_no":"PSCORE-001",
        "service_id":"500001",
        "state":"DONE"
    }"#;
    let result: WxPartnerPayScoreResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.state.as_deref(), Some("DONE"));
}

// ═══ EntPayRequest（SOURCE_PARITY:
//     Java EntPayServiceImplTest.testSendEnterprisePay）═══

/// 企业付款请求 serde（对应 Java `EntPayRequest` XML bean：`partner_trade_no`/
/// `openid`/`check_name`/`amount`/`desc`（rename from `description`））。
/// 对应 Java: EntPayServiceImplTest.testSendEnterprisePay
#[test]
fn test_ent_pay_request_serde() {
    let json = r#"{
        "mch_appid":"wx1234",
        "mch_id":"mch123",
        "nonce_str":"nonce123",
        "partner_trade_no":"PAY-001",
        "openid":"ox123",
        "check_name":"FORCE_CHECK",
        "re_user_name":"张三",
        "amount":100,
        "desc":"付款测试",
        "spbill_create_ip":"127.0.0.1"
    }"#;
    let request: EntPayRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.partner_trade_no.as_deref(), Some("PAY-001"));
    assert_eq!(request.openid.as_deref(), Some("ox123"));
    assert_eq!(request.check_name.as_deref(), Some("FORCE_CHECK"));
    assert_eq!(request.amount, Some(100));
    assert_eq!(request.description.as_deref(), Some("付款测试"));
}

// ═══ EntPayResult（SOURCE_PARITY: Java EntPayServiceImplTest (response)）═══

/// 企业付款结果 serde。
#[test]
fn test_ent_pay_result_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "result_code":"SUCCESS",
        "partner_trade_no":"PAY-001",
        "payment_no":"1000000001",
        "payment_time":"2024-01-01 12:00:00"
    }"#;
    let result: EntPayResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.partner_trade_no.as_deref(), Some("PAY-001"));
    assert_eq!(result.payment_no.as_deref(), Some("1000000001"));
}

// ═══ EntPayQueryRequest/Result（SOURCE_PARITY:
//     Java EntPayServiceImplTest.testQueryEnterprisePay）═══

/// 企业付款查询请求。
/// 对应 Java: EntPayServiceImplTest.testQueryEnterprisePay
#[test]
fn test_ent_pay_query_request_serde() {
    let json = r#"{
        "partner_trade_no":"PAY-001",
        "nonce_str":"nonce123"
    }"#;
    let request: EntPayQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.partner_trade_no.as_deref(), Some("PAY-001"));
}

/// 企业付款查询结果。
#[test]
fn test_ent_pay_query_result_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "result_code":"SUCCESS",
        "partner_trade_no":"PAY-001",
        "detail_id":"1000000001",
        "status":"SUCCESS",
        "reason":""
    }"#;
    let result: EntPayQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.status.as_deref(), Some("SUCCESS"));
    assert_eq!(result.detail_id.as_deref(), Some("1000000001"));
}

// ═══ EntPayBankRequest/Result（SOURCE_PARITY:
//     Java EntPayServiceImplTest.testPayToBank）═══

/// 企业付款到银行请求 serde（对应 Java `EntPayBankRequest`：
/// `enc_bank_no`/`enc_true_name`/`bank_code`/`amount`/`description`）。
/// 对应 Java: EntPayServiceImplTest.testPayToBank
#[test]
fn test_ent_pay_bank_request_serde() {
    let json = r#"{
        "mch_id":"mch123",
        "partner_trade_no":"BANK-001",
        "enc_bank_no":"encrypted_bank_no",
        "enc_true_name":"encrypted_name",
        "bank_code":"1001",
        "amount":10000,
        "description":"银行付款"
    }"#;
    let request: EntPayBankRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.partner_trade_no.as_deref(), Some("BANK-001"));
    assert_eq!(request.bank_code.as_deref(), Some("1001"));
    assert_eq!(request.amount, Some(10000));
}

/// 企业付款到银行结果。
#[test]
fn test_ent_pay_bank_result_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "result_code":"SUCCESS",
        "partner_trade_no":"BANK-001",
        "payment_no":"1000000001",
        "cmms_amt":1
    }"#;
    let result: EntPayBankResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.partner_trade_no.as_deref(), Some("BANK-001"));
    assert_eq!(result.cmms_amount, Some(1));
}

// ═══ EntPayBankQueryRequest/Result ═══

/// 对应 Java: EntPayServiceImplTest.testQueryBank
#[test]
fn test_ent_pay_bank_query_request_serde() {
    let json = r#"{"partner_trade_no":"BANK-001","nonce_str":"nonce123"}"#;
    let request: EntPayBankQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.partner_trade_no.as_deref(), Some("BANK-001"));
}

#[test]
fn test_ent_pay_bank_query_result_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "result_code":"SUCCESS",
        "partner_trade_no":"BANK-001",
        "status":"SUCCESS",
        "reason":""
    }"#;
    let result: EntPayBankQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.status.as_deref(), Some("SUCCESS"));
}

// ═══ EntPayRedpackRequest/Result（SOURCE_PARITY:
//     Java RedpackServiceImplTest.testSendRedpack）═══

/// 企业红包请求 serde（对应 Java `EntPayRedpackRequest`：`mch_billno`/
/// `sender_name`/`re_openid`/`total_amount`/`wishing`/`act_name`/`remark`）。
/// 对应 Java: EntPayServiceImplTest.testSendRedpack
#[test]
fn test_ent_pay_redpack_request_serde() {
    let json = r#"{
        "mch_billno":"REDPACK-001",
        "sender_name":"测试商户",
        "re_openid":"ox123",
        "total_amount":100,
        "wishing":"恭喜发财",
        "act_name":"测试活动",
        "remark":"备注"
    }"#;
    let request: EntPayRedpackRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.mch_bill_no.as_deref(), Some("REDPACK-001"));
    assert_eq!(request.total_amount, Some(100));
    assert_eq!(request.wishing.as_deref(), Some("恭喜发财"));
    assert_eq!(request.sender_name.as_deref(), Some("测试商户"));
}

/// 企业红包结果 serde。
#[test]
fn test_ent_pay_redpack_result_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "result_code":"SUCCESS",
        "mch_billno":"REDPACK-001",
        "send_listid":"1000000001"
    }"#;
    let result: EntPayRedpackResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.mch_bill_no.as_deref(), Some("REDPACK-001"));
}

// ═══ EntPayRedpackQueryRequest/Result（SOURCE_PARITY:
//     Java RedpackServiceImplTest.testQueryRedpack）═══

/// 对应 Java: RedpackServiceImplTest.testQueryRedpack
#[test]
fn test_ent_pay_redpack_query_request_serde() {
    let json = r#"{"mch_billno":"REDPACK-001","nonce_str":"nonce123"}"#;
    let request: EntPayRedpackQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.mch_bill_no.as_deref(), Some("REDPACK-001"));
}

#[test]
fn test_ent_pay_redpack_query_result_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "result_code":"SUCCESS",
        "mch_billno":"REDPACK-001",
        "total_amount":100,
        "send_time":"2024-01-01 12:00:00",
        "status":"RECEIVED",
        "send_type":"API"
    }"#;
    let result: EntPayRedpackQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.status.as_deref(), Some("RECEIVED"));
    assert_eq!(result.total_amount, Some(100));
}

// ═══ GetPublicKeyResult ═══

#[test]
fn test_get_public_key_result_serde() {
    let json = r#"{"return_code":"SUCCESS","result_code":"SUCCESS","pub_key":"MIIBIjANBg..."}"#;
    let result: GetPublicKeyResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.return_code.as_deref(), Some("SUCCESS"));
    assert_eq!(result.pub_key.as_deref(), Some("MIIBIjANBg..."));
}

// ═══ WxPayException 异常映射（SOURCE_PARITY:
//     Java WxPayException.Builder.buildErrorMsg）═══

/// v2 响应校验异常映射（对应 Java `BaseWxPayResult.checkResult` +
/// `WxPayException.from` 的错误文案拼接）。
/// 对应 Java: BaseWxPayServiceImplTest (error mapping)
#[test]
fn test_check_result_error_mapping() {
    let mut config = WxPayDefaultConfig::new();
    config.set_mch_key("192006250b4c09247ec02edce69f6a2d");

    // 1. return_code=FAIL → 错误文案含"返回代码"+"返回信息"
    let xml = r#"<xml>
        <return_code><![CDATA[FAIL]]></return_code>
        <return_msg><![CDATA[签名错误]]></return_msg>
    </xml>"#;
    let err = check_result(&config, xml, None, true).expect_err("应报错");
    let msg = err.to_string();
    assert!(msg.contains("返回代码"), "错误信息: {msg}");
    assert!(msg.contains("FAIL"), "错误信息: {msg}");
    assert!(msg.contains("签名错误"), "错误信息: {msg}");

    // 2. result_code=FAIL → 错误文案含"结果代码"+"错误代码"+"错误详情"
    let xml = r#"<xml>
        <return_code><![CDATA[SUCCESS]]></return_code>
        <result_code><![CDATA[FAIL]]></result_code>
        <err_code><![CDATA[ORDERNOTEXIST]]></err_code>
        <err_code_des><![CDATA[订单不存在]]></err_code_des>
    </xml>"#;
    let err = check_result(&config, xml, None, true).expect_err("应报错");
    let msg = err.to_string();
    assert!(msg.contains("结果代码"), "错误信息: {msg}");
    assert!(msg.contains("ORDERNOTEXIST"), "错误信息: {msg}");
    assert!(msg.contains("订单不存在"), "错误信息: {msg}");

    // 3. 签名不匹配 → "参数格式校验错误！"
    let xml = r#"<xml>
        <return_code><![CDATA[SUCCESS]]></return_code>
        <result_code><![CDATA[SUCCESS]]></result_code>
        <sign><![CDATA[BADSIGN123]]></sign>
    </xml>"#;
    let err = check_result(&config, xml, None, true).expect_err("应报错");
    assert!(
        err.to_string().contains("参数格式校验错误"),
        "错误信息: {}",
        err
    );

    // 4. check_success=false → 不检查 return_code/result_code
    let xml = r#"<xml>
        <return_code><![CDATA[FAIL]]></return_code>
        <return_msg><![CDATA[测试]]></return_msg>
    </xml>"#;
    check_result(&config, xml, None, false).expect("check_success=false 不应报错");

    // 5. 无 sign 字段 → 跳过验签
    let xml = r#"<xml>
        <return_code><![CDATA[SUCCESS]]></return_code>
        <result_code><![CDATA[SUCCESS]]></result_code>
    </xml>"#;
    check_result(&config, xml, None, true).expect("无 sign 不应报错");
}

// ═══ VALUE_ADD: 空值/边界 ═══

#[test]
fn test_ent_pay_request_empty() {
    let request = EntPayRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("partner_trade_no"));
    assert!(!json.contains("openid"));
}

#[test]
fn test_pay_score_request_default() {
    let request = WxPayScoreRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("out_order_no"));
}

#[test]
fn test_risk_fund_default() {
    let fund = RiskFund::default();
    assert!(fund.name.is_none());
    assert_eq!(fund.amount, 0);
}
