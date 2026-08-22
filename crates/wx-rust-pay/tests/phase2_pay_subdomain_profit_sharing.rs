#![allow(clippy::field_reassign_with_default)]
//! Phase 2 补齐: ProfitSharing（分账）子域 Bean 序列化与 XML 往返测试。
//!
//! 镜像 Java:
//! - `ProfitSharingServiceImplTest`（分账请求/结果/查询/添加接收方/删除接收方）
//! - v2 XML 往返：对应 Java XStream 序列化/反序列化
//! - v3 JSON serde：对应 Java Gson 序列化
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java @Test 的序列化/反序列化断言
//! - RUST_OBLIGATION: XML/JSON 往返、serde skip 语义、Receiver 列表
//! - VALUE_ADD: 空值/边界/默认值路径

use wx_rust_pay::bean::profitsharing::*;

// ═══ ProfitSharingRequest v2 XML（SOURCE_PARITY:
//     Java ProfitSharingServiceImplTest.testProfitSharing）═══

/// 分账请求 v2 JSON serde（对应 Java `ProfitSharingRequest` 字段
/// `transaction_id`/`out_order_no`/`receivers` JSON 字符串）。
/// 对应 Java: ProfitSharingServiceImplTest.testProfitSharing
#[test]
fn test_profit_sharing_request_v2_serde() {
    let json = r#"{
        "appid":"wx1234",
        "mch_id":"mch123",
        "nonce_str":"nonce123",
        "sign":"sign123",
        "transaction_id":"4200001234",
        "out_order_no":"ORDER-001",
        "receivers":"[{\"type\":\"MERCHANT_ID\",\"account\":\"1900000109\",\"amount\":100,\"description\":\"分账\"}]"
    }"#;
    let request: ProfitSharingRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.appid.as_deref(), Some("wx1234"));
    assert_eq!(request.transaction_id.as_deref(), Some("4200001234"));
    assert_eq!(request.out_order_no.as_deref(), Some("ORDER-001"));
    assert!(request.receivers.as_ref().unwrap().contains("MERCHANT_ID"));
}

/// 分账请求 XML roundtrip（对应 Java `toXML` + `fromXML`）。
/// 对应 Java: ProfitSharingServiceImplTest (XML serialization)
#[test]
fn test_profit_sharing_request_xml_roundtrip() {
    let mut request = ProfitSharingRequest::default();
    request.appid = Some("wx1234".to_string());
    request.mch_id = Some("mch123".to_string());
    request.transaction_id = Some("4200001234".to_string());
    request.out_order_no = Some("ORDER-001".to_string());
    let xml = request.to_xml().expect("XML 序列化失败");
    assert!(xml.contains("<appid>wx1234</appid>"), "xml={xml}");
    assert!(
        xml.contains("<transaction_id>4200001234</transaction_id>"),
        "xml={xml}"
    );
    let back = ProfitSharingRequest::from_xml(&xml).expect("XML 反序列化失败");
    assert_eq!(back.appid.as_deref(), Some("wx1234"));
    assert_eq!(back.transaction_id.as_deref(), Some("4200001234"));
}

// ═══ ProfitSharingResult v2 XML（SOURCE_PARITY:
//     Java ProfitSharingServiceImplTest (response parse)）═══

/// 分账结果 v2 JSON serde（对应 Java `ProfitSharingResult`）。
/// 对应 Java: ProfitSharingServiceImplTest (result parsing)
#[test]
fn test_profit_sharing_result_v2_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "result_code":"SUCCESS",
        "appid":"wx1234",
        "mch_id":"mch123",
        "transaction_id":"4200001234",
        "out_order_no":"ORDER-001",
        "order_id":"1000000001",
        "status":"FINISHED",
        "receivers":"[{\"type\":\"MERCHANT_ID\",\"account\":\"1900000109\",\"amount\":100,\"result\":\"SUCCESS\"}]"
    }"#;
    let result: ProfitSharingResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.return_code.as_deref(), Some("SUCCESS"));
    assert_eq!(result.status.as_deref(), Some("FINISHED"));
    assert_eq!(result.transaction_id.as_deref(), Some("4200001234"));
}

/// 分账结果 XML roundtrip。
#[test]
fn test_profit_sharing_result_xml_roundtrip() {
    let mut result = ProfitSharingResult::default();
    result.return_code = Some("SUCCESS".to_string());
    result.result_code = Some("SUCCESS".to_string());
    result.transaction_id = Some("4200001234".to_string());
    result.status = Some("FINISHED".to_string());
    let xml = result.to_xml().expect("XML 序列化失败");
    let back = ProfitSharingResult::from_xml(&xml).expect("XML 反序列化失败");
    assert_eq!(back.return_code.as_deref(), Some("SUCCESS"));
    assert_eq!(back.status.as_deref(), Some("FINISHED"));
}

// ═══ ProfitSharingV3Request JSON（SOURCE_PARITY:
//     Java ProfitSharingServiceImplTest.testProfitSharingV3）═══

/// 分账请求 v3 JSON serde（对应 Java `ProfitSharingV3Request`：`receivers`
/// 为结构化数组，`unfreeze_unsplit` 布尔值）。
/// 对应 Java: ProfitSharingServiceImplTest.testProfitSharingV3
#[test]
fn test_profit_sharing_v3_request_serde() {
    let json = r#"{
        "sub_mchid":"sub123",
        "appid":"wx1234",
        "transaction_id":"4200001234",
        "out_order_no":"ORDER-001",
        "unfreeze_unsplit":true,
        "receivers":[
            {"type":"MERCHANT_ID","account":"1900000109","amount":100,"description":"分账","relation_type":"PARTNER"},
            {"type":"PERSONAL_OPENID","account":"ox123","amount":50,"description":"佣金"}
        ]
    }"#;
    let request: ProfitSharingV3Request = serde_json::from_str(json).unwrap();
    assert_eq!(request.sub_mch_id.as_deref(), Some("sub123"));
    assert_eq!(request.transaction_id.as_deref(), Some("4200001234"));
    assert!(request.unfreeze_unsplit);
    assert_eq!(request.receivers.len(), 2);
    assert_eq!(request.receivers[0].r#type.as_deref(), Some("MERCHANT_ID"));
    assert_eq!(request.receivers[0].amount, Some(100));
    assert_eq!(
        request.receivers[1].r#type.as_deref(),
        Some("PERSONAL_OPENID")
    );
}

// ═══ ProfitSharingV3Result JSON（SOURCE_PARITY:
//     Java ProfitSharingServiceImplTest (v3 response)）═══

/// 分账结果 v3 JSON serde。
/// 对应 Java: ProfitSharingServiceImplTest (v3 result)
#[test]
fn test_profit_sharing_v3_result_serde() {
    let json = r#"{
        "sub_mchid":"sub123",
        "transaction_id":"4200001234",
        "out_order_no":"ORDER-001",
        "order_id":"1000000001",
        "state":"FINISHED",
        "receivers":[
            {"type":"MERCHANT_ID","account":"1900000109","amount":100,"result":"SUCCESS","detail_id":"D001"}
        ]
    }"#;
    let result: ProfitSharingV3Result = serde_json::from_str(json).unwrap();
    assert_eq!(result.state.as_deref(), Some("FINISHED"));
    assert_eq!(result.receivers.len(), 1);
    assert_eq!(result.receivers[0].result.as_deref(), Some("SUCCESS"));
    assert_eq!(result.receivers[0].detail_id.as_deref(), Some("D001"));
}

// ═══ ProfitSharingReceiverV3Request（SOURCE_PARITY:
//     Java ProfitSharingServiceImplTest.testAddReceiverV3）═══

/// 添加分账接收方 v3 请求（对应 Java `ProfitSharingReceiverV3Request`：
/// `type`/`account`/`name`/`relation_type`/`custom_relation`）。
/// 对应 Java: ProfitSharingServiceImplTest.testAddReceiverV3
#[test]
fn test_profit_sharing_receiver_v3_request_serde() {
    let json = r#"{
        "sub_mchid":"sub123",
        "type":"MERCHANT_ID",
        "account":"1900000109",
        "name":"商户名称",
        "relation_type":"PARTNER",
        "custom_relation":"自定义关系"
    }"#;
    let request: ProfitSharingReceiverV3Request = serde_json::from_str(json).unwrap();
    assert_eq!(request.r#type.as_deref(), Some("MERCHANT_ID"));
    assert_eq!(request.account.as_deref(), Some("1900000109"));
    assert_eq!(request.relation_type.as_deref(), Some("PARTNER"));
}

/// 添加分账接收方 v3 结果。
#[test]
fn test_profit_sharing_receiver_v3_result_serde() {
    let json = r#"{
        "type":"MERCHANT_ID",
        "account":"1900000109",
        "name":"商户名称",
        "relation_type":"PARTNER"
    }"#;
    let result: ProfitSharingReceiverV3Result = serde_json::from_str(json).unwrap();
    assert_eq!(result.r#type.as_deref(), Some("MERCHANT_ID"));
}

// ═══ ProfitSharingQueryRequest v2 ═══

/// 对应 Java: ProfitSharingServiceImplTest.testProfitSharingQuery
#[test]
fn test_profit_sharing_query_request_serde() {
    let json = r#"{
        "transaction_id":"4200001234",
        "out_order_no":"ORDER-001",
        "nonce_str":"nonce123"
    }"#;
    let request: ProfitSharingQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.transaction_id.as_deref(), Some("4200001234"));
}

/// 分账查询结果 v2。
#[test]
fn test_profit_sharing_query_result_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "result_code":"SUCCESS",
        "transaction_id":"4200001234",
        "out_order_no":"ORDER-001",
        "status":"FINISHED",
        "receivers":"[{\"type\":\"MERCHANT_ID\",\"result\":\"SUCCESS\"}]"
    }"#;
    let result: ProfitSharingQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.status.as_deref(), Some("FINISHED"));
}

// ═══ ProfitSharingQueryV3Request ═══

/// 对应 Java: ProfitSharingServiceImplTest.testProfitSharingQueryV3
#[test]
fn test_profit_sharing_query_v3_request_serde() {
    let json = r#"{
        "transaction_id":"4200001234",
        "out_order_no":"ORDER-001"
    }"#;
    let request: ProfitSharingQueryV3Request = serde_json::from_str(json).unwrap();
    assert_eq!(request.transaction_id.as_deref(), Some("4200001234"));
}

// ═══ ProfitSharingReturnRequest v2（SOURCE_PARITY:
//     Java ProfitSharingServiceImplTest.testProfitSharingReturn）═══

/// 分账回退请求 v2（对应 Java `ProfitSharingReturnRequest`）。
/// 对应 Java: ProfitSharingServiceImplTest.testProfitSharingReturn
#[test]
fn test_profit_sharing_return_request_serde() {
    let json = r#"{
        "order_id":"1000000001",
        "out_order_no":"ORDER-001",
        "out_return_no":"RETURN-001",
        "return_account_type":"MERCHANT_ID",
        "return_account":"1900000109",
        "return_amount":50,
        "description":"回退测试"
    }"#;
    let request: ProfitSharingReturnRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.order_id.as_deref(), Some("1000000001"));
    assert_eq!(request.return_amount, Some(50));
}

/// 分账回退结果 v2。
#[test]
fn test_profit_sharing_return_result_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "result_code":"SUCCESS",
        "order_id":"1000000001",
        "out_return_no":"RETURN-001",
        "return_no":"RF-001",
        "return_amount":50,
        "result":"SUCCESS"
    }"#;
    let result: ProfitSharingReturnResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.return_no.as_deref(), Some("RF-001"));
    assert_eq!(result.return_amount, Some(50));
}

// ═══ ProfitSharingReturnV3Request v3 ═══

/// 分账回退请求 v3。
/// 对应 Java: ProfitSharingServiceImplTest.testProfitSharingReturnV3
#[test]
fn test_profit_sharing_return_v3_request_serde() {
    let json = r#"{
        "sub_mchid":"sub123",
        "order_id":"1000000001",
        "out_order_no":"ORDER-001",
        "out_return_no":"RETURN-001",
        "return_mchid":"mch123",
        "amount":50,
        "description":"回退测试"
    }"#;
    let request: ProfitSharingReturnV3Request = serde_json::from_str(json).unwrap();
    assert_eq!(request.order_id.as_deref(), Some("1000000001"));
    assert_eq!(request.amount, Some(50));
}

/// 分账回退结果 v3。
#[test]
fn test_profit_sharing_return_v3_result_serde() {
    let json = r#"{
        "order_id":"1000000001",
        "out_return_no":"RETURN-001",
        "return_id":"RF-001",
        "amount":50,
        "result":"SUCCESS"
    }"#;
    let result: ProfitSharingReturnV3Result = serde_json::from_str(json).unwrap();
    assert_eq!(result.return_id.as_deref(), Some("RF-001"));
    assert_eq!(result.amount, Some(50));
}

// ═══ ProfitSharingUnfreezeRequest（SOURCE_PARITY:
//     Java ProfitSharingServiceImplTest.testProfitSharingFinish）═══

/// 分账完结请求（对应 Java `ProfitSharingUnfreezeRequest`）。
/// 对应 Java: ProfitSharingServiceImplTest.testProfitSharingFinish
#[test]
fn test_profit_sharing_unfreeze_request_serde() {
    let json = r#"{
        "transaction_id":"4200001234",
        "out_order_no":"ORDER-001",
        "description":"分账完结"
    }"#;
    let request: ProfitSharingUnfreezeRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.transaction_id.as_deref(), Some("4200001234"));
}

// ═══ ProfitSharingUnfreezeV3Request ═══

/// 分账完结请求 v3。
#[test]
fn test_profit_sharing_unfreeze_v3_request_serde() {
    let json = r#"{
        "sub_mchid":"sub123",
        "transaction_id":"4200001234",
        "out_order_no":"ORDER-001",
        "description":"分账完结"
    }"#;
    let request: ProfitSharingUnfreezeV3Request = serde_json::from_str(json).unwrap();
    assert_eq!(request.transaction_id.as_deref(), Some("4200001234"));
}

// ═══ ProfitSharingUnfreezeV3Result ═══

#[test]
fn test_profit_sharing_unfreeze_v3_result_serde() {
    let json = r#"{
        "transaction_id":"4200001234",
        "out_order_no":"ORDER-001",
        "order_id":"1000000001",
        "state":"FINISHED"
    }"#;
    let result: ProfitSharingUnfreezeV3Result = serde_json::from_str(json).unwrap();
    assert_eq!(result.state.as_deref(), Some("FINISHED"));
}

// ═══ ProfitSharingNotifyV3Result ═══

/// 分账通知 v3 结果（对应 Java `ProfitSharingNotifyV3Result`：扁平结构，
/// `mchid`/`transaction_id`/`out_order_no`/`receiver` 等字段）。
#[test]
fn test_profit_sharing_notify_v3_result_serde() {
    let json = r#"{
        "mchid":"mch123",
        "transaction_id":"4200001234",
        "out_order_no":"ORDER-001",
        "order_id":"1000000001",
        "receiver":{"type":"MERCHANT_ID","account":"1900000109","amount":100}
    }"#;
    let result: ProfitSharingNotifyV3Result = serde_json::from_str(json).unwrap();
    assert_eq!(result.mch_id.as_deref(), Some("mch123"));
    assert_eq!(result.transaction_id.as_deref(), Some("4200001234"));
    assert!(result.receiver.is_some());
}

// ═══ ProfitSharingOrderAmountQueryResult ═══

/// 对应 Java: ProfitSharingServiceImplTest.testProfitSharingOrderAmountQuery
#[test]
fn test_profit_sharing_order_amount_query_result_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "result_code":"SUCCESS",
        "transaction_id":"4200001234",
        "unsplit_amount":200
    }"#;
    let result: ProfitSharingOrderAmountQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.transaction_id.as_deref(), Some("4200001234"));
    assert_eq!(result.un_split_amount, Some(200));
}

// ═══ ProfitSharingMerchantRatioQueryV3Result ═══

/// 对应 Java: ProfitSharingServiceImplTest.testProfitSharingMerchantRatioQueryV3
#[test]
fn test_profit_sharing_merchant_ratio_query_v3_result_serde() {
    let json = r#"{
        "sub_mchid":"sub123",
        "max_ratio":3000
    }"#;
    let result: ProfitSharingMerchantRatioQueryV3Result = serde_json::from_str(json).unwrap();
    assert_eq!(result.max_ratio, Some(3000));
}

// ═══ ProfitSharingBillV3Request ═══

/// 对应 Java: ProfitSharingServiceImplTest.testProfitSharingBill
#[test]
fn test_profit_sharing_bill_v3_request_serde() {
    let json = r#"{
        "bill_date":"2024-01-01",
        "tar_type":"GZIP"
    }"#;
    let request: ProfitSharingBillV3Request = serde_json::from_str(json).unwrap();
    assert_eq!(request.bill_date.as_deref(), Some("2024-01-01"));
}

// ═══ VALUE_ADD: 空值/边界 ═══

#[test]
fn test_profit_sharing_request_empty_receivers() {
    let request = ProfitSharingRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("appid"));
    assert!(!json.contains("transaction_id"));
}

#[test]
fn test_profit_sharing_v3_request_empty_receivers() {
    let json = r#"{"transaction_id":"T1","out_order_no":"O1","receivers":[]}"#;
    let request: ProfitSharingV3Request = serde_json::from_str(json).unwrap();
    assert!(request.receivers.is_empty());
    assert!(!request.unfreeze_unsplit);
}

#[test]
fn test_profit_sharing_v3_result_empty() {
    let result: ProfitSharingV3Result = serde_json::from_str("{}").unwrap();
    assert!(result.receivers.is_empty());
    assert!(result.state.is_none());
}

/// 分账回退查询请求（对应 Java `ProfitSharingReturnQueryRequest`）。
#[test]
fn test_profit_sharing_return_query_request_serde() {
    let json = r#"{
        "order_id":"1000000001",
        "out_return_no":"RETURN-001"
    }"#;
    let request: ProfitSharingReturnQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.out_return_no.as_deref(), Some("RETURN-001"));
}

/// 分账回退查询 v3 请求。
#[test]
fn test_profit_sharing_return_v3_query_request_serde() {
    let json = r#"{
        "order_id":"1000000001",
        "out_return_no":"RETURN-001",
        "out_order_no":"ORDER-001"
    }"#;
    let request: ProfitSharingReturnV3Request = serde_json::from_str(json).unwrap();
    assert_eq!(request.out_return_no.as_deref(), Some("RETURN-001"));
}
