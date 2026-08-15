//! wx-rust-channel Bean 序列化综合测试（SOURCE_PARITY + VALUE_ADD）。

use wx_rust_channel::bean::address::*;
use wx_rust_channel::bean::after::*;
use wx_rust_channel::bean::base::*;
use wx_rust_channel::bean::order::*;
use wx_rust_channel::bean::product::{DescriptionInfo, ExpressInfo, ExtraServiceInfo, LimitInfo};

// ═══ Order ═══

#[test]
fn test_order_id_param_serde() {
    let json = r#"{"order_id":"ORDER-001"}"#;
    let param: OrderIdParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.order_id, "ORDER-001");
}

#[test]
fn test_order_info_serde() {
    let json = r#"{"order_id":"ORD-001","status":1,"openid":"ox123","unionid":"u1","create_time":1700000000,"update_time":1700000100}"#;
    let order: OrderInfo = serde_json::from_str(json).unwrap();
    assert_eq!(order.order_id, "ORD-001");
    assert_eq!(order.status, 1);
    assert_eq!(order.openid, "ox123");
}

#[test]
fn test_order_info_roundtrip() {
    let json = r#"{"order_id":"ORD-002","status":2,"openid":"ox456","unionid":"u2"}"#;
    let order: OrderInfo = serde_json::from_str(json).unwrap();
    let serialized = serde_json::to_string(&order).unwrap();
    let deserialized: OrderInfo = serde_json::from_str(&serialized).unwrap();
    assert_eq!(order, deserialized);
}

#[test]
fn test_order_info_present_fields() {
    let json = r#"{"order_id":"ORD-GIFT","is_present":true,"present_order_id_str":"GIFT-001","present_note":"生日快乐","present_giver_openid":"giver-ox","present_giver_unionid":"giver-union"}"#;
    let order: OrderInfo = serde_json::from_str(json).unwrap();
    assert!(order.present);
    assert_eq!(order.present_order_id, "GIFT-001");
    assert_eq!(order.present_note, "生日快乐");
}

#[test]
fn test_order_list_param_serde() {
    let json = r#"{"status":1,"page_size":10,"next_key":"key123"}"#;
    let param: OrderListParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.status, 1);
    assert_eq!(param.page_size, 10);
}

#[test]
fn test_order_remark_param_serde() {
    let json = r#"{"order_id":"ORD-003","merchant_notes":"请尽快发货"}"#;
    let param: OrderRemarkParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.order_id, "ORD-003");
    assert_eq!(param.merchant_notes, "请尽快发货");
}

#[test]
fn test_order_search_param_serde() {
    let json = r#"{"page_size":20,"next_key":"","status":1}"#;
    let param: OrderSearchParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.page_size, 20);
}

#[test]
fn test_delivery_update_param_serde() {
    let json = r#"{"order_id":"ORD-005","delivery_list":[{"waybill_id":"WB-456","delivery_id":"SF-123","deliver_type":1}]}"#;
    let param: DeliveryUpdateParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.order_id, "ORD-005");
    assert_eq!(param.delivery_list.len(), 1);
    assert_eq!(param.delivery_list[0].delivery_id, "SF-123");
}

// ═══ Address ═══

#[test]
fn test_address_add_param_serde() {
    let json = r#"{"address_detail":{"address_id":"addr-001","name":"张三","landline":"","send_addr":true,"recv_addr":true}}"#;
    let param: AddressAddParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.address_detail.name, "张三");
}

#[test]
fn test_address_detail_serde() {
    let json = r#"{"address_id":"addr-002","name":"李四","landline":"010-1234","send_addr":true,"recv_addr":false,"default_send":true,"default_recv":false,"create_time":1700000000}"#;
    let detail: AddressDetail = serde_json::from_str(json).unwrap();
    assert_eq!(detail.name, "李四");
    assert_eq!(detail.address_id, "addr-002");
}

#[test]
fn test_address_id_param_serde() {
    let json = r#"{"address_id":"addr-003"}"#;
    let param: AddressIdParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.address_id, "addr-003");
}

#[test]
fn test_address_list_param_serde() {
    let json = r#"{"offset":0,"limit":20}"#;
    let param: AddressListParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.offset, 0);
    assert_eq!(param.limit, 20);
}

#[test]
fn test_address_code_serde() {
    let json = r#"{"name":"广东省","code":440000,"level":1}"#;
    let code: AddressCode = serde_json::from_str(json).unwrap();
    assert_eq!(code.name, "广东省");
    assert_eq!(code.code, 440000);
}

// ═══ AfterSale ═══

#[test]
fn test_after_sale_id_param_serde() {
    let json = r#"{"after_sale_order_id":"AS-001"}"#;
    let param: AfterSaleIdParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.after_sale_order_id, "AS-001");
}

#[test]
fn test_after_sale_info_serde() {
    let json = r#"{"after_sale_order_id":"AS-002","order_id":"ORD-001","status":"PROCESSING","openid":"ox123"}"#;
    let info: wx_rust_channel::bean::after::AfterSaleInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.after_sale_order_id, "AS-002");
    assert_eq!(info.order_id, "ORD-001");
}

#[test]
fn test_after_sale_accept_param_serde() {
    let json = r#"{"after_sale_order_id":"AS-003","address_id":"addr-001","accept_type":1}"#;
    let param: AfterSaleAcceptParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.after_sale_order_id, "AS-003");
    assert_eq!(param.accept_type, 1);
}

#[test]
fn test_after_sale_list_param_serde() {
    let json = r#"{"begin_create_time":1700000000,"end_create_time":1700000100,"next_key":""}"#;
    let param: AfterSaleListParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.begin_create_time, 1700000000);
}

#[test]
fn test_after_sale_reason_serde() {
    let json = r#"{"reason":1,"reason_text":"质量问题"}"#;
    let reason: AfterSaleReason = serde_json::from_str(json).unwrap();
    assert_eq!(reason.reason, 1);
    assert_eq!(reason.reason_text, "质量问题");
}

#[test]
fn test_refund_info_serde() {
    let json = r#"{"amount":100,"refund_reason":1}"#;
    let info: RefundInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.amount, 100);
}

// ═══ Product ═══

#[test]
fn test_description_info_serde() {
    let json = r#"{"desc":"产品描述","imgs":["img1.jpg","img2.jpg"]}"#;
    let info: DescriptionInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.desc, "产品描述");
    assert_eq!(info.imgs.len(), 2);
}

#[test]
fn test_express_info_serde() {
    let json = r#"{"template_id":"TPL-001","weight":500}"#;
    let info: ExpressInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.template_id, "TPL-001");
    assert_eq!(info.weight, 500);
}

#[test]
fn test_extra_service_info_serde() {
    let json = r#"{"seven_day_return":1,"pay_after_use":0,"freight_insurance":1,"fake_one_pay_three":0,"damage_guarantee":1}"#;
    let info: ExtraServiceInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.seven_day_return, 1);
    assert_eq!(info.freight_insurance, 1);
}

#[test]
fn test_limit_info_serde() {
    let json = r#"{"period_type":1,"limited_buy_num":100}"#;
    let info: LimitInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.period_type, 1);
    assert_eq!(info.num, 100);
}

// ═══ Base ═══

#[test]
fn test_address_info_serde() {
    let json = r#"{"user_name":"王五","tel_number":"13700137000","province_name":"浙江","city_name":"杭州","county_name":"西湖","detail_info":"文三路"}"#;
    let info: AddressInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.user_name, "王五");
    assert_eq!(info.tel_number, "13700137000");
}

#[test]
fn test_attr_info_serde() {
    let json = r#"{"attr_key":"color","attr_value":"红色"}"#;
    let info: AttrInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.key, "color");
    assert_eq!(info.value, "红色");
}

// ═══ VALUE_ADD ═══

#[test]
fn test_empty_json_defaults() {
    let order: OrderInfo = serde_json::from_str("{}").unwrap();
    assert_eq!(order.order_id, "");
    assert_eq!(order.status, 0);
}

#[test]
fn test_order_detail_info_serde() {
    let json = r#"{"product_infos":[],"pay_info":{},"price_info":{},"delivery_info":{},"coupon_info":{},"ext_info":{}}"#;
    let detail: OrderDetailInfo = serde_json::from_str(json).unwrap();
    assert_eq!(detail.product_infos.len(), 0);
}
