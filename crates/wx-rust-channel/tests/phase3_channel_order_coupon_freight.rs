//! Phase 3 P2 扩展: channel 订单边缘/优惠券/运费模板 Bean 测试。
//!
//! 镜像 Java:
//! - `WxChannelOrderServiceImplTest`（订单价格/发货/收货地址边缘）
//! - `WxChannelCouponServiceImplTest`（优惠券详情/列表/用户券）
//! - `WxChannelFreightTemplateServiceImplTest`（运费模板详情/列表）
//! - `WxChannelAfterSaleServiceImplTest`（售后详情/换货/物流信息边缘）
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java @Test 的序列化/反序列化断言
//! - RUST_OBLIGATION: serde rename、default 语义、嵌套结构
//! - VALUE_ADD: 空值/边界/默认值路径
//!
//! 注意: channel bean 使用非 Option 类型 + serde(default)，字段默认为空字符串/0/false。

use wx_rust_channel::bean::after::{
    AfterSaleAcceptParam, AfterSaleDetail as AfterSaleDetailInfo, AfterSaleIdParam,
    AfterSaleInfoResponse, AfterSaleListParam, AfterSaleReasonResponse, AfterSaleRejectParam,
};
use wx_rust_channel::bean::coupon::*;
use wx_rust_channel::bean::freight::*;
use wx_rust_channel::bean::order::*;

// ═══════════════════════════════════════════════════════════════
// 1. 订单支付信息（SOURCE_PARITY:
//    Java WxChannelOrderServiceImplTest (pay info)）
// ═══════════════════════════════════════════════════════════════

/// 订单支付信息 serde（对应 Java `OrderPayInfo`：
/// `payment_method`/`pay_time`/`transaction_id`）。
/// 对应 Java: WxChannelOrderServiceImplTest (pay info)
#[test]
fn test_order_pay_info_serde() {
    let json = r#"{
        "payment_method":1,
        "pay_time":1662480000,
        "transaction_id":"4200001234"
    }"#;
    let info: OrderPayInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.payment_method, 1);
    assert_eq!(info.pay_time, 1662480000);
    assert_eq!(info.transaction_id, "4200001234");
}

/// 订单支付信息默认值。
#[test]
fn test_order_pay_info_default() {
    let info = OrderPayInfo::default();
    assert_eq!(info.payment_method, 0);
    assert_eq!(info.pay_time, 0);
    assert!(info.transaction_id.is_empty());
}

// ═══════════════════════════════════════════════════════════════
// 2. 订单详情信息（SOURCE_PARITY:
//    Java WxChannelOrderServiceImplTest (detail info)）
// ═══════════════════════════════════════════════════════════════

/// 订单详情信息 serde（对应 Java `OrderDetailInfo`：
/// `product_infos`/`pay_info`/`price_info`/`delivery_info`/`coupon_info`/
/// `ext_info`/`commission_infos`/`sharer_info`/`settle_info`）。
/// 对应 Java: WxChannelOrderServiceImplTest (detail info)
#[test]
fn test_order_detail_info_serde() {
    let json = r#"{
        "product_infos":[],
        "price_info":{"order_price":10000,"freight":1000},
        "pay_info":{"payment_method":1,"pay_time":1662480000,"transaction_id":"4200001234"}
    }"#;
    let info: OrderDetailInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.price_info.order_price, 10000);
    assert_eq!(info.price_info.freight, 1000);
    assert_eq!(info.pay_info.transaction_id, "4200001234");
}

// ═══════════════════════════════════════════════════════════════
// 3. 订单信息（SOURCE_PARITY:
//    Java WxChannelOrderServiceImplTest (order info)）
// ═══════════════════════════════════════════════════════════════

/// 订单信息 serde（对应 Java `OrderInfo`：`order_id`/`status`/`openid`/
/// `order_detail`/`aftersale_detail`/`create_time`(i32)/`update_time`(i32)）。
/// 对应 Java: WxChannelOrderServiceImplTest (order info)
#[test]
fn test_order_info_serde() {
    let json = r#"{
        "order_id":"ORDER-001",
        "status":1,
        "openid":"ox123",
        "unionid":"union123",
        "order_detail":{"product_infos":[],"price_info":{"order_price":5000}},
        "aftersale_detail":{"desc":"","cancel_time":0},
        "is_present":false,
        "create_time":1662480,
        "update_time":1662481
    }"#;
    let info: OrderInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.order_id, "ORDER-001");
    assert_eq!(info.status, 1);
    assert_eq!(info.openid, "ox123");
    assert_eq!(info.create_time, 1662480);
    assert!(!info.present);
}

/// 订单信息默认值。
#[test]
fn test_order_info_default() {
    let info = OrderInfo::default();
    assert!(info.order_id.is_empty());
    assert_eq!(info.status, 0);
    assert!(!info.present);
}

// ═══════════════════════════════════════════════════════════════
// 4. 售后详情信息（SOURCE_PARITY:
//    Java WxChannelAfterSaleServiceImplTest (detail)）
// ═══════════════════════════════════════════════════════════════

/// 售后详情 serde（对应 Java `AfterSaleDetail`：`desc`/`receive_product`/
/// `cancel_time`/`prove_imgs`/`tel_number`/`media_id_list`）。
/// 对应 Java: WxChannelAfterSaleServiceImplTest (detail)
#[test]
fn test_after_sale_detail_serde() {
    let json = r#"{
        "desc":"商品有质量问题",
        "receive_product":true,
        "cancel_time":1662480000,
        "prove_imgs":["https://example.com/img1.jpg"],
        "tel_number":"13800138000",
        "media_id_list":["MEDIA-001"]
    }"#;
    let detail: AfterSaleDetailInfo = serde_json::from_str(json).unwrap();
    assert_eq!(detail.desc, "商品有质量问题");
    assert!(detail.receive_product);
    assert_eq!(detail.cancel_time, 1662480000);
    assert_eq!(detail.prove_imgs.len(), 1);
    assert_eq!(detail.tel_number, "13800138000");
    assert_eq!(detail.media_id_list.len(), 1);
}

/// 售后详情默认值。
#[test]
fn test_after_sale_detail_default() {
    let detail = AfterSaleDetailInfo::default();
    assert!(detail.desc.is_empty());
    assert!(!detail.receive_product);
    assert_eq!(detail.cancel_time, 0);
}

// ═══════════════════════════════════════════════════════════════
// 5. 售后信息响应（SOURCE_PARITY:
//    Java WxChannelAfterSaleServiceImplTest (info response)）
// ═══════════════════════════════════════════════════════════════

/// 售后信息响应 serde（对应 Java `AfterSaleInfoResponse`：
/// `errcode`/`errmsg`/`after_sale_order`，`AfterSaleInfo` 含 `after_sale_order_id`/
/// `status`/`order_id`/`openid`/`type`/`create_time`/`update_time`/`reason`）。
/// 对应 Java: WxChannelAfterSaleServiceImplTest (info)
#[test]
fn test_after_sale_info_response_serde() {
    let json = r#"{
        "errcode":0,
        "errmsg":"success",
        "after_sale_order":{
            "after_sale_order_id":"AS-001",
            "status":"DOING",
            "order_id":"ORDER-001",
            "openid":"ox123",
            "type":"REFUND",
            "create_time":1662480000,
            "update_time":1662480100,
            "reason":"质量问题",
            "reason_text":"商品有瑕疵"
        }
    }"#;
    let resp: AfterSaleInfoResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.err_msg, "success");
    assert_eq!(resp.info.after_sale_order_id, "AS-001");
    assert_eq!(resp.info.status, "DOING");
    assert_eq!(resp.info.reason, "质量问题");
}

// ═══════════════════════════════════════════════════════════════
// 6. 售后列表参数（SOURCE_PARITY:
//    Java WxChannelAfterSaleServiceImplTest (list param)）
// ═══════════════════════════════════════════════════════════════

/// 售后列表参数 serde（对应 Java `AfterSaleListParam`：
/// `begin_create_time`/`end_create_time`/`next_key`）。
/// 对应 Java: WxChannelAfterSaleServiceImplTest (list param)
#[test]
fn test_after_sale_list_param_serde() {
    let json = r#"{
        "begin_create_time":1662400000,
        "end_create_time":1662480000,
        "next_key":"next_page_key"
    }"#;
    let param: AfterSaleListParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.begin_create_time, 1662400000);
    assert_eq!(param.end_create_time, 1662480000);
    assert_eq!(param.next_key, "next_page_key");
}

// ═══════════════════════════════════════════════════════════════
// 7. 售后原因（SOURCE_PARITY:
//    Java WxChannelAfterSaleServiceImplTest (reason)）
// ═══════════════════════════════════════════════════════════════

/// 售后原因响应 serde（对应 Java `AfterSaleReasonResponse`：
/// `errcode`/`errmsg`/`reason_list`，每项含 `reason`(i32)/`reason_text`）。
/// 对应 Java: WxChannelAfterSaleServiceImplTest (reason)
#[test]
fn test_after_sale_reason_response_serde() {
    let json = r#"{
        "errcode":0,
        "errmsg":"success",
        "reason_list":[
            {"reason":1,"reason_text":"质量问题"},
            {"reason":2,"reason_text":"发错货"}
        ]
    }"#;
    let resp: AfterSaleReasonResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.reason_list.len(), 2);
    assert_eq!(resp.reason_list[0].reason, 1);
    assert_eq!(resp.reason_list[0].reason_text, "质量问题");
}

// ═══════════════════════════════════════════════════════════════
// 8. 售后接受/拒绝参数（SOURCE_PARITY:
//    Java WxChannelAfterSaleServiceImplTest (accept/reject)）
// ═══════════════════════════════════════════════════════════════

/// 售后接受参数 serde（对应 Java `AfterSaleAcceptParam`：
/// `after_sale_order_id`/`address_id`/`accept_type`）。
/// 对应 Java: WxChannelAfterSaleServiceImplTest (accept)
#[test]
fn test_after_sale_accept_param_serde() {
    let json = r#"{
        "after_sale_order_id":"AS-001",
        "address_id":"ADDR-001",
        "accept_type":1
    }"#;
    let param: AfterSaleAcceptParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.after_sale_order_id, "AS-001");
    assert_eq!(param.accept_type, 1);
}

/// 售后拒绝参数 serde（对应 Java `AfterSaleRejectParam`：
/// `after_sale_order_id`/`reject_reason`/`reject_reason_type`）。
/// 对应 Java: WxChannelAfterSaleServiceImplTest (reject)
#[test]
fn test_after_sale_reject_param_serde() {
    let json = r#"{
        "after_sale_order_id":"AS-001",
        "reject_reason":"不符合退货条件",
        "reject_reason_type":1
    }"#;
    let param: AfterSaleRejectParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.after_sale_order_id, "AS-001");
    assert_eq!(param.reject_reason, "不符合退货条件");
}

/// 售后ID参数 serde（对应 Java `AfterSaleIdParam`）。
#[test]
fn test_after_sale_id_param_serde() {
    let json = r#"{"after_sale_order_id":"AS-001"}"#;
    let param: AfterSaleIdParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.after_sale_order_id, "AS-001");
}

// ═══════════════════════════════════════════════════════════════
// 9. 优惠券信息（SOURCE_PARITY:
//    Java WxChannelCouponServiceImplTest (coupon info)）
// ═══════════════════════════════════════════════════════════════

/// 优惠券信息 serde（对应 Java `CouponInfo`：`coupon_id`/`type`/`status`/
/// `create_time`/`update_time`/`coupon_info`(→`detail`)/`stock_info`）。
/// 对应 Java: WxChannelCouponServiceImplTest (coupon info)
#[test]
fn test_coupon_info_serde() {
    let json = r#"{
        "coupon_id":"COUPON-001",
        "type":1,
        "status":1,
        "create_time":1662480000,
        "update_time":1662480100,
        "coupon_info":{"name":"满减券"},
        "stock_info":{"stock_num":100}
    }"#;
    let info: CouponInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.coupon_id, "COUPON-001");
    assert_eq!(info.r#type, 1);
    assert_eq!(info.status, 1);
    assert_eq!(info.detail.name, "满减券");
}

// ═══════════════════════════════════════════════════════════════
// 10. 优惠券详情（SOURCE_PARITY:
//     Java WxChannelCouponServiceImplTest (detail)）
// ═══════════════════════════════════════════════════════════════

/// 优惠券详情 serde（对应 Java `CouponDetailInfo`：`name`/`valid_info`/
/// `promote_info`/`discount_info`/`ext_info`/`receive_info`）。
/// 对应 Java: WxChannelCouponServiceImplTest (detail)
#[test]
fn test_coupon_detail_info_serde() {
    let json = r#"{
        "name":"满减券",
        "valid_info":{"start_time":1662400000,"end_time":1665000000},
        "promote_info":{},
        "discount_info":{"discount_num":1,"discount_fee":500},
        "ext_info":{},
        "receive_info":{"limit_num":3}
    }"#;
    let info: CouponDetailInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.name, "满减券");
    assert_eq!(info.discount_info.discount_fee, 500);
    assert_eq!(info.discount_info.discount_num, 1);
}

/// 优惠券详情默认值。
#[test]
fn test_coupon_detail_info_default() {
    let info = CouponDetailInfo::default();
    assert!(info.name.is_empty());
}

// ═══════════════════════════════════════════════════════════════
// 11. 优惠券列表参数/响应（SOURCE_PARITY:
//     Java WxChannelCouponServiceImplTest (list)）
// ═══════════════════════════════════════════════════════════════

/// 优惠券列表参数 serde（对应 Java `CouponListParam`：`status`/`page`/
/// `page_size`/`page_ctx`）。
/// 对应 Java: WxChannelCouponServiceImplTest (list param)
#[test]
fn test_coupon_list_param_serde() {
    let json = r#"{"status":1,"page":0,"page_size":20,"page_ctx":"ctx123"}"#;
    let param: CouponListParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.status, 1);
    assert_eq!(param.page, 0);
    assert_eq!(param.page_size, 20);
    assert_eq!(param.page_ctx, "ctx123");
}

/// 优惠券列表响应 serde（对应 Java `CouponListResponse`：
/// `err_code`/`err_msg`/`coupons`/`total_num`/`page_ctx`）。
/// 对应 Java: WxChannelCouponServiceImplTest (list response)
#[test]
fn test_coupon_list_response_serde() {
    let json = r#"{
        "err_code":0,
        "err_msg":"success",
        "coupons":[{"coupon_id":"COUPON-001","user_coupon_id":"UC-001"}],
        "total_num":10,
        "page_ctx":"ctx123"
    }"#;
    let resp: CouponListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.total_num, 10);
    assert_eq!(resp.coupons.len(), 1);
    assert_eq!(resp.coupons[0].coupon_id, "COUPON-001");
}

// ═══════════════════════════════════════════════════════════════
// 12. 优惠券状态/用户券（SOURCE_PARITY:
//     Java WxChannelCouponServiceImplTest (status/user)）
// ═══════════════════════════════════════════════════════════════

/// 优惠券状态参数 serde（对应 Java `CouponStatusParam`：`coupon_id`/`status`）。
/// 对应 Java: WxChannelCouponServiceImplTest (status)
#[test]
fn test_coupon_status_param_serde() {
    let json = r#"{"coupon_id":"COUPON-001","status":2}"#;
    let param: CouponStatusParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.coupon_id, "COUPON-001");
    assert_eq!(param.status, 2);
}

/// 用户优惠券ID信息 serde（对应 Java `UserCouponIdInfo`：
/// `coupon_id`/`user_coupon_id`）。
/// 对应 Java: WxChannelCouponServiceImplTest (user coupon)
#[test]
fn test_user_coupon_id_info_serde() {
    let json = r#"{"user_coupon_id":"UC-001","coupon_id":"COUPON-001"}"#;
    let info: UserCouponIdInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.user_coupon_id, "UC-001");
    assert_eq!(info.coupon_id, "COUPON-001");
}

// ═══════════════════════════════════════════════════════════════
// 13. 运费模板（SOURCE_PARITY:
//     Java WxChannelFreightTemplateServiceImplTest）
// ═══════════════════════════════════════════════════════════════

/// 运费模板 serde（对应 Java `FreightTemplate`：`template_id`/`name`/
/// `valuation_type`/`send_time`/`delivery_type`/`is_default`）。
/// 对应 Java: WxChannelFreightTemplateServiceImplTest (template)
#[test]
fn test_freight_template_serde() {
    let json = r#"{
        "template_id":"FT-001",
        "name":"全国包邮",
        "valuation_type":"1",
        "send_time":"48小时",
        "delivery_type":"1",
        "shipping_method":"express",
        "is_default":true,
        "create_time":1662480000,
        "update_time":1662480100
    }"#;
    let template: FreightTemplate = serde_json::from_str(json).unwrap();
    assert_eq!(template.template_id, "FT-001");
    assert_eq!(template.name, "全国包邮");
    assert!(template.is_default);
}

/// 运费模板默认值。
#[test]
fn test_freight_template_default() {
    let template = FreightTemplate::default();
    assert!(template.template_id.is_empty());
    assert!(template.name.is_empty());
    assert!(!template.is_default);
}

// ═══════════════════════════════════════════════════════════════
// 14. 运费模板列表（SOURCE_PARITY:
//     Java WxChannelFreightTemplateServiceImplTest (list)）
// ═══════════════════════════════════════════════════════════════

/// 运费模板列表参数 serde（对应 Java `TemplateListParam`：`offset`/`limit`）。
/// 对应 Java: WxChannelFreightTemplateServiceImplTest (list param)
#[test]
fn test_template_list_param_serde() {
    let json = r#"{"offset":0,"limit":20}"#;
    let param: TemplateListParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.offset, 0);
    assert_eq!(param.limit, 20);
}

/// 运费模板列表响应 serde（对应 Java `TemplateListResponse`：
/// `errcode`/`errmsg`/`template_id_list`）。
/// 对应 Java: WxChannelFreightTemplateServiceImplTest (list response)
#[test]
fn test_template_list_response_serde() {
    let json = r#"{
        "errcode":0,
        "errmsg":"success",
        "template_id_list":["FT-001","FT-002"]
    }"#;
    let resp: TemplateListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.ids.len(), 2);
    assert_eq!(resp.ids[0], "FT-001");
}

// ═══════════════════════════════════════════════════════════════
// 15. 运费模板详情/ID（SOURCE_PARITY:
//     Java WxChannelFreightTemplateServiceImplTest (info/id)）
// ═══════════════════════════════════════════════════════════════

/// 运费模板详情响应 serde（对应 Java `TemplateInfoResponse`：
/// `errcode`/`errmsg`/`freight_template`）。
/// 对应 Java: WxChannelFreightTemplateServiceImplTest (info response)
#[test]
fn test_template_info_response_serde() {
    let json = r#"{
        "errcode":0,
        "errmsg":"success",
        "freight_template":{"template_id":"FT-001","name":"全国包邮","is_default":true}
    }"#;
    let resp: TemplateInfoResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.template.template_id, "FT-001");
    assert_eq!(resp.template.name, "全国包邮");
    assert!(resp.template.is_default);
}

/// 运费模板ID响应 serde（对应 Java `TemplateIdResponse`：
/// `err_code`/`err_msg`/`template_id`）。
/// 对应 Java: WxChannelFreightTemplateServiceImplTest (id response)
#[test]
fn test_template_id_response_serde() {
    let json = r#"{"err_code":0,"err_msg":"success","template_id":"FT-002"}"#;
    let resp: TemplateIdResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.template_id, "FT-002");
}

// ═══════════════════════════════════════════════════════════════
// VALUE_ADD: 边界/空值
// ═══════════════════════════════════════════════════════════════

/// 订单详情信息默认值。
#[test]
fn test_order_detail_info_default() {
    let info = OrderDetailInfo::default();
    assert!(info.product_infos.is_empty());
    assert_eq!(info.price_info.order_price, 0);
}

/// 售后列表参数默认值。
#[test]
fn test_after_sale_list_param_default() {
    let param = AfterSaleListParam::default();
    assert_eq!(param.begin_create_time, 0);
    assert!(param.next_key.is_empty());
}

/// 优惠券列表参数默认值。
#[test]
fn test_coupon_list_param_default() {
    let param = CouponListParam::default();
    assert_eq!(param.status, 0);
    assert_eq!(param.page, 0);
}

/// 优惠券列表响应默认值。
#[test]
fn test_coupon_list_response_default() {
    let resp: CouponListResponse = serde_json::from_str("{}").unwrap();
    assert_eq!(resp.total_num, 0);
    assert!(resp.coupons.is_empty());
}

/// 运费模板列表响应默认值。
#[test]
fn test_template_list_response_default() {
    let resp: TemplateListResponse = serde_json::from_str("{}").unwrap();
    assert!(resp.ids.is_empty());
}

/// 售后接受参数默认值。
#[test]
fn test_after_sale_accept_param_default() {
    let param = AfterSaleAcceptParam::default();
    assert!(param.after_sale_order_id.is_empty());
    assert_eq!(param.accept_type, 0);
}

/// 售后拒绝参数默认值。
#[test]
fn test_after_sale_reject_param_default() {
    let param = AfterSaleRejectParam::default();
    assert!(param.after_sale_order_id.is_empty());
    assert!(param.reject_reason.is_empty());
}

/// 订单ID参数 serde。
#[test]
fn test_order_id_param_serde() {
    let json = r#"{"order_id":"ORDER-001"}"#;
    let param: OrderIdParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.order_id, "ORDER-001");
}
