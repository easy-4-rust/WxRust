//! Phase 1 Batch 1.7: wx-rust-channel 核心测试
//!
//! 镜像 Java WxChannelBasicServiceImplTest / WxChannelOrderServiceImplTest /
//! WxChannelProductServiceImplTest / WxChannelAfterSaleServiceImplTest

use wx_rust_channel::bean::order::*;
use wx_rust_channel::bean::address::*;
use wx_rust_channel::bean::after::*;
use wx_rust_channel::bean::product::*;
use wx_rust_channel::bean::base::*;

// ═══ Order ═══

#[test]
fn test_order_id_param_serde() {
    let json = r#"{"order_id":"ORDER-001"}"#;
    let param: OrderIdParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.order_id, "ORDER-001");
}

#[test]
fn test_order_info_serde() {
    let json = r#"{"order_id":"ORD-001","status":1,"openid":"ox123","unionid":"u1"}"#;
    let order: OrderInfo = serde_json::from_str(json).unwrap();
    assert_eq!(order.order_id, "ORD-001");
    assert_eq!(order.status, 1);
}

#[test]
fn test_order_list_param_serde() {
    let json = r#"{"status":1,"page_size":10,"next_key":"key123"}"#;
    let param: OrderListParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.status, 1);
}

#[test]
fn test_order_remark_param_serde() {
    let json = r#"{"order_id":"ORD-003","merchant_notes":"请尽快发货"}"#;
    let param: OrderRemarkParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.order_id, "ORD-003");
}

#[test]
fn test_delivery_update_param_serde() {
    let json = r#"{"order_id":"ORD-005","delivery_list":[{"waybill_id":"WB-456","delivery_id":"SF-123","deliver_type":1}]}"#;
    let param: DeliveryUpdateParam = serde_json::from_str(json).unwrap();
    assert_eq!(param.order_id, "ORD-005");
    assert_eq!(param.delivery_list[0].delivery_id, "SF-123");
}

// ═══ Address ═══

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
}

#[test]
fn test_after_sale_reason_serde() {
    let json = r#"{"reason":1,"reason_text":"质量问题"}"#;
    let reason: AfterSaleReason = serde_json::from_str(json).unwrap();
    assert_eq!(reason.reason, 1);
}

// ═══ Product ═══

#[test]
fn test_description_info_serde() {
    let json = r#"{"desc":"产品描述","imgs":["img1.jpg"]}"#;
    let info: DescriptionInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.desc, "产品描述");
}

#[test]
fn test_express_info_serde() {
    let json = r#"{"template_id":"TPL-001","weight":500}"#;
    let info: ExpressInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.template_id, "TPL-001");
}

#[test]
fn test_extra_service_info_serde() {
    let json = r#"{"seven_day_return":1,"pay_after_use":0,"freight_insurance":1,"fake_one_pay_three":0,"damage_guarantee":1}"#;
    let info: ExtraServiceInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.seven_day_return, 1);
}

// ═══ Base ═══

#[test]
fn test_address_info_serde() {
    let json = r#"{"user_name":"王五","tel_number":"13700137000","province_name":"浙江","city_name":"杭州","county_name":"西湖","detail_info":"文三路"}"#;
    let info: AddressInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.user_name, "王五");
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
fn test_order_info_present_fields() {
    let json = r#"{"order_id":"ORD-GIFT","is_present":true,"present_order_id_str":"GIFT-001","present_note":"生日快乐","present_giver_openid":"giver-ox","present_giver_unionid":"giver-union"}"#;
    let order: OrderInfo = serde_json::from_str(json).unwrap();
    assert!(order.present);
    assert_eq!(order.present_note, "生日快乐");
}

#[test]
fn test_empty_json_defaults() {
    let order: OrderInfo = serde_json::from_str("{}").unwrap();
    assert_eq!(order.order_id, "");
    assert_eq!(order.status, 0);
}
