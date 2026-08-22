#![allow(clippy::field_reassign_with_default)]
//! Phase 3 P2 扩展: pay 营销（Marketing）+ 电商（Ecommerce）子域 Bean 测试。
//!
//! 镜像 Java:
//! - `MarketingFavorServiceImplTest`（代金券批次创建/查询/暂停/重启/使用/领取）
//! - `MarketingBusiFavorServiceImplTest`（商家券创建/关联/核销/查询）
//! - `EcommerceServiceImplTest`（电商退款/退款查询/结算/资金余额/补差）
//! - `WxPayCouponServiceImplTest`（发放代金券/查询批次/查询券信息）
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java @Test 的序列化/反序列化断言
//! - RUST_OBLIGATION: serde skip_serializing_if、Option 语义、嵌套结构
//! - VALUE_ADD: 空值/边界/默认值路径

use wx_rust_pay::bean::coupon::*;
use wx_rust_pay::bean::ecommerce;
use wx_rust_pay::bean::marketing::*;

// ═══════════════════════════════════════════════════════════════
// 1. 代金券批次创建请求（SOURCE_PARITY:
//    Java MarketingFavorServiceImplTest.testCreateFavorStocks）
// ═══════════════════════════════════════════════════════════════

/// 代金券批次创建请求 serde（对应 Java `FavorStocksCreateRequest`：
/// `stock_name`/`comment`/`belong_merchant`/`available_begin_time`/
/// `available_end_time`/`stock_use_rule`/`coupon_use_rule`/`no_cash`/
/// `stock_type`/`out_request_no`）。
/// 对应 Java: MarketingFavorServiceImplTest.testCreateFavorStocks
#[test]
fn test_favor_stocks_create_request_serde() {
    let json = r#"{
        "stock_name":"测试代金券",
        "comment":"全场通用",
        "belong_merchant":"mch123",
        "available_begin_time":"2024-01-01T00:00:00+08:00",
        "available_end_time":"2024-12-31T23:59:59+08:00",
        "stock_use_rule":{
            "max_coupons":1000,
            "max_amount":500000,
            "max_amount_by_day":50000,
            "max_coupons_per_user":3,
            "natural_person_limit":false,
            "prevent_api_abuse":true
        },
        "coupon_use_rule":{
            "fixed_normal_coupon":{"coupon_amount":500,"transaction_minimum":1000},
            "goods_tag":["wx1234"],
            "trade_type":["MICRO","NATIVE"]
        },
        "no_cash":false,
        "stock_type":"NORMAL",
        "out_request_no":"REQ-001"
    }"#;
    let request: FavorStocksCreateRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.stock_name.as_deref(), Some("测试代金券"));
    assert_eq!(request.belong_merchant.as_deref(), Some("mch123"));
    assert_eq!(request.stock_type, "NORMAL");
    assert_eq!(request.out_request_no.as_deref(), Some("REQ-001"));
    let rule = request.stock_use_rule.as_ref().unwrap();
    assert_eq!(rule.max_coupons, Some(1000));
    assert_eq!(rule.max_coupons_per_user, Some(3));
    let coupon = request.coupon_use_rule.as_ref().unwrap();
    let fixed = coupon.fixed_normal_coupon.as_ref().unwrap();
    assert_eq!(fixed.coupon_amount, Some(500));
    assert_eq!(fixed.transaction_minimum, Some(1000));
}

/// 代金券批次创建请求空值跳过（对应 Java Gson skip null）。
/// 对应 Java: MarketingFavorServiceImplTest (VALUE_ADD)
#[test]
fn test_favor_stocks_create_request_none_skipping() {
    let request = FavorStocksCreateRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("stock_name"));
    assert!(!json.contains("belong_merchant"));
    assert!(json.contains("\"stock_type\":\"\""));
}

/// 代金券批次创建结果 serde。
/// 对应 Java: MarketingFavorServiceImplTest (result)
#[test]
fn test_favor_stocks_create_result_serde() {
    let json = r#"{"stock_id":"STOCK-001","create_time":"2024-01-01T00:00:00+08:00"}"#;
    let result: FavorStocksCreateResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.stock_id.as_deref(), Some("STOCK-001"));
}

// ═══════════════════════════════════════════════════════════════
// 2. 代金券批次查询结果（SOURCE_PARITY:
//    Java MarketingFavorServiceImplTest.testQueryFavorStocks）
// ═══════════════════════════════════════════════════════════════

/// 代金券批次详情 serde（对应 Java `FavorStocksGetResult`：
/// `stock_id`/`stock_name`/`status`/`create_time`/`description`/
/// `stock_use_rule`/`available_begin_time`/`available_end_time`）。
/// 对应 Java: MarketingFavorServiceImplTest.testQueryFavorStocks
#[test]
fn test_favor_stocks_get_result_serde() {
    let json = r#"{
        "stock_id":"STOCK-001",
        "stock_name":"测试代金券",
        "status":"RUNNING",
        "create_time":"2024-01-01T00:00:00+08:00",
        "description":"全场通用",
        "available_begin_time":"2024-01-01T00:00:00+08:00",
        "available_end_time":"2024-12-31T23:59:59+08:00",
        "stock_use_rule":{"max_coupons":1000,"max_amount":500000,"max_coupons_per_user":3},
        "no_cash":false,
        "send_count_information":{"total_send_count":100,"total_send_amount":50000}
    }"#;
    let result: FavorStocksGetResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.stock_id.as_deref(), Some("STOCK-001"));
    assert_eq!(result.status.as_deref(), Some("RUNNING"));
}

/// 代金券批次查询请求 serde（对应 Java `FavorStocksQueryRequest`：
/// `stock_creator_mchid`/`create_start_time`/`create_end_time`/`status`/
/// `offset`/`limit`）。
/// 对应 Java: MarketingFavorServiceImplTest.testQueryFavorStocks
#[test]
fn test_favor_stocks_query_request_serde() {
    let json = r#"{"stock_creator_mchid":"mch123","create_start_time":"2024-01-01","create_end_time":"2024-12-31","status":"RUNNING","offset":0,"limit":10}"#;
    let request: FavorStocksQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.stock_creator_mchid.as_deref(), Some("mch123"));
    assert_eq!(request.status.as_deref(), Some("RUNNING"));
}

/// 代金券批次查询结果 serde（对应 Java `FavorStocksQueryResult`：
/// `total_count`/`data`/`limit`/`offset`）。
#[test]
fn test_favor_stocks_query_result_serde() {
    let json = r#"{"total_count":1,"data":[{"stock_id":"STOCK-001","stock_name":"测试","status":"RUNNING"}],"limit":10,"offset":0}"#;
    let result: FavorStocksQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.total_count, Some(1));
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data[0].stock_id.as_deref(), Some("STOCK-001"));
}

// ═══════════════════════════════════════════════════════════════
// 3. 代金券暂停/重启（SOURCE_PARITY:
//    Java MarketingFavorServiceImplTest.testPauseFavorStocks）
// ═══════════════════════════════════════════════════════════════

/// 代金券暂停结果 serde。
/// 对应 Java: MarketingFavorServiceImplTest.testPauseFavorStocks
#[test]
fn test_favor_stocks_pause_result_serde() {
    let json = r#"{"stock_id":"STOCK-001","pause_time":"2024-06-01T00:00:00+08:00"}"#;
    let result: FavorStocksPauseResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.stock_id.as_deref(), Some("STOCK-001"));
}

/// 代金券重启结果 serde。
/// 对应 Java: MarketingFavorServiceImplTest.testRestartFavorStocks
#[test]
fn test_favor_stocks_restart_result_serde() {
    let json = r#"{"stock_id":"STOCK-001","restart_time":"2024-06-02T00:00:00+08:00"}"#;
    let result: FavorStocksRestartResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.stock_id.as_deref(), Some("STOCK-001"));
}

// ═══════════════════════════════════════════════════════════════
// 4. 代金券领取/发放（SOURCE_PARITY:
//    Java MarketingFavorServiceImplTest.testCreateFavorCoupons）
// ═══════════════════════════════════════════════════════════════

/// 代金券领取请求 serde（对应 Java `FavorCouponsCreateRequest`：
/// `stock_id`/`out_request_no`/`appid`/`stock_creator_mchid`/
/// `coupon_value`/`coupon_minimum`）。
/// 对应 Java: MarketingFavorServiceImplTest.testCreateFavorCoupons
#[test]
fn test_favor_coupons_create_request_serde() {
    let json = r#"{
        "stock_id":"STOCK-001",
        "out_request_no":"REQ-001",
        "appid":"wx1234",
        "stock_creator_mchid":"mch123",
        "coupon_value":500,
        "coupon_minimum":1000
    }"#;
    let request: FavorCouponsCreateRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.stock_id.as_deref(), Some("STOCK-001"));
    assert_eq!(request.appid.as_deref(), Some("wx1234"));
    assert_eq!(request.coupon_value, Some(500));
}

/// 代金券领取结果 serde。
#[test]
fn test_favor_coupons_create_result_serde() {
    let json = r#"{"coupon_id":"COUPON-001","stock_id":"STOCK-001"}"#;
    let result: FavorCouponsCreateResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.coupon_id.as_deref(), Some("COUPON-001"));
}

// ═══════════════════════════════════════════════════════════════
// 5. 代金券查询（SOURCE_PARITY:
//    Java MarketingFavorServiceImplTest.testQueryFavorCoupons）
// ═══════════════════════════════════════════════════════════════

/// 代金券查询请求 serde（对应 Java `FavorCouponsQueryRequest`：
/// `openid`/`appid`/`stock_id`/`status`/`creator_mchid`/`offset`/`limit`）。
/// 对应 Java: MarketingFavorServiceImplTest.testQueryFavorCoupons
#[test]
fn test_favor_coupons_query_request_serde() {
    let json = r#"{"openid":"ox123","appid":"wx1234","stock_id":"STOCK-001","status":"SENDED","offset":0,"limit":20}"#;
    let request: FavorCouponsQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.openid.as_deref(), Some("ox123"));
    assert_eq!(request.stock_id.as_deref(), Some("STOCK-001"));
}

/// 代金券查询结果 serde（对应 Java `FavorCouponsQueryResult`：
/// `total_count`/`data`/`limit`/`offset`）。
#[test]
fn test_favor_coupons_query_result_serde() {
    let json = r#"{"total_count":1,"data":[{"stock_id":"STOCK-001","coupon_id":"COUPON-001","coupon_name":"测试券","status":"SENDED"}],"limit":20,"offset":0}"#;
    let result: FavorCouponsQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.total_count, Some(1));
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data[0].coupon_id.as_deref(), Some("COUPON-001"));
}

// ═══════════════════════════════════════════════════════════════
// 6. 代金券详情（SOURCE_PARITY:
//    Java MarketingFavorServiceImplTest.testGetFavorCoupons）
// ═══════════════════════════════════════════════════════════════

/// 代金券详情 serde（对应 Java `FavorCouponsGetResult`：
/// `stock_id`/`coupon_id`/`coupon_name`/`status`/`description`/
/// `create_time`/`coupon_type`/`no_cash`/`available_begin_time`/
/// `available_end_time`/`normal_coupon_information`/`cut_to_message`）。
/// 对应 Java: MarketingFavorServiceImplTest.testGetFavorCoupons
#[test]
fn test_favor_coupons_get_result_serde() {
    let json = r#"{
        "stock_creator_mchid":"mch123",
        "stock_id":"STOCK-001",
        "coupon_id":"COUPON-001",
        "coupon_name":"满减券",
        "status":"SENDED",
        "description":"全场满10减5",
        "create_time":"2024-01-01T00:00:00+08:00",
        "coupon_type":"NORMAL",
        "no_cash":false,
        "available_begin_time":"2024-01-01T00:00:00+08:00",
        "available_end_time":"2024-12-31T23:59:59+08:00",
        "normal_coupon_information":{"coupon_amount":500,"transaction_minimum":1000}
    }"#;
    let result: FavorCouponsGetResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.coupon_id.as_deref(), Some("COUPON-001"));
    assert_eq!(result.status.as_deref(), Some("SENDED"));
    assert_eq!(result.coupon_type.as_deref(), Some("NORMAL"));
    let info = result.normal_coupon_information.as_ref().unwrap();
    assert_eq!(info.coupon_amount, Some(500));
    assert_eq!(info.transaction_minimum, Some(1000));
}

/// CutToMessage serde（对应 Java `FavorCouponsGetResult.CutToMessage`）。
#[test]
fn test_cut_to_message_serde() {
    let json = r#"{"single_price_max":10000,"cut_to_price":5000}"#;
    let msg: CutToMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.single_price_max, Some(10000));
    assert_eq!(msg.cut_to_price, Some(5000));
}

// ═══════════════════════════════════════════════════════════════
// 7. 代金券核销（SOURCE_PARITY:
//    Java MarketingFavorServiceImplTest.testUseFavorCoupons）
// ═══════════════════════════════════════════════════════════════

/// 代金券核销结果 serde（对应 Java `FavorCouponsUseResult`：
/// `stock_id`/`coupon_id`/`coupon_name`/`status`/`consume_information`）。
/// 对应 Java: MarketingFavorServiceImplTest.testUseFavorCoupons
#[test]
fn test_favor_coupons_use_result_serde() {
    let json = r#"{
        "stock_creator_mchid":"mch123",
        "stock_id":"STOCK-001",
        "coupon_id":"COUPON-001",
        "coupon_name":"满减券",
        "status":"USED",
        "consume_information":{
            "consume_time":"2024-01-15T12:00:00+08:00",
            "consume_mchid":"mch123",
            "transaction_id":"4200001234",
            "goods_detail":[{"goods_id":"G1","quantity":1,"price":500,"discount_amount":500}]
        }
    }"#;
    let result: FavorCouponsUseResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.status.as_deref(), Some("USED"));
    let consume = result.consume_information.as_ref().unwrap();
    assert_eq!(consume.transaction_id.as_deref(), Some("4200001234"));
    assert_eq!(consume.goods_detail.len(), 1);
    assert_eq!(consume.goods_detail[0].goods_id.as_deref(), Some("G1"));
}

// ═══════════════════════════════════════════════════════════════
// 8. 商家券（SOURCE_PARITY:
//    Java MarketingBusiFavorServiceImplTest）
// ═══════════════════════════════════════════════════════════════

/// 商家券批次创建请求 serde（对应 Java `BusiFavorStocksCreateRequest`）。
/// 对应 Java: MarketingBusiFavorServiceImplTest.testCreateBusiFavorStocks
#[test]
fn test_busi_favor_stocks_create_request_serde() {
    let json = r#"{
        "stock_name":"商家满减券",
        "comment":"新客专享",
        "belong_merchant":"mch123",
        "available_begin_time":"2024-01-01T00:00:00+08:00",
        "available_end_time":"2024-12-31T23:59:59+08:00",
        "stock_use_rule":{"max_coupons":500,"max_coupons_per_user":1}
    }"#;
    let request: BusiFavorStocksCreateRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.stock_name.as_deref(), Some("商家满减券"));
}

/// 商家券批次创建结果 serde。
#[test]
fn test_busi_favor_stocks_create_result_serde() {
    let json = r#"{"stock_id":"BSTOCK-001"}"#;
    let result: BusiFavorStocksCreateResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.stock_id.as_deref(), Some("BSTOCK-001"));
}

/// 商家券批次详情 serde（对应 Java `BusiFavorStocksGetResult`：
/// `stock_name`/`belong_merchant`/`comment`/`stock_type`/`coupon_use_rule`/
/// `stock_send_rule`/`out_request_no`）。
/// 对应 Java: MarketingBusiFavorServiceImplTest (result)
#[test]
fn test_busi_favor_stocks_get_result_serde() {
    let json = r#"{"stock_name":"商家满减券","belong_merchant":"mch123","comment":"新客专享","stock_type":"NORMAL","out_request_no":"REQ-001"}"#;
    let result: BusiFavorStocksGetResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.stock_name.as_deref(), Some("商家满减券"));
    assert_eq!(result.belong_merchant.as_deref(), Some("mch123"));
    assert_eq!(result.stock_type, "NORMAL");
}

/// 商家券批次预算查询结果 serde（对应 Java `BusiFavorStocksBudgetResult`：
/// `max_coupons`/`max_coupons_by_day`）。
/// 对应 Java: MarketingBusiFavorServiceImplTest.testQueryBudget
#[test]
fn test_busi_favor_stocks_budget_result_serde() {
    let json = r#"{"max_coupons":500,"max_coupons_by_day":50}"#;
    let result: BusiFavorStocksBudgetResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.max_coupons, Some(500));
    assert_eq!(result.max_coupons_by_day, Some(50));
}

/// 商家券关联请求 serde（对应 Java `BusiFavorCouponsAssociateRequest`）。
/// 对应 Java: MarketingBusiFavorServiceImplTest.testAssociateBusiFavorCoupons
#[test]
fn test_busi_favor_coupons_associate_request_serde() {
    let json = r#"{"stock_id":"BSTOCK-001","coupon_code":"CODE-001"}"#;
    let request: BusiFavorCouponsAssociateRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.stock_id.as_deref(), Some("BSTOCK-001"));
}

/// 商家券关联结果 serde（对应 Java `BusiFavorCouponsAssociateResult`：
/// `wechatpay_associate_time`/`wechatpay_disassociate_time`）。
/// 对应 Java: MarketingBusiFavorServiceImplTest.testAssociateBusiFavorCoupons
#[test]
fn test_busi_favor_coupons_associate_result_serde() {
    let json = r#"{"wechatpay_associate_time":"2024-01-01T00:00:00+08:00","wechatpay_disassociate_time":"2024-02-01T00:00:00+08:00"}"#;
    let result: BusiFavorCouponsAssociateResult = serde_json::from_str(json).unwrap();
    assert_eq!(
        result.wechatpay_associate_time.as_deref(),
        Some("2024-01-01T00:00:00+08:00")
    );
}

/// 商家券核销结果 serde（对应 Java `BusiFavorCouponsUseResult`：
/// `stock_id`/`openid`/`wechatpay_use_time`）。
/// 对应 Java: MarketingBusiFavorServiceImplTest.testUseBusiFavorCoupons
#[test]
fn test_busi_favor_coupons_use_result_serde() {
    let json = r#"{"stock_id":"BSTOCK-001","openid":"ox123","wechatpay_use_time":"2024-01-15T12:00:00+08:00"}"#;
    let result: BusiFavorCouponsUseResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.stock_id.as_deref(), Some("BSTOCK-001"));
    assert_eq!(result.openid.as_deref(), Some("ox123"));
}

/// 商家券查询单用户券请求 serde。
/// 对应 Java: MarketingBusiFavorServiceImplTest.testQueryOneUserBusiFavorCoupons
#[test]
fn test_busi_favor_query_one_user_coupons_request_serde() {
    let json = r#"{"stock_id":"BSTOCK-001","openid":"ox123","appid":"wx1234"}"#;
    let request: BusiFavorQueryOneUserCouponsRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.openid.as_deref(), Some("ox123"));
}

/// 商家券查询单用户券结果 serde。
#[test]
fn test_busi_favor_query_one_user_coupons_result_serde() {
    let json = r#"{"stock_id":"BSTOCK-001","openid":"ox123","coupon_code":"CODE-001"}"#;
    let result: BusiFavorQueryOneUserCouponsResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.coupon_code.as_deref(), Some("CODE-001"));
}

// ═══════════════════════════════════════════════════════════════
// 9. 电商退款（SOURCE_PARITY:
//    Java EcommerceServiceImplTest.testRefunds）
// ═══════════════════════════════════════════════════════════════

/// 电商退款请求 serde（对应 Java `ecommerce.RefundsRequest`：
/// `sub_mchid`/`sp_appid`/`transaction_id`/`out_refund_no`/`reason`/
/// `amount`/`notify_url`）。
/// 对应 Java: EcommerceServiceImplTest.testRefunds
#[test]
fn test_ecommerce_refunds_request_serde() {
    let json = r#"{
        "sub_mchid":"sub123",
        "sp_appid":"wx1234",
        "sub_appid":"wx5678",
        "transaction_id":"4200001234",
        "out_trade_no":"ORDER-001",
        "out_refund_no":"REFUND-001",
        "reason":"用户申请退款",
        "amount":{"refund":500,"total":1000,"currency":"CNY"},
        "notify_url":"https://example.com/notify",
        "refund_account":"REFUND_SOURCE_RECHARGE_FUNDS",
        "funds_account":"AVAILABLE"
    }"#;
    let request: ecommerce::RefundsRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.sub_mchid.as_deref(), Some("sub123"));
    assert_eq!(request.out_refund_no.as_deref(), Some("REFUND-001"));
    assert_eq!(request.reason.as_deref(), Some("用户申请退款"));
    let amount = request.amount.as_ref().unwrap();
    assert_eq!(amount.refund, Some(500));
    assert_eq!(amount.total, Some(1000));
    assert_eq!(amount.currency.as_deref(), Some("CNY"));
}

/// 电商退款结果 serde。
/// 对应 Java: EcommerceServiceImplTest (result)
#[test]
fn test_ecommerce_refunds_result_serde() {
    let json = r#"{
        "refund_id":"RF-001",
        "out_refund_no":"REFUND-001",
        "create_time":"2024-01-01T12:00:00+08:00",
        "amount":{"refund":500,"payer_refund":500,"discount_refund":0,"currency":"CNY"}
    }"#;
    let result: ecommerce::RefundsResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.refund_id.as_deref(), Some("RF-001"));
    assert_eq!(result.out_refund_no.as_deref(), Some("REFUND-001"));
    let amount = result.amount.as_ref().unwrap();
    assert_eq!(amount.payer_refund, Some(500));
}

// ═══════════════════════════════════════════════════════════════
// 10. 电商退款查询（SOURCE_PARITY:
//     Java EcommerceServiceImplTest.testRefundQuery）
// ═══════════════════════════════════════════════════════════════

/// 电商退款查询结果 serde（对应 Java `ecommerce.RefundQueryResult`：
/// `refund_id`/`out_refund_no`/`transaction_id`/`channel`/
/// `user_received_account`/`success_time`/`status`/`amount`）。
/// 对应 Java: EcommerceServiceImplTest.testRefundQuery
#[test]
fn test_ecommerce_refund_query_result_serde() {
    let json = r#"{
        "refund_id":"RF-001",
        "out_refund_no":"REFUND-001",
        "transaction_id":"4200001234",
        "out_trade_no":"ORDER-001",
        "channel":"ORIGINAL",
        "user_received_account":"用户零钱",
        "success_time":"2024-01-02T00:00:00+08:00",
        "create_time":"2024-01-01T12:00:00+08:00",
        "status":"SUCCESS",
        "amount":{"refund":"500","payer_refund":"500","discount_refund":0,"currency":"CNY"},
        "refund_account":"REFUND_SOURCE_RECHARGE_FUNDS",
        "funds_account":"AVAILABLE"
    }"#;
    let result: ecommerce::RefundQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.refund_id.as_deref(), Some("RF-001"));
    assert_eq!(result.status.as_deref(), Some("SUCCESS"));
    assert_eq!(result.channel.as_deref(), Some("ORIGINAL"));
    assert_eq!(result.user_received_account.as_deref(), Some("用户零钱"));
    let amount = result.amount.as_ref().unwrap();
    assert_eq!(amount.refund.as_deref(), Some("500"));
    assert_eq!(amount.discount_refund, Some(0));
}

// ═══════════════════════════════════════════════════════════════
// 11. 电商结算（SOURCE_PARITY:
//     Java EcommerceServiceImplTest.testSettlement）
// ═══════════════════════════════════════════════════════════════

/// 电商结算请求 serde。
/// 对应 Java: EcommerceServiceImplTest.testSettlement
#[test]
fn test_ecommerce_settlement_request_serde() {
    let json = r#"{
        "account_type":"BASIC",
        "account_bank":"工商银行",
        "bank_address_code":"110000",
        "bank_name":"中国工商银行",
        "bank_branch_id":"1001",
        "account_number":"1234567890"
    }"#;
    let request: ecommerce::SettlementRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.account_type.as_deref(), Some("BASIC"));
    assert_eq!(request.account_bank.as_deref(), Some("工商银行"));
}

/// 电商结算结果 serde。
#[test]
fn test_ecommerce_settlement_result_serde() {
    let json = r#"{
        "account_type":"BASIC",
        "account_bank":"工商银行",
        "bank_name":"中国工商银行",
        "bank_branch_id":"1001",
        "account_number":"1234****7890",
        "verify_result":"SUCCESS"
    }"#;
    let result: ecommerce::SettlementResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.account_type.as_deref(), Some("BASIC"));
    assert_eq!(result.verify_result.as_deref(), Some("SUCCESS"));
}

// ═══════════════════════════════════════════════════════════════
// 12. 电商资金余额（SOURCE_PARITY:
//     Java EcommerceServiceImplTest.testFundBalance）
// ═══════════════════════════════════════════════════════════════

/// 电商资金余额结果 serde（对应 Java `ecommerce.FundBalanceResult`：
/// `sub_mchid`/`available_amount`/`pending_amount`）。
/// 对应 Java: EcommerceServiceImplTest.testFundBalance
#[test]
fn test_ecommerce_fund_balance_result_serde() {
    let json = r#"{"sub_mchid":"sub123","available_amount":10000,"pending_amount":5000}"#;
    let result: ecommerce::FundBalanceResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.sub_mchid.as_deref(), Some("sub123"));
    assert_eq!(result.available_amount, Some(10000));
    assert_eq!(result.pending_amount, Some(5000));
}

// ═══════════════════════════════════════════════════════════════
// 13. 代金券发放（SOURCE_PARITY:
//     Java WxPayCouponServiceImplTest.testSendCoupon）
// ═══════════════════════════════════════════════════════════════

/// 代金券发放请求 serde（对应 Java `WxPayCouponSendRequest`：
/// `coupon_stock_id`/`openid`/`partner_trade_no`/`appid`/`mch_id`）。
/// 对应 Java: WxPayCouponServiceImplTest.testSendCoupon
#[test]
fn test_coupon_send_request_serde() {
    let json = r#"{
        "coupon_stock_id":"STOCK-001",
        "openid":"ox123",
        "partner_trade_no":"TRADE-001",
        "appid":"wx1234",
        "mch_id":"mch123",
        "nonce_str":"nonce123"
    }"#;
    let request: WxPayCouponSendRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.coupon_stock_id.as_deref(), Some("STOCK-001"));
    assert_eq!(request.openid.as_deref(), Some("ox123"));
}

/// 代金券发放结果 serde。
#[test]
fn test_coupon_send_result_serde() {
    let json = r#"{"return_code":"SUCCESS","result_code":"SUCCESS","coupon_stock_id":"STOCK-001"}"#;
    let result: WxPayCouponSendResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.return_code.as_deref(), Some("SUCCESS"));
}

// ═══════════════════════════════════════════════════════════════
// 14. 代金券批次查询（SOURCE_PARITY:
//     Java WxPayCouponServiceImplTest.testQueryCouponStock）
// ═══════════════════════════════════════════════════════════════

/// 代金券批次查询请求 serde。
/// 对应 Java: WxPayCouponServiceImplTest.testQueryCouponStock
#[test]
fn test_coupon_stock_query_request_serde() {
    let json = r#"{"coupon_stock_id":"STOCK-001","nonce_str":"nonce123"}"#;
    let request: WxPayCouponStockQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.coupon_stock_id.as_deref(), Some("STOCK-001"));
}

/// 代金券批次查询结果 serde。
#[test]
fn test_coupon_stock_query_result_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "result_code":"SUCCESS",
        "coupon_stock_id":"STOCK-001",
        "name":"测试券",
        "status":"SENDED",
        "coupon_total":1000
    }"#;
    let result: WxPayCouponStockQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.return_code.as_deref(), Some("SUCCESS"));
    assert_eq!(result.coupon_stock_id.as_deref(), Some("STOCK-001"));
}

// ═══════════════════════════════════════════════════════════════
// 15. 代金券信息查询（SOURCE_PARITY:
//     Java WxPayCouponServiceImplTest.testQueryCouponInfo）
// ═══════════════════════════════════════════════════════════════

/// 代金券信息查询请求 serde。
/// 对应 Java: WxPayCouponServiceImplTest.testQueryCouponInfo
#[test]
fn test_coupon_info_query_request_serde() {
    let json =
        r#"{"coupon_id":"COUPON-001","appid":"wx1234","mch_id":"mch123","nonce_str":"nonce123"}"#;
    let request: WxPayCouponInfoQueryRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.coupon_id.as_deref(), Some("COUPON-001"));
}

/// 代金券信息查询结果 serde。
#[test]
fn test_coupon_info_query_result_serde() {
    let json = r#"{
        "return_code":"SUCCESS",
        "result_code":"SUCCESS",
        "coupon_id":"COUPON-001",
        "coupon_name":"测试券",
        "status":"USED"
    }"#;
    let result: WxPayCouponInfoQueryResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.return_code.as_deref(), Some("SUCCESS"));
    assert_eq!(result.coupon_id.as_deref(), Some("COUPON-001"));
}

// ═══════════════════════════════════════════════════════════════
// VALUE_ADD: 边界/空值
// ═══════════════════════════════════════════════════════════════

/// 代金券批次创建请求默认值序列化。
#[test]
fn test_favor_stocks_create_request_default() {
    let request = FavorStocksCreateRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"stock_type\":\"\""));
    assert!(!json.contains("stock_name"));
}

/// 电商退款请求默认值序列化。
#[test]
fn test_ecommerce_refunds_request_default() {
    let request = ecommerce::RefundsRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("sub_mchid"));
    assert!(!json.contains("transaction_id"));
}

/// 代金券批次查询请求默认值序列化。
#[test]
fn test_favor_stocks_query_request_default() {
    let request = FavorStocksQueryRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("stock_creator_mchid"));
    assert!(!json.contains("status"));
}

/// 商家券批次创建请求默认值序列化。
#[test]
fn test_busi_favor_stocks_create_request_default() {
    let request = BusiFavorStocksCreateRequest::default();
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("stock_name"));
}

/// 代金券批次详情默认值。
#[test]
fn test_favor_coupons_get_result_default() {
    let result: FavorCouponsGetResult = serde_json::from_str("{}").unwrap();
    assert!(result.stock_id.is_none());
    assert!(result.coupon_id.is_none());
    assert!(result.normal_coupon_information.is_none());
    assert!(result.cut_to_message.is_none());
}

/// 电商退款结果默认值。
#[test]
fn test_ecommerce_refunds_result_default() {
    let result: ecommerce::RefundsResult = serde_json::from_str("{}").unwrap();
    assert!(result.refund_id.is_none());
    assert!(result.amount.is_none());
    assert!(result.promotion_detail.is_empty());
}

/// 代金券核销结果默认值。
#[test]
fn test_favor_coupons_use_result_default() {
    let result: FavorCouponsUseResult = serde_json::from_str("{}").unwrap();
    assert!(result.stock_id.is_none());
    assert!(result.consume_information.is_none());
}

/// 代金券核销结果内嵌默认值。
#[test]
fn test_favor_coupons_use_result_empty_goods() {
    let json = r#"{"consume_information":{"goods_detail":[]}}"#;
    let result: FavorCouponsUseResult = serde_json::from_str(json).unwrap();
    let consume = result.consume_information.as_ref().unwrap();
    assert!(consume.goods_detail.is_empty());
}

/// 电商结算结果默认值。
#[test]
fn test_ecommerce_settlement_result_default() {
    let result: ecommerce::SettlementResult = serde_json::from_str("{}").unwrap();
    assert!(result.account_type.is_none());
    assert!(result.verify_result.is_none());
}
