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

// ═══ Ewaybill ═══

#[test]
fn test_ewaybill_template_config_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","template_config_list":[{"template_code":"T001","template_name":"标准模板"}]}"#;
    let resp: wx_rust_channel::bean::ewaybill::TemplateConfigResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.template_config_list.len(), 1);
    assert_eq!(resp.template_config_list[0].template_code, "T001");
}

#[test]
fn test_ewaybill_create_order_request_serde() {
    let json = r#"{"delivery_id":"SF","template_id":"T001","order_id":"ORD-001","recv_addr":{"name":"张三","phone":"13700137000","province":"浙江","city":"杭州","district":"西湖","address":"文三路"},"send_addr":{"name":"李四","phone":"13800138000","province":"广东","city":"深圳","district":"南山","address":"科技园"}}"#;
    let req: wx_rust_channel::bean::ewaybill::CreateOrderRequest =
        serde_json::from_str(json).unwrap();
    assert_eq!(req.delivery_id, "SF");
    assert_eq!(req.recv_addr.name, "张三");
    assert_eq!(req.send_addr.city, "深圳");
}

// ═══ Favorite ═══

#[test]
fn test_favorite_count_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","favorite_count":12345}"#;
    let resp: wx_rust_channel::bean::favorite::FavoriteCountResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.favorite_count, 12345);
}

// ═══ Kf ═══

#[test]
fn test_kf_send_msg_param_serde() {
    let json = r#"{"open_id":"ox123","msg_type":"text","content":"你好"}"#;
    let param: wx_rust_channel::bean::kf::WxChannelKfSendMsgParam =
        serde_json::from_str(json).unwrap();
    assert_eq!(param.open_id, "ox123");
    assert_eq!(param.msg_type, "text");
    assert_eq!(param.content, "你好");
}

#[test]
fn test_kf_send_msg_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let resp: wx_rust_channel::bean::kf::WxChannelKfSendMsgResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
}

// ═══ Qic ═══

#[test]
fn test_qic_inspect_config_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","is_opened":true}"#;
    let resp: wx_rust_channel::bean::qic::InspectConfigResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert!(resp.is_opened);
}

#[test]
fn test_qic_submit_inspect_request_serde() {
    let json = r#"{"order_id":"ORD-001","inspect_code":"CODE-001"}"#;
    let req: wx_rust_channel::bean::qic::SubmitInspectRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.order_id, "ORD-001");
    assert_eq!(req.inspect_code, "CODE-001");
}

// ═══ Supplier ═══

#[test]
fn test_supplier_list_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","supplier_list":[{"supplier_id":"S001","supplier_name":"供货商A"}],"next_key":"key123"}"#;
    let resp: wx_rust_channel::bean::supplier::SupplierListResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.supplier_list.len(), 1);
    assert_eq!(resp.supplier_list[0].supplier_id, "S001");
    assert_eq!(resp.next_key, "key123");
}

#[test]
fn test_dropship_assign_request_serde() {
    let json = r#"{"order_id":"ORD-001","supplier_id":"S001"}"#;
    let req: wx_rust_channel::bean::supplier::DropshipAssignRequest =
        serde_json::from_str(json).unwrap();
    assert_eq!(req.order_id, "ORD-001");
    assert_eq!(req.supplier_id, "S001");
}

// ═══ Talent ═══

#[test]
fn test_talent_order_list_param_serde() {
    let json = r#"{"page_size":10,"next_key":"key123"}"#;
    let param: wx_rust_channel::bean::talent::TalentOrderListParam =
        serde_json::from_str(json).unwrap();
    assert_eq!(param.page_size, 10);
    assert_eq!(param.next_key, "key123");
}

#[test]
fn test_talent_order_list_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","order_list":[{"order_id":"T-001","product_id":"P-001"}],"next_key":"key456"}"#;
    let resp: wx_rust_channel::bean::talent::TalentOrderListResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.order_list.len(), 1);
    assert_eq!(resp.order_list[0].order_id, "T-001");
}

#[test]
fn test_talent_window_product_list_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","product_list":[{"product_id":"P-001","product_name":"商品A"}],"next_key":""}"#;
    let resp: wx_rust_channel::bean::talent::TalentWindowProductListResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.product_list.len(), 1);
    assert_eq!(resp.product_list[0].product_name, "商品A");
}

// ═══ Limit (update) ═══

#[test]
fn test_limit_task_update_param_serde() {
    let json = r#"{"task_id":"T-001","product_id":"P-001","start_time":"2024-01-01","end_time":"2024-01-02","limited_discount_skus":[]}"#;
    let param: wx_rust_channel::bean::limit::LimitTaskUpdateParam =
        serde_json::from_str(json).unwrap();
    assert_eq!(param.task_id, "T-001");
    assert_eq!(param.product_id, "P-001");
}

// ═══ Product Gift ═══

#[test]
fn test_gift_product_info_serde() {
    let json = r#"{"product_id":"GP-001","title":"赠品A","sub_title":"赠品副标题"}"#;
    let info: wx_rust_channel::bean::product::GiftProductInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.product_id, "GP-001");
    assert_eq!(info.title, "赠品A");
}

#[test]
fn test_gift_activity_info_serde() {
    let json = r#"{"activity_name":"买一送一","start_time":"2024-01-01","end_time":"2024-01-31"}"#;
    let info: wx_rust_channel::bean::product::GiftActivityInfo =
        serde_json::from_str(json).unwrap();
    assert_eq!(info.activity_name, "买一送一");
}

// ═══ Product Assistant ═══

#[test]
fn test_category_pre_check_param_serde() {
    let json = r#"{"category_id":"CAT-001"}"#;
    let param: wx_rust_channel::bean::product::assistant::CategoryPreCheckParam =
        serde_json::from_str(json).unwrap();
    assert_eq!(param.category_id, "CAT-001");
}

#[test]
fn test_product_brand_recommend_param_serde() {
    let json = r#"{"product_name":"手机壳"}"#;
    let param: wx_rust_channel::bean::product::assistant::ProductBrandRecommendParam =
        serde_json::from_str(json).unwrap();
    assert_eq!(param.product_name, "手机壳");
}

#[test]
fn test_begin_timing_sale_param_serde() {
    let json = r#"{"product_id":"P-001"}"#;
    let param: wx_rust_channel::bean::product::assistant::BeginTimingSaleParam =
        serde_json::from_str(json).unwrap();
    assert_eq!(param.product_id, "P-001");
}

// ═══ Product Stock Flow ═══

#[test]
fn test_stock_flow_param_serde() {
    let json = r#"{"product_id":"P-001","sku_id":"SKU-001","start_time":"2024-01-01","end_time":"2024-01-31","page_size":10,"next_key":""}"#;
    let param: wx_rust_channel::bean::product::stock::StockFlowParam =
        serde_json::from_str(json).unwrap();
    assert_eq!(param.product_id, "P-001");
    assert_eq!(param.sku_id, "SKU-001");
}

#[test]
fn test_stock_flow_response_serde() {
    let json = r#"{"errcode":0,"errmsg":"ok","flow_list":[{"flow_id":"F-001","flow_type":1,"stock_num":10,"create_time":"2024-01-01"}],"next_key":""}"#;
    let resp: wx_rust_channel::bean::product::stock::StockFlowResponse =
        serde_json::from_str(json).unwrap();
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.flow_list.len(), 1);
    assert_eq!(resp.flow_list[0].flow_id, "F-001");
}
