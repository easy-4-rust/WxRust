#![allow(clippy::field_reassign_with_default)]
//! Phase 2 补齐: Transfer / Redpack / Notify 子域 Bean 序列化测试。
//!
//! 镜像 Java:
//! - `TransferServiceImplTest`（商家批量转账、转账账单查询、转账通知解析）
//! - `RedpackServiceImplTest`（发放红包、查询红包）
//! - `WxPayOrderNotifyResultTest`（代金券 coupon_count 解析）
//! - `WxScanPayNotifyResultTest`（扫码支付通知 bean）
//! - `WxPayNotifyResponseTest`（通知应答 XML 生成）
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java @Test 的序列化/反序列化断言
//! - RUST_OBLIGATION: serde skip_serializing_if、Option 语义
//! - VALUE_ADD: 空值/边界/异常映射路径

use wx_rust_pay::bean::notify::*;
use wx_rust_pay::bean::transfer::*;

// ═══ Transfer Batches Request（SOURCE_PARITY: Java TransferServiceImplTest）═══

/// 商家批量转账请求 serde（对应 Java `TransferBatchesRequest` 字段
/// `appid`/`out_batch_no`/`batch_name`/`total_amount`/`total_num`/
/// `transfer_detail_list`/`transfer_scene_id`/`notify_url`）。
/// 对应 Java: TransferServiceImplTest.testTransferBatches
#[test]
fn test_transfer_batches_request_serde() {
    let json = r#"{
        "appid":"wx1234",
        "out_batch_no":"BATCH-001",
        "batch_name":"测试转账",
        "batch_remark":"备注",
        "total_amount":1000,
        "total_num":2,
        "transfer_scene_id":"1000",
        "notify_url":"https://example.com/notify",
        "transfer_detail_list":[
            {"out_detail_no":"D1","transfer_amount":500,"transfer_remark":"明细1","openid":"ox123"},
            {"out_detail_no":"D2","transfer_amount":500,"transfer_remark":"明细2","openid":"ox456"}
        ]
    }"#;
    let request: TransferBatchesRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.appid.as_deref(), Some("wx1234"));
    assert_eq!(request.out_batch_no.as_deref(), Some("BATCH-001"));
    assert_eq!(request.batch_name.as_deref(), Some("测试转账"));
    assert_eq!(request.total_amount, Some(1000));
    assert_eq!(request.total_num, Some(2));
    assert_eq!(request.transfer_detail_list.len(), 2);
    assert_eq!(
        request.transfer_detail_list[0].openid.as_deref(),
        Some("ox123")
    );
    assert_eq!(request.transfer_detail_list[1].transfer_amount, Some(500));
    assert_eq!(request.transfer_scene_id.as_deref(), Some("1000"));
    assert_eq!(
        request.notify_url.as_deref(),
        Some("https://example.com/notify")
    );
}

/// 转账请求空值跳过（对应 Java Gson skip null）。
/// 对应 Java: TransferServiceImplTest (VALUE_ADD)
#[test]
fn test_transfer_batches_request_none_skipping() {
    let request = TransferBatchesRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("appid"));
    assert!(!json.contains("out_batch_no"));
    assert!(json.contains("\"transfer_detail_list\":[]"));
}

// ═══ Transfer Batches Result（SOURCE_PARITY: Java TransferServiceImplTest）═══

/// 对应 Java: TransferServiceImplTest.testTransferBatches (response parse)
#[test]
fn test_transfer_batches_result_serde() {
    let json = r#"{"out_batch_no":"BATCH-001","batch_id":"1000000001","create_time":"2024-01-01T00:00:00+08:00","batch_status":"WAIT_PAY"}"#;
    let result: TransferBatchesResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.out_batch_no.as_deref(), Some("BATCH-001"));
    assert_eq!(result.batch_id.as_deref(), Some("1000000001"));
    assert_eq!(result.batch_status.as_deref(), Some("WAIT_PAY"));
}

// ═══ Transfer Notify Result（SOURCE_PARITY: Java parseTransferNotifyResult）═══

/// 商家转账通知解析结果 serde（对应 Java `TransferNotifyResult` + 内嵌
/// `DecryptNotifyResult`）。
/// 对应 Java: TransferServiceImplTest.testParseTransferNotifyResult
#[test]
fn test_transfer_notify_result_serde() {
    let json = r#"{
        "rawData":{"id":"ev-001","create_time":"2024-01-01","event_type":"MCHTRANSFER.BATCH.FINISH"},
        "result":{
            "mchid":"1234567891",
            "out_batch_no":"BATCH-001",
            "batch_id":"1000000001",
            "batch_status":"FINISHED",
            "total_num":2,
            "total_amount":1000,
            "success_amount":1000,
            "success_num":2,
            "fail_amount":0,
            "fail_num":0,
            "update_time":"2024-01-01T01:00:00+08:00",
            "close_reason":""
        }
    }"#;
    let result: TransferNotifyResult = serde_json::from_str(json).unwrap();
    assert!(result.raw_data.is_some());
    let raw = result.raw_data.unwrap();
    assert_eq!(raw.event_type.as_deref(), Some("MCHTRANSFER.BATCH.FINISH"));
    let inner = result.result.unwrap();
    assert_eq!(inner.mchid.as_deref(), Some("1234567891"));
    assert_eq!(inner.out_batch_no.as_deref(), Some("BATCH-001"));
    assert_eq!(inner.batch_status.as_deref(), Some("FINISHED"));
    assert_eq!(inner.total_num, Some(2));
    assert_eq!(inner.total_amount, Some(1000));
    assert_eq!(inner.success_num, Some(2));
    assert_eq!(inner.fail_num, Some(0));
}

// ═══ TransferBillsRequest（SOURCE_PARITY: Java transferBills 接口）═══

/// 对应 Java: TransferServiceImplTest.testTransferBills
#[test]
fn test_transfer_bills_request_serde() {
    let json = r#"{
        "appid":"wx1234",
        "out_bill_no":"BILL-001",
        "transfer_scene_id":"1000",
        "transfer_amount":100,
        "transfer_remark":"转账备注",
        "openid":"ox123",
        "user_name":"encrypted_name"
    }"#;
    let request: TransferBillsRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.appid.as_deref(), Some("wx1234"));
    assert_eq!(request.out_bill_no.as_deref(), Some("BILL-001"));
    assert_eq!(request.transfer_amount, Some(100));
    assert_eq!(request.openid.as_deref(), Some("ox123"));
}

/// 对应 Java: TransferServiceImplTest.testTransferBills (response)
#[test]
fn test_transfer_bills_result_serde() {
    let json = r#"{"out_bill_no":"BILL-001","transfer_bill_no":"TB-001","state":"ACCEPTED"}"#;
    let result: TransferBillsResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.out_bill_no.as_deref(), Some("BILL-001"));
    assert_eq!(result.transfer_bill_no.as_deref(), Some("TB-001"));
    assert_eq!(result.state.as_deref(), Some("ACCEPTED"));
}

/// 对应 Java: TransferServiceImplTest.testGetBillsByOutBillNo
#[test]
fn test_transfer_bills_get_result_serde() {
    let json = r#"{
        "mch_id":"10000100",
        "out_bill_no":"BILL-001",
        "transfer_bill_no":"TB-001",
        "appid":"wx1234",
        "state":"SUCCESS",
        "transfer_amount":"100",
        "openid":"ox123",
        "fail_reason":"",
        "create_time":"2024-01-01T00:00:00+08:00"
    }"#;
    let result: TransferBillsGetResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.out_bill_no.as_deref(), Some("BILL-001"));
    assert_eq!(result.state.as_deref(), Some("SUCCESS"));
    assert_eq!(result.transfer_amount.as_deref(), Some("100"));
}

// ═══ TransferBillsCancelResult ═══

/// 对应 Java: TransferServiceImplTest.testTransformBillsCancel
#[test]
fn test_transfer_bills_cancel_result_serde() {
    let json = r#"{"out_bill_no":"BILL-001","transfer_bill_no":"TB-001","state":"CANCELLED"}"#;
    let result: TransferBillsCancelResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.out_bill_no.as_deref(), Some("BILL-001"));
    assert_eq!(result.state.as_deref(), Some("CANCELLED"));
}

// ═══ QueryTransferBatchesResult（SOURCE_PARITY: Java 查询批次结果）═══

/// 对应 Java: TransferServiceImplTest.testTransferBatchesBatchId
#[test]
fn test_query_transfer_batches_result_serde() {
    let json = r#"{
        "offset":0,
        "limit":10,
        "transfer_batch":{
            "mchid":"10000100",
            "out_batch_no":"BATCH-001",
            "batch_id":"1000000001",
            "batch_status":"FINISHED",
            "batch_type":"API",
            "total_num":2,
            "total_amount":1000,
            "success_num":2,
            "success_amount":1000,
            "fail_num":0,
            "fail_amount":0
        },
        "transfer_detail_list":[
            {"detail_id":"D001","out_detail_no":"D1","detail_status":"SUCCESS"},
            {"detail_id":"D002","out_detail_no":"D2","detail_status":"SUCCESS"}
        ]
    }"#;
    let result: QueryTransferBatchesResult = serde_json::from_str(json).unwrap();
    let batch = result.transfer_batch.as_ref().unwrap();
    assert_eq!(batch.batch_id.as_deref(), Some("1000000001"));
    assert_eq!(batch.batch_status.as_deref(), Some("FINISHED"));
    assert_eq!(batch.total_num, Some(2));
    assert_eq!(result.transfer_detail_list.len(), 2);
    assert_eq!(
        result.transfer_detail_list[0].detail_id.as_deref(),
        Some("D001")
    );
}

// ═══ QueryTransferBatchesRequest ═══

/// 对应 Java: TransferServiceImplTest (query params)
#[test]
fn test_query_transfer_batches_request_serde() {
    let json = r#"{"batchId":"1000000001","needQueryDetail":true,"offset":0,"limit":10,"detailStatus":"SUCCESS"}"#;
    let request: QueryTransferBatchesRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.batch_id.as_deref(), Some("1000000001"));
    assert_eq!(request.need_query_detail, Some(true));
    assert_eq!(request.offset, Some(0));
    assert_eq!(request.limit, Some(10));
}

// ═══ BusinessOperationTransfer（SOURCE_PARITY:
//     Java BusinessOperationTransferServiceTest）═══

/// 对应 Java: BusinessOperationTransferServiceTest.testTransfer
#[test]
fn test_business_operation_transfer_request_serde() {
    let json = r#"{
        "appid":"wx1234",
        "out_bill_no":"BIZ-001",
        "transfer_scene_id":"1001",
        "openid":"ox789",
        "transfer_amount":500,
        "transfer_remark":"业务转账"
    }"#;
    let request: BusinessOperationTransferRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.out_bill_no.as_deref(), Some("BIZ-001"));
    assert_eq!(request.transfer_amount, Some(500));
    assert_eq!(request.openid.as_deref(), Some("ox789"));
}

/// 对应 Java: BusinessOperationTransferServiceTest.testTransfer (response)
#[test]
fn test_business_operation_transfer_result_serde() {
    let json = r#"{"out_bill_no":"BIZ-001","transfer_bill_no":"TB-001","state":"ACCEPTED"}"#;
    let result: BusinessOperationTransferResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.out_bill_no.as_deref(), Some("BIZ-001"));
    assert_eq!(result.state.as_deref(), Some("ACCEPTED"));
}

// ═══ WxScanPayNotifyResult（SOURCE_PARITY: Java WxScanPayNotifyResultTest）═══

/// 扫码支付通知结果 bean JSON serde（对应 Java `WxScanPayNotifyResult`：
/// `openid`/`is_subscribe`/`product_id` 等字段）。
/// 对应 Java: WxScanPayNotifyResultTest
#[test]
fn test_scan_pay_notify_result_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "return_msg":"OK",
        "appid":"wx1234",
        "mch_id":"mch123",
        "nonce_str":"nonce123",
        "sign":"sign123",
        "result_code":"SUCCESS",
        "openid":"ox123",
        "is_subscribe":"Y",
        "product_id":"88888"
    }"#;
    let result: WxScanPayNotifyResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.openid.as_deref(), Some("ox123"));
    assert_eq!(result.is_subscribe.as_deref(), Some("Y"));
    assert_eq!(result.product_id.as_deref(), Some("88888"));
}

// ═══ WxPayOrderNotifyResult 代金券（SOURCE_PARITY:
//     Java WxPayOrderNotifyResultTest.testFromXML coupon_count/couponList）═══

/// 订单通知结果 coupon_count 字段（对应 Java `couponCount` + `couponList`）。
/// 对应 Java: WxPayOrderNotifyResultTest.testFromXML
#[test]
fn test_order_notify_coupon_count_field() {
    let json = r#"{
        "return_code":"SUCCESS",
        "appid":"wx2421b1c4370ec43b",
        "mch_id":"10000100",
        "out_trade_no":"1409811653",
        "total_fee":1,
        "coupon_count":2
    }"#;
    let result: WxPayOrderNotifyResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.coupon_count, Some(2));
    assert_eq!(result.appid.as_deref(), Some("wx2421b1c4370ec43b"));
    assert_eq!(result.total_fee, Some(1));
}

// ═══ WxPayOrderNotifyCoupon（RUST_OBLIGATION: to_map 辅助）═══

/// 代金券 to_map 辅助方法（对应 Java `WxPayOrderNotifyCoupon.toMap(index)`）。
/// 对应 Java: WxPayOrderNotifyResultTest (coupon list processing)
#[test]
fn test_order_notify_coupon_to_map() {
    let coupon = WxPayOrderNotifyCoupon {
        coupon_id: Some("10000".to_string()),
        coupon_type: Some("CASH".to_string()),
        coupon_fee: Some(100),
    };
    let map = coupon.to_map(0);
    assert_eq!(map.get("coupon_id_0").map(String::as_str), Some("10000"));
    assert_eq!(map.get("coupon_type_0").map(String::as_str), Some("CASH"));
    assert_eq!(map.get("coupon_fee_0").map(String::as_str), Some("100"));
}

// ═══ WxPayNotifyResponse（SOURCE_PARITY: Java WxPayNotifyResponseTest）═══

/// 支付通知应答 bean（对应 Java `WxPayNotifyResponse`）。
/// 对应 Java: WxPayNotifyResponseTest
#[test]
fn test_pay_notify_response_serde() {
    let json = r#"{"return_code":"SUCCESS","return_msg":"OK"}"#;
    let resp: WxPayNotifyResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.return_code.as_deref(), Some("SUCCESS"));
    assert_eq!(resp.return_msg.as_deref(), Some("OK"));
}

/// 支付通知应答 XML 生成（对应 Java `WxPayNotifyResponse.successResp`/
/// `failResp`/`generateXml`）。
/// 对应 Java: WxPayNotifyResponseTest.testGenerateXml
#[test]
fn test_pay_notify_response_generate_xml() {
    let success_xml = WxPayNotifyResponse::success();
    assert!(success_xml.contains("<return_code><![CDATA[SUCCESS]]></return_code>"));
    assert!(success_xml.contains("<return_msg><![CDATA[OK]]></return_msg>"));

    let fail_xml = WxPayNotifyResponse::fail("签名错误");
    assert!(fail_xml.contains("<return_code><![CDATA[FAIL]]></return_code>"));
    assert!(fail_xml.contains("<return_msg><![CDATA[签名错误]]></return_msg>"));

    let custom = WxPayNotifyResponse::success_resp("自定义消息");
    assert!(custom.contains("<return_msg><![CDATA[自定义消息]]></return_msg>"));
}

/// v3 通知应答 JSON 生成（对应 Java `WxPayNotifyV3Response`）。
/// 对应 Java: WxPayNotifyResponseTest (v3)
#[test]
fn test_pay_notify_v3_response_generate_json() {
    let success = WxPayNotifyV3Response::success("成功");
    let v: serde_json::Value = serde_json::from_str(&success).unwrap();
    assert_eq!(v["code"], "SUCCESS");
    assert_eq!(v["message"], "成功");

    let fail = WxPayNotifyV3Response::fail("失败原因");
    let v: serde_json::Value = serde_json::from_str(&fail).unwrap();
    assert_eq!(v["code"], "FAIL");
    assert_eq!(v["message"], "失败原因");
}

// ═══ VALUE_ADD: 空值/边界 ═══

#[test]
fn test_transfer_batches_request_empty_details() {
    let json = r#"{"out_batch_no":"B1","transfer_detail_list":[]}"#;
    let request: TransferBatchesRequest = serde_json::from_str(json).unwrap();
    assert!(request.transfer_detail_list.is_empty());
    assert_eq!(request.out_batch_no.as_deref(), Some("B1"));
}

#[test]
fn test_transfer_notify_result_none_fields() {
    let result: TransferNotifyResult = serde_json::from_str("{}").unwrap();
    assert!(result.raw_data.is_none());
    assert!(result.result.is_none());
}

#[test]
fn test_query_transfer_batches_result_empty_details() {
    let json = r#"{"transfer_detail_list":[]}"#;
    let result: QueryTransferBatchesResult = serde_json::from_str(json).unwrap();
    assert!(result.transfer_detail_list.is_empty());
    assert!(result.transfer_batch.is_none());
}

#[test]
fn test_transfer_bills_request_default_serde() {
    let request = TransferBillsRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("appid"));
    assert!(!json.contains("out_bill_no"));
    assert!(json.contains("\"transfer_scene_report_infos\":[]"));
}

/// 转账通知空 rawData/result（VALUE_ADD: 未解密通知只有 envelope）。
#[test]
fn test_transfer_notify_result_envelope_only() {
    let json = r#"{"rawData":{"id":"ev-002","event_type":"MCHTRANSFER.BATCH.FINISH"}}"#;
    let result: TransferNotifyResult = serde_json::from_str(json).unwrap();
    assert!(result.raw_data.is_some());
    assert!(result.result.is_none());
}
