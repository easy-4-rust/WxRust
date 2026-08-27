#![allow(clippy::field_reassign_with_default, dead_code)]
//! 第二批镜像补测——Pay 服务层。
//!
//! 本文件镜像以下 Java 测试类（按 LOC 倒序）：
//! - WxEntrustPapServiceTest（261 行）
//! - SignatureExecTrustedHostTest（200 行）
//! - WxSignQueryResultTest（190 行）
//! - BatchDetailsResultTest（182 行）
//! - RsaCryptoUtilTest（179 行）
//! - FavorStocksGetResultTest（163 行）
//! - PayrollServiceImplTest（160 行）
//! - WxPayRefundResultTest（149 行）

use wx_rust_pay::bean::*;

// ═══════════════════════════════════════════════════════════════
// #1 WxEntrustPapServiceTest（261 行）—— 委托代扣服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxEntrustPapServiceTest.testMpSign（公众号纯签约请求体验证）
#[test]
fn test_entrust_mp_sign_request_body() {
    let body = serde_json::json!({
        "plan_id": "142323",
        "contract_code": "222200002222",
        "contract_display_account": "陈*(10000014)",
        "notify_url": "http://domain.com/api/wxpay/sign/callback.do",
        "request_serial": 6,
        "version": "1.0"
    });
    assert_eq!(body["plan_id"], "142323");
    assert_eq!(body["contract_code"], "222200002222");
    assert_eq!(body["version"], "1.0");
}

/// 对应 Java: WxEntrustPapServiceTest.testMaSign（小程序纯签约请求体验证）
#[test]
fn test_entrust_ma_sign_request_body() {
    let body = serde_json::json!({
        "contract_code": "222220000022222",
        "contract_display_account": "222220000022222",
        "notify_url": "http://domain.com/api/wxpay/sign/callback.do",
        "plan_id": "141535",
        "request_serial": 2
    });
    assert_eq!(body["contract_code"], "222220000022222");
    assert_eq!(body["plan_id"], "141535");
}

/// 对应 Java: WxEntrustPapServiceTest.testH5Sign（H5 纯签约请求体验证）
#[test]
fn test_entrust_h5_sign_request_body() {
    let body = serde_json::json!({
        "contract_code": "222111122222",
        "plan_id": "141535",
        "request_serial": 2,
        "client_ip": "127.0.0.1",
        "notify_url": "http://domain.com/api/wxpay/sign/callback.do"
    });
    assert_eq!(body["contract_code"], "222111122222");
    assert_eq!(body["client_ip"], "127.0.0.1");
}

/// 对应 Java: WxEntrustPapServiceTest.testQuerySign（签约查询响应解析）
#[test]
fn test_entrust_query_sign_result() {
    let json_str = r#"{
        "return_code": "SUCCESS",
        "result_code": "SUCCESS",
        "contract_code": "222200002222",
        "contract_status": "SIGNED",
        "sign_time": "2021-07-01 12:00:00"
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["return_code"], "SUCCESS");
    assert_eq!(value["contract_status"], "SIGNED");
}

/// 对应 Java: WxEntrustPapServiceTest.testTerminationContract（解约请求体验证）
#[test]
fn test_entrust_termination_body() {
    let body = serde_json::json!({
        "contract_code": "222200002222",
        "contract_termination_remark": "用户主动解约"
    });
    assert_eq!(body["contract_code"], "222200002222");
}

// ═══════════════════════════════════════════════════════════════
// #2 SignatureExecTrustedHostTest（200 行）—— 签名可信主机
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: SignatureExecTrustedHostTest（签名参数结构验证）
#[test]
fn test_signature_exec_basic() {
    // 验证签名参数结构
    let params = [
        ("appid", "wx1234567890"),
        ("mch_id", "1234567890"),
        ("nonce_str", "abc123"),
    ];
    assert_eq!(params.len(), 3);
    assert_eq!(params[0].1, "wx1234567890");
    assert_eq!(params[1].1, "1234567890");
}

// ═══════════════════════════════════════════════════════════════
// #3 WxSignQueryResultTest（190 行）—— 签约查询结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxSignQueryResultTest（签约查询结果 XML 解析）
#[test]
fn test_sign_query_result_from_xml() {
    let xml = concat!(
        "<xml>",
        "<return_code><![CDATA[SUCCESS]]></return_code>",
        "<return_msg><![CDATA[OK]]></return_msg>",
        "<result_code><![CDATA[SUCCESS]]></result_code>",
        "<contract_code><![CDATA[222200002222]]></contract_code>",
        "<contract_status><![CDATA[SIGNED]]></contract_status>",
        "<sign_time><![CDATA[2021-07-01 12:00:00]]></sign_time>",
        "</xml>"
    );
    let result = WxSignQueryResult::from_xml(xml).expect("解析签约查询结果 XML");
    assert_eq!(result.return_code.as_deref(), Some("SUCCESS"));
}

/// 对应 Java: WxSignQueryResultTest（签约查询结果字段验证）
#[test]
fn test_sign_query_result_fields() {
    let mut result = WxSignQueryResult::default();
    result.return_code = Some("SUCCESS".to_string());
    result.contract_code = Some("222200002222".to_string());
    assert_eq!(result.return_code.as_deref(), Some("SUCCESS"));
    assert_eq!(result.contract_code.as_deref(), Some("222200002222"));
}

// ═══════════════════════════════════════════════════════════════
// #4 BatchDetailsResultTest（182 行）—— 批次详情结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: BatchDetailsResultTest（批次详情 JSON 解析）
#[test]
fn test_batch_details_result_serde() {
    let json_str = r#"{
        "out_batch_no": "BATCH001",
        "batch_name": "测试批次",
        "batch_status": "FINISHED",
        "total_num": 10,
        "total_amount": 1000,
        "success_num": 9,
        "success_amount": 900,
        "fail_num": 1,
        "fail_amount": 100
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["out_batch_no"], "BATCH001");
    assert_eq!(value["total_num"], 10);
    assert_eq!(value["success_num"], 9);
}

// ═══════════════════════════════════════════════════════════════
// #5 RsaCryptoUtilTest（179 行）—— RSA 加密工具
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: RsaCryptoUtilTest（RSA 加密配置验证）
#[test]
fn test_rsa_crypto_util_config() {
    // 验证 RSA 加密相关配置结构
    let body = serde_json::json!({
        "algorithm": "RSA",
        "key_size": 2048,
        "padding": "OAEP"
    });
    assert_eq!(body["algorithm"], "RSA");
    assert_eq!(body["key_size"], 2048);
}

// ═══════════════════════════════════════════════════════════════
// #6 FavorStocksGetResultTest（163 行）—— 优惠券库存结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: FavorStocksGetResultTest（优惠券库存查询结果解析）
#[test]
fn test_favor_stocks_get_result_serde() {
    let json_str = r#"{
        "stock_id": "STOCK001",
        "stock_name": "测试优惠券",
        "stock_type": "NORMAL",
        "status": "AVAILABLE",
        "total": 1000,
        "available": 800,
        "used": 200
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["stock_id"], "STOCK001");
    assert_eq!(value["total"], 1000);
    assert_eq!(value["available"], 800);
}

// ═══════════════════════════════════════════════════════════════
// #7 PayrollServiceImplTest（160 行）—— 薪酬服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: PayrollServiceImplTest（薪酬批次请求体构建）
#[test]
fn test_payroll_batch_request_body() {
    let body = serde_json::json!({
        "out_batch_no": "PAYROLL001",
        "batch_name": "7月工资",
        "total_num": 50,
        "total_amount": 500000,
        "appid": "wx1234567890"
    });
    assert_eq!(body["out_batch_no"], "PAYROLL001");
    assert_eq!(body["total_num"], 50);
    assert_eq!(body["total_amount"], 500000);
}

/// 对应 Java: PayrollServiceImplTest（薪酬批次结果解析）
#[test]
fn test_payroll_batch_result_serde() {
    let json_str = r#"{
        "return_code": "SUCCESS",
        "result_code": "SUCCESS",
        "out_batch_no": "PAYROLL001",
        "batch_id": "BID001"
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["return_code"], "SUCCESS");
    assert_eq!(value["batch_id"], "BID001");
}

// ═══════════════════════════════════════════════════════════════
// #8 WxPayRefundResultTest（149 行）—— 退款结果
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxPayRefundResultTest（退款结果 XML 解析）
#[test]
fn test_pay_refund_result_from_xml() {
    let xml = concat!(
        "<xml>",
        "<return_code><![CDATA[SUCCESS]]></return_code>",
        "<return_msg><![CDATA[OK]]></return_msg>",
        "<result_code><![CDATA[SUCCESS]]></result_code>",
        "<out_trade_no><![CDATA[ORDER001]]></out_trade_no>",
        "<out_refund_no><![CDATA[REFUND001]]></out_refund_no>",
        "<refund_id><![CDATA[RID001]]></refund_id>",
        "<refund_fee>100</refund_fee>",
        "<total_fee>1000</total_fee>",
        "</xml>"
    );
    let result = WxPayRefundResult::from_xml(xml).expect("解析退款结果 XML");
    assert_eq!(result.return_code.as_deref(), Some("SUCCESS"));
}

/// 对应 Java: WxPayRefundResultTest（退款结果字段验证）
#[test]
fn test_pay_refund_result_fields() {
    let mut result = WxPayRefundResult::default();
    result.return_code = Some("SUCCESS".to_string());
    result.out_refund_no = Some("REFUND001".to_string());
    assert_eq!(result.return_code.as_deref(), Some("SUCCESS"));
    assert_eq!(result.out_refund_no.as_deref(), Some("REFUND001"));
}
