#![allow(clippy::field_reassign_with_default)]
//! Phase 3 P2 扩展: pay 商家转账（MerchantTransfer）+ 品牌转账（BrandMerchantTransfer）+
//! 海关申报（Customs）+ 媒体（Media）Bean 测试。
//!
//! 镜像 Java:
//! - `MerchantTransferServiceImplTest`（商家转账创建/查询/明细查询）
//! - `BrandMerchantTransferServiceImplTest`（品牌转账批次/查询/明细）
//! - `CustomDeclarationServiceImplTest`（海关申报/查询/重报/证书校验）
//! - `MerchantMediaServiceImplTest`（商户媒体上传）
//! - `BusinessCircleServiceImplTest`（商圈数据查询）
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java @Test 的序列化/反序列化断言
//! - RUST_OBLIGATION: serde skip_serializing_if、Option 语义、嵌套结构
//! - VALUE_ADD: 空值/边界/默认值路径

use wx_rust_pay::bean::brandmerchanttransfer::*;
use wx_rust_pay::bean::customs::*;
use wx_rust_pay::bean::media::*;
use wx_rust_pay::bean::merchanttransfer::*;

// ═══════════════════════════════════════════════════════════════
// 1. 商家转账创建（SOURCE_PARITY:
//    Java MerchantTransferServiceImplTest.testCreateTransfer）
// ═══════════════════════════════════════════════════════════════

/// 商家转账创建请求 serde（对应 Java `TransferCreateRequest`：
/// `appid`/`out_batch_no`/`batch_name`/`batch_remark`/`total_amount`/
/// `total_num`/`transfer_detail_list`/`transfer_scene_id`/`notify_url`）。
/// 对应 Java: MerchantTransferServiceImplTest.testCreateTransfer
#[test]
fn test_merchant_transfer_create_request_serde() {
    let json = r#"{
        "appid":"wx1234",
        "out_batch_no":"BATCH-001",
        "batch_name":"商家转账",
        "batch_remark":"测试转账",
        "total_amount":2000,
        "total_num":2,
        "transfer_scene_id":"1000",
        "notify_url":"https://example.com/notify",
        "transfer_detail_list":[
            {"out_detail_no":"D1","transfer_amount":1000,"transfer_remark":"明细1","openid":"ox123"},
            {"out_detail_no":"D2","transfer_amount":1000,"transfer_remark":"明细2","openid":"ox456"}
        ]
    }"#;
    let request: TransferCreateRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.appid.as_deref(), Some("wx1234"));
    assert_eq!(request.out_batch_no.as_deref(), Some("BATCH-001"));
    assert_eq!(request.total_amount, Some(2000));
    assert_eq!(request.total_num, Some(2));
    assert_eq!(request.transfer_detail_list.len(), 2);
    assert_eq!(
        request.transfer_detail_list[0].openid.as_deref(),
        Some("ox123")
    );
    assert_eq!(request.transfer_detail_list[1].transfer_amount, Some(1000));
}

/// 商家转账创建结果 serde。
#[test]
fn test_merchant_transfer_create_result_serde() {
    let json = r#"{"out_batch_no":"BATCH-001","batch_id":"1000000001","create_time":"2024-01-01T00:00:00+08:00"}"#;
    let result: TransferCreateResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.out_batch_no.as_deref(), Some("BATCH-001"));
    assert_eq!(result.batch_id.as_deref(), Some("1000000001"));
}

// ═══════════════════════════════════════════════════════════════
// 2. 商家转账批次查询（SOURCE_PARITY:
//    Java MerchantTransferServiceImplTest.testQueryBatch）
// ═══════════════════════════════════════════════════════════════

/// 商家转账批次查询请求 serde（对应 Java `MerchantBatchesQueryRequest`：
/// `out_batch_no`/`need_query_detail`/`offset`/`limit`/`detail_status`）。
/// 对应 Java: MerchantTransferServiceImplTest.testQueryBatch
#[test]
fn test_merchant_batches_query_request_serde() {
    let json = r#"{"out_batch_no":"BATCH-001","need_query_detail":true,"offset":0,"limit":20,"detail_status":"SUCCESS"}"#;
    let request: MerchantBatchesQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.out_batch_no.as_deref(), Some("BATCH-001"));
    assert_eq!(request.need_query_detail, Some(true));
}

/// 商家转账批次查询结果 serde（对应 Java `BatchesQueryResult`：
/// `batch_id`/`out_batch_no`/`batch_status`/`total_num`/`total_amount`/
/// `transfer_detail_list`）。
#[test]
fn test_merchant_batches_query_result_serde() {
    let json = r#"{
        "transfer_batch":{"batch_id":"1000000001","out_batch_no":"BATCH-001","batch_status":"FINISHED","total_num":2,"total_amount":2000,"create_time":"2024-01-01"},
        "transfer_detail_list":[
            {"detail_id":"D001","out_detail_no":"D1","detail_status":"SUCCESS","transfer_amount":1000}
        ]
    }"#;
    let result: BatchesQueryResult = serde_json::from_str(json).unwrap();
    assert!(result.transfer_batch.is_some());
    let batch = result.transfer_batch.as_ref().unwrap();
    assert_eq!(batch.batch_id.as_deref(), Some("1000000001"));
    assert_eq!(batch.batch_status.as_deref(), Some("FINISHED"));
}

// ═══════════════════════════════════════════════════════════════
// 3. 商家转账明细查询（SOURCE_PARITY:
//    Java MerchantTransferServiceImplTest.testQueryDetail）
// ═══════════════════════════════════════════════════════════════

/// 商家转账明细查询请求 serde（对应 Java `MerchantDetailsQueryRequest`：
/// `out_batch_no`/`out_detail_no`）。
/// 对应 Java: MerchantTransferServiceImplTest.testQueryDetail
#[test]
fn test_merchant_details_query_request_serde() {
    let json = r#"{"out_batch_no":"BATCH-001","out_detail_no":"D1"}"#;
    let request: MerchantDetailsQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.out_batch_no.as_deref(), Some("BATCH-001"));
    assert_eq!(request.out_detail_no.as_deref(), Some("D1"));
}

/// 商家转账明细查询结果 serde。
#[test]
fn test_merchant_details_query_result_serde() {
    let json = r#"{
        "detail_id":"D001",
        "out_detail_no":"D1",
        "detail_status":"SUCCESS",
        "transfer_amount":1000,
        "transfer_remark":"测试",
        "openid":"ox123",
        "user_name":"张三"
    }"#;
    let result: DetailsQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.detail_id.as_deref(), Some("D001"));
    assert_eq!(result.detail_status.as_deref(), Some("SUCCESS"));
    assert_eq!(result.transfer_amount, Some(1000));
}

// ═══════════════════════════════════════════════════════════════
// 4. 品牌转账批次（SOURCE_PARITY:
//    Java BrandMerchantTransferServiceImplTest.testCreateTransfer）
// ═══════════════════════════════════════════════════════════════

/// 品牌转账批次创建请求 serde（对应 Java `BrandTransferBatchesRequest`：
/// `brand_id`/`brand_appid`/`scene`/`template_id`/`out_batch_no`/
/// `batch_name`/`total_amount`/`total_num`/`detail_list`）。
/// 对应 Java: BrandMerchantTransferServiceImplTest.testCreateTransfer
#[test]
fn test_brand_transfer_batches_request_serde() {
    let json = r#"{
        "brand_id":1001,
        "brand_appid":"wx1234",
        "scene":"PRODUCT_PROMOTION",
        "template_id":"TPL-001",
        "out_batch_no":"BBATCH-001",
        "batch_name":"品牌转账",
        "batch_remark":"测试",
        "total_amount":3000,
        "total_num":2,
        "detail_list":[
            {"out_detail_no":"BD1","amount":1500,"openid":"ox123","remark":"明细1"},
            {"out_detail_no":"BD2","amount":1500,"openid":"ox456","remark":"明细2"}
        ]
    }"#;
    let request: BrandTransferBatchesRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.brand_id, Some(1001));
    assert_eq!(request.brand_appid.as_deref(), Some("wx1234"));
    assert_eq!(request.out_batch_no.as_deref(), Some("BBATCH-001"));
    assert_eq!(request.total_amount, Some(3000));
    assert_eq!(request.detail_list.len(), 2);
    assert_eq!(request.detail_list[0].openid.as_deref(), Some("ox123"));
    assert_eq!(request.detail_list[1].amount, Some(1500));
}

/// 品牌转账批次创建结果 serde（对应 Java `BrandTransferBatchesResult`：
/// `out_batch_no`/`batch_no`/`create_time`）。
/// 对应 Java: BrandMerchantTransferServiceImplTest (result)
#[test]
fn test_brand_transfer_batches_result_serde() {
    let json = r#"{"out_batch_no":"BBATCH-001","batch_no":"2000000001","create_time":"2024-01-01T00:00:00+08:00"}"#;
    let result: BrandTransferBatchesResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.out_batch_no.as_deref(), Some("BBATCH-001"));
    assert_eq!(result.batch_no.as_deref(), Some("2000000001"));
}

// ═══════════════════════════════════════════════════════════════
// 5. 品牌转账查询（SOURCE_PARITY:
//    Java BrandMerchantTransferServiceImplTest.testQueryBatch）
// ═══════════════════════════════════════════════════════════════

/// 品牌转账批次查询请求 serde（对应 Java `BrandMerchantBatchesQueryRequest`：
/// `out_batch_no`/`need_query_detail`/`detail_state`）。
/// 对应 Java: BrandMerchantTransferServiceImplTest.testQueryBatch
#[test]
fn test_brand_merchant_batches_query_request_serde() {
    let json = r#"{"out_batch_no":"BBATCH-001","need_query_detail":true,"detail_state":"SUCCESS"}"#;
    let request: BrandMerchantBatchesQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.out_batch_no.as_deref(), Some("BBATCH-001"));
    assert_eq!(request.detail_state.as_deref(), Some("SUCCESS"));
}

/// 品牌微信批次查询请求 serde（对应 Java `BrandWxBatchesQueryRequest`：
/// `batch_no`/`need_query_detail`/`detail_state`）。
#[test]
fn test_brand_wx_batches_query_request_serde() {
    let json = r#"{"batch_no":"2000000001","need_query_detail":false}"#;
    let request: BrandWxBatchesQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.batch_no.as_deref(), Some("2000000001"));
}

/// 品牌批次查询结果 serde（对应 Java `BrandBatchesQueryResult`：扁平结构，
/// `batch_no`/`out_batch_no`/`batch_state`/`batch_name`/`total_amount`/
/// `total_num`/`detail_list`）。
/// 对应 Java: BrandMerchantTransferServiceImplTest (result)
#[test]
fn test_brand_batches_query_result_serde() {
    let json = r#"{
        "brand_mchid":"mch123",
        "batch_no":"2000000001",
        "out_batch_no":"BBATCH-001",
        "brand_id":1001,
        "batch_state":"FINISHED",
        "batch_name":"品牌转账",
        "total_amount":3000,
        "total_num":2,
        "detail_list":[
            {"transfer_detail_no":"BD001","out_detail_no":"BD1","detail_state":"SUCCESS"}
        ]
    }"#;
    let result: BrandBatchesQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.batch_no.as_deref(), Some("2000000001"));
    assert_eq!(result.batch_state.as_deref(), Some("FINISHED"));
    assert_eq!(result.total_amount, Some(3000));
    assert_eq!(result.detail_list.len(), 1);
    assert_eq!(
        result.detail_list[0].detail_state.as_deref(),
        Some("SUCCESS")
    );
}

/// 品牌明细查询请求 serde（对应 Java `BrandMerchantDetailsQueryRequest`：
/// `out_batch_no`/`out_detail_no`）。
/// 对应 Java: BrandMerchantTransferServiceImplTest.testQueryDetail
#[test]
fn test_brand_details_query_request_serde() {
    let json = r#"{"out_batch_no":"BBATCH-001","out_detail_no":"BD1"}"#;
    let request: BrandMerchantDetailsQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.out_batch_no.as_deref(), Some("BBATCH-001"));
    assert_eq!(request.out_detail_no.as_deref(), Some("BD1"));
}

/// 品牌微信明细查询请求 serde（对应 Java `BrandWxDetailsQueryRequest`：
/// `batch_no`/`detail_no`）。
#[test]
fn test_brand_wx_details_query_request_serde() {
    let json = r#"{"batch_no":"2000000001","detail_no":"BD001"}"#;
    let request: BrandWxDetailsQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.batch_no.as_deref(), Some("2000000001"));
    assert_eq!(request.detail_no.as_deref(), Some("BD001"));
}

/// 品牌明细查询结果 serde（对应 Java `BrandDetailsQueryResult`：
/// `out_batch_no`/`batch_no`/`out_detail_no`/`detail_no`/`detail_state`/
/// `amount`/`openid`/`user_name`）。
#[test]
fn test_brand_details_query_result_serde() {
    let json = r#"{
        "brand_mchid":"mch123",
        "out_batch_no":"BBATCH-001",
        "batch_no":"2000000001",
        "out_detail_no":"BD1",
        "detail_no":"BD001",
        "detail_state":"SUCCESS",
        "amount":1500,
        "openid":"ox123",
        "user_name":"张三"
    }"#;
    let result: BrandDetailsQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.detail_no.as_deref(), Some("BD001"));
    assert_eq!(result.detail_state.as_deref(), Some("SUCCESS"));
    assert_eq!(result.amount, Some(1500));
}

// ═══════════════════════════════════════════════════════════════
// 6. 海关申报（SOURCE_PARITY:
//    Java CustomDeclarationServiceImplTest.testDeclare）
// ═══════════════════════════════════════════════════════════════

/// 海关申报请求 serde（对应 Java `DeclarationRequest`：
/// `appid`/`mchid`/`transaction_id`/`customs`/`merchant_customs_no`/
/// `duty`/`fee_type`/`order_fee`/`transport_fee`/`product_fee`）。
/// 对应 Java: CustomDeclarationServiceImplTest.testDeclare
#[test]
fn test_declaration_request_serde() {
    let json = r#"{
        "appid":"wx1234",
        "mchid":"mch123",
        "out_trade_no":"ORDER-001",
        "transaction_id":"4200001234",
        "customs":"GUANGZHOU_ZS",
        "merchant_customs_no":"MC-001",
        "duty":100,
        "fee_type":"CNY",
        "order_fee":10000,
        "transport_fee":1000,
        "product_fee":9000
    }"#;
    let request: DeclarationRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.appid.as_deref(), Some("wx1234"));
    assert_eq!(request.customs.as_deref(), Some("GUANGZHOU_ZS"));
    assert_eq!(request.duty, Some(100));
    assert_eq!(request.order_fee, Some(10000));
    assert_eq!(request.transport_fee, Some(1000));
    assert_eq!(request.product_fee, Some(9000));
}

/// 海关申报结果 serde（对应 Java `DeclarationResult`：
/// `appid`/`mchid`/`transaction_id`/`state`/`sub_order_no`/`sub_order_id`/
/// `verify_department`/`verify_department_trade_id`）。
/// 对应 Java: CustomDeclarationServiceImplTest (result)
#[test]
fn test_declaration_result_serde() {
    let json = r#"{
        "appid":"wx1234",
        "mchid":"mch123",
        "transaction_id":"4200001234",
        "state":"PROCESSING",
        "sub_order_no":"MC-001",
        "verify_department":"GUANGZHOU_ZS",
        "verify_department_trade_id":"VD-001"
    }"#;
    let result: DeclarationResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.transaction_id.as_deref(), Some("4200001234"));
    assert_eq!(result.state.as_deref(), Some("PROCESSING"));
    assert_eq!(result.verify_department.as_deref(), Some("GUANGZHOU_ZS"));
}

// ═══════════════════════════════════════════════════════════════
// 7. 海关申报查询（SOURCE_PARITY:
//    Java CustomDeclarationServiceImplTest.testQueryDeclaration）
// ═══════════════════════════════════════════════════════════════

/// 海关申报查询请求 serde（对应 Java `DeclarationQueryRequest`：
/// `appid`/`mchid`/`order_type`/`order_no`/`customs`/`offset`/`limit`）。
/// 对应 Java: CustomDeclarationServiceImplTest.testQueryDeclaration
#[test]
fn test_declaration_query_request_serde() {
    let json = r#"{
        "appid":"wx1234",
        "mchid":"mch123",
        "order_type":"TRANSACTION_ID",
        "order_no":"4200001234",
        "customs":"GUANGZHOU_ZS",
        "offset":"0",
        "limit":"20"
    }"#;
    let request: DeclarationQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.order_no.as_deref(), Some("4200001234"));
    assert_eq!(request.customs.as_deref(), Some("GUANGZHOU_ZS"));
}

/// 海关申报查询结果 serde（对应 Java `DeclarationQueryResult`：
/// `transaction_id`/`verify_department`/`data` 列表，每项含 `customs`/
/// `merchant_customs_no`/`duty`/`fee_type`/`order_fee`/`state`）。
#[test]
fn test_declaration_query_result_serde() {
    let json = r#"{
        "appid":"wx1234",
        "mchid":"mch123",
        "transaction_id":"4200001234",
        "verify_department":"GUANGZHOU_ZS",
        "total_count":1,
        "data":[
            {
                "customs":"GUANGZHOU_ZS",
                "merchant_customs_no":"MC-001",
                "duty":100,
                "fee_type":"CNY",
                "order_fee":10000,
                "state":"DEDUCT_SUCCESS"
            }
        ]
    }"#;
    let result: DeclarationQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.transaction_id.as_deref(), Some("4200001234"));
    assert_eq!(result.total_count, Some(1));
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data[0].customs.as_deref(), Some("GUANGZHOU_ZS"));
    assert_eq!(result.data[0].state.as_deref(), Some("DEDUCT_SUCCESS"));
}

// ═══════════════════════════════════════════════════════════════
// 8. 海关重报（SOURCE_PARITY:
//    Java CustomDeclarationServiceImplTest.testRedeclare）
// ═══════════════════════════════════════════════════════════════

/// 海关重报请求 serde。
/// 对应 Java: CustomDeclarationServiceImplTest.testRedeclare
#[test]
fn test_redeclare_request_serde() {
    let json = r#"{
        "transaction_id":"4200001234",
        "customs":"GUANGZHOU_ZS",
        "mchid":"mch123",
        "merchant_customs_no":"MC-002"
    }"#;
    let request: RedeclareRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.transaction_id.as_deref(), Some("4200001234"));
}

/// 海关重报结果 serde（对应 Java `RedeclareResult`：
/// `appid`/`mchid`/`transaction_id`/`state`/`explanation`/`modify_time`）。
/// 对应 Java: CustomDeclarationServiceImplTest (redeclare result)
#[test]
fn test_redeclare_result_serde() {
    let json = r#"{
        "appid":"wx1234",
        "mchid":"mch123",
        "transaction_id":"4200001234",
        "state":"DEDUCT_SUCCESS",
        "explanation":"重报成功",
        "modify_time":"2024-01-02T00:00:00+08:00"
    }"#;
    let result: RedeclareResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.state.as_deref(), Some("DEDUCT_SUCCESS"));
    assert_eq!(result.explanation.as_deref(), Some("重报成功"));
}

// ═══════════════════════════════════════════════════════════════
// 9. 海关证书校验（SOURCE_PARITY:
//    Java CustomDeclarationServiceImplTest.testVerifyCertificate）
// ═══════════════════════════════════════════════════════════════

/// 海关证书校验请求 serde（对应 Java `VerifyCertificateRequest`：
/// `appid`/`mchid`/`transaction_id`/`customs`/`merchant_customs_no`/
/// `certificate_type`/`certificate_id`/`certificate_name`）。
/// 对应 Java: CustomDeclarationServiceImplTest.testVerifyCertificate
#[test]
fn test_verify_certificate_request_serde() {
    let json = r#"{
        "appid":"wx1234",
        "mchid":"mch123",
        "transaction_id":"4200001234",
        "customs":"GUANGZHOU_ZS",
        "merchant_customs_no":"MC-001",
        "certificate_type":"ID_CARD",
        "certificate_id":"310101199001010001",
        "certificate_name":"张三"
    }"#;
    let request: VerifyCertificateRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.mchid.as_deref(), Some("mch123"));
    assert_eq!(request.certificate_type.as_deref(), Some("ID_CARD"));
    assert_eq!(request.certificate_name.as_deref(), Some("张三"));
}

/// 海关证书校验结果 serde（对应 Java `VerifyCertificateResult`：
/// `appid`/`mchid`/`transaction_id`/`certificate_check_result`）。
#[test]
fn test_verify_certificate_result_serde() {
    let json = r#"{
        "appid":"wx1234",
        "mchid":"mch123",
        "transaction_id":"4200001234",
        "certificate_check_result":"MATCHED"
    }"#;
    let result: VerifyCertificateResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.mchid.as_deref(), Some("mch123"));
    assert_eq!(result.certificate_check_result.as_deref(), Some("MATCHED"));
}

// ═══════════════════════════════════════════════════════════════
// 10. 商圈通知（SOURCE_PARITY:
//     Java BusinessCircleServiceImplTest.testBusinessCircleNotify）
// ═══════════════════════════════════════════════════════════════

/// 商圈支付结果通知 serde（对应 Java `BusinessCircleNotifyData`：
/// `id`/`create_time`/`resource_type`/`event_type`/`resource`）。
/// 对应 Java: BusinessCircleServiceImplTest.testBusinessCircleNotify
#[test]
fn test_business_circle_notify_data_serde() {
    let json = r#"{
        "id":"ev-001",
        "create_time":"2024-01-01",
        "resource_type":"encrypt-resource",
        "event_type":"POINTS_NOTIFY",
        "resource":{"algorithm":"AEAD_AES_256_GCM","ciphertext":"encrypted","nonce":"nonce123","original_type":"POINTS"}
    }"#;
    let data: wx_rust_pay::bean::businesscircle::BusinessCircleNotifyData =
        serde_json::from_str(json).unwrap();
    assert_eq!(data.event_type.as_deref(), Some("POINTS_NOTIFY"));
    let resource = data.resource.as_ref().unwrap();
    assert_eq!(resource.algorithm.as_deref(), Some("AEAD_AES_256_GCM"));
    assert_eq!(resource.cipher_text.as_deref(), Some("encrypted"));
}

/// 商圈积分通知请求 serde（对应 Java `PointsNotifyRequest`：
/// `sub_mchid`/`transaction_id`/`appid`/`openid`/`earn_points`/
/// `increased_points`/`total_points`）。
/// 对应 Java: BusinessCircleServiceImplTest.testPointsNotify
#[test]
fn test_points_notify_request_serde() {
    let json = r#"{
        "sub_mchid":"sub123",
        "transaction_id":"4200001234",
        "appid":"wx1234",
        "openid":"ox123",
        "earn_points":true,
        "increased_points":100,
        "total_points":500
    }"#;
    let request: wx_rust_pay::bean::businesscircle::PointsNotifyRequest =
        serde_json::from_str(json).unwrap();
    assert_eq!(request.sub_mchid.as_deref(), Some("sub123"));
    assert_eq!(request.transaction_id.as_deref(), Some("4200001234"));
    assert_eq!(request.earn_points, Some(true));
    assert_eq!(request.increased_points, Some(100));
    assert_eq!(request.total_points, Some(500));
}

// ═══════════════════════════════════════════════════════════════
// 11. 媒体上传（SOURCE_PARITY:
//     Java MerchantMediaServiceImplTest.testUploadMedia）
// ═══════════════════════════════════════════════════════════════

/// 商户图片上传结果 serde（对应 Java `ImageUploadResult`：`media_id`）。
/// 对应 Java: MerchantMediaServiceImplTest.testUploadMedia
#[test]
fn test_image_upload_result_serde() {
    let json = r#"{"media_id":"MEDIA-001"}"#;
    let result: ImageUploadResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.media_id.as_deref(), Some("MEDIA-001"));
}

/// 商户营销图片上传结果 serde（对应 Java `MarketingImageUploadResult`：
/// `media_url`）。
/// 对应 Java: MerchantMediaServiceImplTest.testUploadMarketingImage
#[test]
fn test_marketing_image_upload_result_serde() {
    let json = r#"{"media_url":"https://example.com/img.jpg"}"#;
    let result: MarketingImageUploadResult = serde_json::from_str(json).unwrap();
    assert_eq!(
        result.media_url.as_deref(),
        Some("https://example.com/img.jpg")
    );
}

/// 商户视频上传结果 serde（对应 Java `VideoUploadResult`：`media_id`）。
/// 对应 Java: MerchantMediaServiceImplTest.testUploadVideo
#[test]
fn test_video_upload_result_serde() {
    let json = r#"{"media_id":"VIDEO-001"}"#;
    let result: VideoUploadResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.media_id.as_deref(), Some("VIDEO-001"));
}

// ═══════════════════════════════════════════════════════════════
// VALUE_ADD: 边界/空值
// ═══════════════════════════════════════════════════════════════

/// 商家转账创建请求默认值。
#[test]
fn test_merchant_transfer_create_request_default() {
    let request = TransferCreateRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("appid"));
    assert!(json.contains("\"transfer_detail_list\":[]"));
}

/// 品牌转账批次创建请求默认值。
#[test]
fn test_brand_transfer_batches_request_default() {
    let request = BrandTransferBatchesRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("brand_id"));
    assert!(json.contains("\"detail_list\":[]"));
}

/// 海关申报请求默认值。
#[test]
fn test_declaration_request_default() {
    let request = DeclarationRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("appid"));
    assert!(!json.contains("customs"));
}

/// 海关申报查询结果默认值。
#[test]
fn test_declaration_query_result_default() {
    let result: DeclarationQueryResult = serde_json::from_str("{}").unwrap();
    assert!(result.data.is_empty());
}

/// 商家转账批次查询结果默认值。
#[test]
fn test_merchant_batches_query_result_default() {
    let result: BatchesQueryResult = serde_json::from_str("{}").unwrap();
    assert!(result.transfer_batch.is_none());
    assert!(result.transfer_detail_list.is_empty());
}

/// 商家转账明细查询结果默认值。
#[test]
fn test_merchant_details_query_result_default() {
    let result: DetailsQueryResult = serde_json::from_str("{}").unwrap();
    assert!(result.detail_id.is_none());
    assert_eq!(result.transfer_amount, None);
}

/// 品牌批次查询结果默认值。
#[test]
fn test_brand_batches_query_result_default() {
    let result: BrandBatchesQueryResult = serde_json::from_str("{}").unwrap();
    assert!(result.batch_no.is_none());
    assert!(result.detail_list.is_empty());
}

/// 品牌明细查询结果默认值。
#[test]
fn test_brand_details_query_result_default() {
    let result: BrandDetailsQueryResult = serde_json::from_str("{}").unwrap();
    assert!(result.detail_no.is_none());
    assert_eq!(result.amount, None);
}

/// 海关重报请求默认值。
#[test]
fn test_redeclare_request_default() {
    let request = RedeclareRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("transaction_id"));
    assert!(!json.contains("customs"));
}

/// 海关证书校验结果默认值。
#[test]
fn test_verify_certificate_result_default() {
    let result: VerifyCertificateResult = serde_json::from_str("{}").unwrap();
    assert!(result.certificate_check_result.is_none());
    assert!(result.mchid.is_none());
}
