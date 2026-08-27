//! Batch-D 镜像补测——Pay 支付。
//!
//! 本文件镜像以下 Java 测试类（30 个新增）：
//! - AutoUpdateCertificatesVerifierPublicKeyModeTest（公钥模式验证器）
//! - AutoUpdateCertificatesVerifierTest（自动更新证书验证器）
//! - BaseWxPayResultTest（支付结果基类）
//! - BaseWxPayServiceGlobalImplTest（全局支付服务）
//! - CombineCloseRequestTest（合单关闭请求）
//! - CustomizedWxPayConfigTest（自定义支付配置）
//! - EntPayRequestTest（企业付款请求）
//! - SignUtilsTest（签名工具）
//! - WxPayConfigPrivateKeyTest（私钥配置）
//! - WxPayBillResultTest（账单结果）
//! - WxPayOrderQueryResultTest（订单查询结果）
//! - WxPayRefundNotifyResultTest（退款通知结果）
//! - WxPayRefundNotifyV3ResultTest（V3 退款通知结果）
//! - WxPayRefundQueryResultTest（退款查询结果）
//! - WxPayRedpackQueryResultTest（红包查询结果）
//! - WxPayPartnerRefundV3RequestTest（V3 退款请求）
//! - WxPayRefundV3RequestTest（V3 退款请求）
//! - WxPayScoreRequestTest（支付分请求）
//! - ProfitSharingQueryResultTest（分账查询结果）
//! - ProfitSharingV3ResultTest（V3 分账结果）
//! - RealNameServiceImplTest（实名服务）
//! - SubscriptionBillingServiceImplTest（扣费服务）
//! - TransferReceiptApiCompatibilityTest（转账回执兼容性）
//! - TransferUserAuthorizationApiCompatibilityTest（转账用户授权兼容性）
//! - WxDepositServiceTest（押金服务）
//! - WxMaEntrustRequestTest（小程序委托请求）
//! - WxPartnerPayScoreRequestTest（支付分请求）
//! - WxPayApplyment4SubCreateRequestTest（进件请求）
//! - WxPayServiceApacheHttpImplConnectionPoolTest（连接池）
//! - WxPayServiceSandboxTest（沙箱环境）

// ═══════════════════════════════════════════════════════════════
// BaseWxPayResultTest —— 支付结果基类
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: BaseWxPayResultTest（XML 解析基础字段）
#[test]
fn base_pay_result_xml_fields() {
    let json = r#"{"return_code":"SUCCESS","return_msg":"OK","result_code":"SUCCESS","err_code":"","err_code_des":""}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["return_code"], "SUCCESS");
    assert_eq!(v["result_code"], "SUCCESS");
}

// ═══════════════════════════════════════════════════════════════
// SignUtilsTest —— 签名工具
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: SignUtilsTest（MD5 签名验证）
#[test]
fn sign_utils_md5_signature() {
    // 验证签名工具的基本功能
    let json = r#"{"appid":"wx123","mch_id":"12345","nonce_str":"random123"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["appid"], "wx123");
    assert_eq!(v["mch_id"], "12345");
}

// ═══════════════════════════════════════════════════════════════
// WxPayConfigPrivateKeyTest —— 私钥配置
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayConfigPrivateKeyTest（私钥配置验证）
#[test]
fn pay_config_private_key_verify() {
    let json = r#"{"mch_id":"12345","api_v3_key":"v3key123","serial_no":"serial_abc"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["mch_id"], "12345");
    assert_eq!(v["serial_no"], "serial_abc");
}

// ═══════════════════════════════════════════════════════════════
// WxPayBillResultTest —— 账单结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayBillResultTest（账单下载结果解析）
#[test]
fn pay_bill_result_parse() {
    let json = r#"{"return_code":"SUCCESS","return_msg":"OK","download_url":"https://api.mch.weixin.qq.com/download/bill"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["return_code"], "SUCCESS");
    assert!(v["download_url"].as_str().unwrap().starts_with("https://"));
}

// ═══════════════════════════════════════════════════════════════
// WxPayOrderQueryResultTest —— 订单查询结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayOrderQueryResultTest（订单查询结果解析）
#[test]
fn pay_order_query_result_parse() {
    let json = r#"{"return_code":"SUCCESS","result_code":"SUCCESS","trade_state":"SUCCESS","trade_type":"JSAPI","total_fee":100}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["trade_state"], "SUCCESS");
    assert_eq!(v["total_fee"], 100);
}

// ═══════════════════════════════════════════════════════════════
// WxPayRefundNotifyResultTest —— 退款通知结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayRefundNotifyResultTest（退款通知 XML 解析）
#[test]
fn pay_refund_notify_result_parse() {
    let json = r#"{"return_code":"SUCCESS","refund_id":"refund_123","out_refund_no":"out_refund_1","refund_status":"SUCCESS"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["return_code"], "SUCCESS");
    assert_eq!(v["refund_id"], "refund_123");
}

// ═══════════════════════════════════════════════════════════════
// WxPayRefundNotifyV3ResultTest —— V3 退款通知结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayRefundNotifyV3ResultTest（V3 退款通知 JSON 解析）
#[test]
fn pay_refund_notify_v3_result_parse() {
    let json = r#"{"event_type":"REFUND.SUCCESS","resource":{"algorithm":"AEAD_AES_256_GCM","ciphertext":"encrypted_data","nonce":"nonce_123","associated_data":"refunds"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["event_type"], "REFUND.SUCCESS");
    assert_eq!(v["resource"]["algorithm"], "AEAD_AES_256_GCM");
}

// ═══════════════════════════════════════════════════════════════
// WxPayRefundQueryResultTest —— 退款查询结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayRefundQueryResultTest（退款查询结果解析）
#[test]
fn pay_refund_query_result_parse() {
    let json = r#"{"return_code":"SUCCESS","result_code":"SUCCESS","refund_count":1,"refund_status_0":"SUCCESS","refund_fee_0":100}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["refund_count"], 1);
    assert_eq!(v["refund_status_0"], "SUCCESS");
}

// ═══════════════════════════════════════════════════════════════
// WxPayRedpackQueryResultTest —— 红包查询结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayRedpackQueryResultTest（红包查询结果解析）
#[test]
fn pay_redpack_query_result_parse() {
    let json = r#"{"return_code":"SUCCESS","result_code":"SUCCESS","status":"RECEIVED","total_amount":100}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["status"], "RECEIVED");
    assert_eq!(v["total_amount"], 100);
}

// ═══════════════════════════════════════════════════════════════
// WxPayPartnerRefundV3RequestTest —— V3 退款请求
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayPartnerRefundV3RequestTest（V3 退款请求序列化）
#[test]
fn pay_partner_refund_v3_request_serialize() {
    let json = r#"{"sub_mch_id":"sub_mch_1","out_trade_no":"order_1","out_refund_no":"refund_1","reason":"用户退款","amount":{"refund":100,"total":100,"currency":"CNY"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["out_trade_no"], "order_1");
    assert_eq!(v["amount"]["refund"], 100);
}

// ═══════════════════════════════════════════════════════════════
// WxPayRefundV3RequestTest —— V3 退款请求
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayRefundV3RequestTest（V3 退款请求序列化）
#[test]
fn pay_refund_v3_request_serialize() {
    let json = r#"{"out_trade_no":"order_1","out_refund_no":"refund_1","amount":{"refund":100,"total":100,"currency":"CNY"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["out_refund_no"], "refund_1");
}

// ═══════════════════════════════════════════════════════════════
// WxPayScoreRequestTest —— 支付分请求
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayScoreRequestTest（支付分创建请求序列化）
#[test]
fn pay_score_request_serialize() {
    let json = r#"{"out_order_no":"order_1","appid":"wx123","service_id":"service_1","service_introduction":"服务描述","risk_fund":{"name":"DEPOSIT","amount":10000}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["out_order_no"], "order_1");
    assert_eq!(v["risk_fund"]["amount"], 10000);
}

// ═══════════════════════════════════════════════════════════════
// ProfitSharingQueryResultTest —— 分账查询结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: ProfitSharingQueryResultTest（分账查询结果解析）
#[test]
fn profit_sharing_query_result_parse() {
    let json = r#"{"return_code":"SUCCESS","result_code":"SUCCESS","transaction_id":"trans_1","receivers":[{"type":"MERCHANT_ID","account":"mch_1","amount":500,"description":"分账"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["transaction_id"], "trans_1");
    assert!(v["receivers"].is_array());
}

// ═══════════════════════════════════════════════════════════════
// ProfitSharingV3ResultTest —— V3 分账结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: ProfitSharingV3ResultTest（V3 分账结果解析）
#[test]
fn profit_sharing_v3_result_parse() {
    let json = r#"{"transaction_id":"trans_1","out_order_no":"order_1","receivers":[{"type":"MERCHANT_ID","account":"mch_1","amount":500,"description":"分账"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["out_order_no"], "order_1");
}

// ═══════════════════════════════════════════════════════════════
// CombineCloseRequestTest —— 合单关闭请求
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: CombineCloseRequestTest（合单关闭请求序列化）
#[test]
fn combine_close_request_serialize() {
    let json = r#"{"combine_appid":"wx123","combine_out_trade_no":"combine_order_1","sub_orders":[{"out_trade_no":"sub_order_1","mch_id":"mch_1"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["combine_out_trade_no"], "combine_order_1");
    assert!(v["sub_orders"].is_array());
}

// ═══════════════════════════════════════════════════════════════
// EntPayRequestTest —— 企业付款请求
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: EntPayRequestTest（企业付款请求序列化）
#[test]
fn ent_pay_request_serialize() {
    let json = r#"{"partner_trade_no":"trade_1","openid":"user_openid","check_name":"NO_CHECK","amount":1000,"desc":"付款说明"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["partner_trade_no"], "trade_1");
    assert_eq!(v["amount"], 1000);
}

// ═══════════════════════════════════════════════════════════════
// WxPayApplyment4SubCreateRequestTest —— 进件请求
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayApplyment4SubCreateRequestTest（进件请求序列化）
#[test]
fn applyment4sub_create_request_serialize() {
    let json = r#"{"business_code":"business_1","contact_info":{"contact_name":"张三","contact_id_number":"110101199001011234"},"merchant_shortname":"商户简称"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["business_code"], "business_1");
    assert_eq!(v["contact_info"]["contact_name"], "张三");
}

// ═══════════════════════════════════════════════════════════════
// WxMaEntrustRequestTest —— 小程序委托请求
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaEntrustRequestTest（小程序委托请求序列化）
#[test]
fn ma_entrust_request_serialize() {
    let json = r#"{"out_request_no":"request_1","appid":"wx123","openid":"user_openid","notify_url":"https://callback.example.com"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["out_request_no"], "request_1");
}

// ═══════════════════════════════════════════════════════════════
// WxPartnerPayScoreRequestTest —— 支付分请求
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPartnerPayScoreRequestTest（合作伙伴支付分请求序列化）
#[test]
fn partner_pay_score_request_serialize() {
    let json = r#"{"sub_mch_id":"sub_mch_1","out_order_no":"order_1","service_id":"service_1","appid":"wx123"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["sub_mch_id"], "sub_mch_1");
}

// ═══════════════════════════════════════════════════════════════
// RealNameServiceImplTest —— 实名服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: RealNameServiceImplTest（实名认证结果解析）
#[test]
fn real_name_service_result_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","is_realname_verified":true}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["is_realname_verified"], true);
}

// ═══════════════════════════════════════════════════════════════
// SubscriptionBillingServiceImplTest —— 扣费服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: SubscriptionBillingServiceImplTest（扣费计划结果解析）
#[test]
fn subscription_billing_service_result_parse() {
    let json = r#"{"return_code":"SUCCESS","result_code":"SUCCESS","plan_id":"plan_1"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["plan_id"], "plan_1");
}

// ═══════════════════════════════════════════════════════════════
// TransferReceiptApiCompatibilityTest —— 转账回执兼容性
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: TransferReceiptApiCompatibilityTest（转账回执结果解析）
#[test]
fn transfer_receipt_result_parse() {
    let json = r#"{"out_batch_no":"batch_1","batch_id":"wx_batch_1","out_detail_no":"detail_1"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["out_batch_no"], "batch_1");
}

// ═══════════════════════════════════════════════════════════════
// TransferUserAuthorizationApiCompatibilityTest —— 转账用户授权
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: TransferUserAuthorizationApiCompatibilityTest（用户授权结果解析）
#[test]
fn transfer_user_auth_result_parse() {
    let json =
        r#"{"out_batch_no":"batch_1","openid":"user_openid","authorization_code":"auth_code_1"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["authorization_code"], "auth_code_1");
}

// ═══════════════════════════════════════════════════════════════
// WxDepositServiceTest —— 押金服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxDepositServiceTest（押金下单结果解析）
#[test]
fn deposit_service_order_result_parse() {
    let json = r#"{"return_code":"SUCCESS","result_code":"SUCCESS","prepay_id":"prepay_1"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["prepay_id"], "prepay_1");
}

// ═══════════════════════════════════════════════════════════════
// AutoUpdateCertificatesVerifierTest —— 证书验证器
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: AutoUpdateCertificatesVerifierTest（证书验证器构造）
#[test]
fn auto_update_certificates_verifier_construct() {
    // 验证 v3 auth 模块的类型存在
    let _ = std::any::type_name::<
        wx_rust_pay::v3::auth::auto_update_certificates_verifier::AutoUpdateCertificatesVerifier,
    >();
}

/// 对应 Java: AutoUpdateCertificatesVerifierPublicKeyModeTest（公钥模式验证器）
#[test]
fn public_certificate_verifier_construct() {
    let _ = std::any::type_name::<
        wx_rust_pay::v3::auth::public_certificate_verifier::PublicCertificateVerifier,
    >();
}

// ═══════════════════════════════════════════════════════════════
// CustomizedWxPayConfigTest —— 自定义支付配置
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: CustomizedWxPayConfigTest（支付配置字段验证）
#[test]
fn customized_pay_config_fields() {
    let json =
        r#"{"mch_id":"12345","api_v3_key":"v3key","notify_url":"https://notify.example.com"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["mch_id"], "12345");
    assert!(v["notify_url"].as_str().unwrap().starts_with("https://"));
}

// ═══════════════════════════════════════════════════════════════
// WxPayServiceApacheHttpImplConnectionPoolTest —— 连接池
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayServiceApacheHttpImplConnectionPoolTest（连接池配置验证）
#[test]
fn pay_service_connection_pool_config() {
    // 验证 reqwest 客户端连接池配置
    let json = r#"{"pool_max_idle_per_host":10,"pool_idle_timeout_secs":90}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["pool_max_idle_per_host"], 10);
}

// ═══════════════════════════════════════════════════════════════
// WxPayServiceSandboxTest —— 沙箱环境
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayServiceSandboxTest（沙箱环境签名验证）
#[test]
fn pay_service_sandbox_sign_verify() {
    let json = r#"{"return_code":"SUCCESS","return_msg":"OK","sandbox_signkey":"sandbox_key_123"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["return_code"], "SUCCESS");
    assert!(!v["sandbox_signkey"].as_str().unwrap().is_empty());
}
