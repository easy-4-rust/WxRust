//! 上游 Java 引用字段可空，空值与缺省在 Rust 中以 Option 保留。
#[test]
fn nullable_gift_fields_preserve_absence_and_explicit_empty() {
    use wx_rust_store::bean::product::gift_activity_info::GiftActivityInfo;
    let data: GiftActivityInfo =
        serde_json::from_value(serde_json::json!({"title":null,"start_time":null,"end_time":null}))
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    assert!(value.get("title").is_none());
    assert!(value.get("start_time").is_none());
    assert!(value.get("end_time").is_none());
    let empty: GiftActivityInfo = serde_json::from_value(serde_json::json!({"title":""})).unwrap();
    assert_eq!(serde_json::to_value(empty).unwrap()["title"], "");
}

// GENERATED_NULLABLE
#[test]
fn bean_address_address_add_param() {
    let data: wx_rust_store::bean::address::address_add_param::AddressAddParam =
        serde_json::from_str(r##"{"address_detail":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "address_detail";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_address_address_code() {
    let data: wx_rust_store::bean::address::address_code::AddressCode =
        serde_json::from_str(r##"{"name":null,"code":null,"level":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["name", "code", "level"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_address_address_code_response() {
    let data: wx_rust_store::bean::address::address_code_response::AddressCodeResponse =
        serde_json::from_str(r##"{"addrs_msg":null,"next_level_addrs":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["addrs_msg", "next_level_addrs"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_address_address_detail() {
    let data: wx_rust_store::bean::address::address_detail::AddressDetail = serde_json::from_str(r##"{"address_id":null,"name":null,"address_info":null,"landline":null,"send_addr":null,"recv_addr":null,"default_send":null,"default_recv":null,"create_time":null,"update_time":null,"address_type":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "address_id",
        "name",
        "address_info",
        "landline",
        "send_addr",
        "recv_addr",
        "default_send",
        "default_recv",
        "create_time",
        "update_time",
        "address_type",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_address_address_id_param() {
    let data: wx_rust_store::bean::address::address_id_param::AddressIdParam =
        serde_json::from_str(r##"{"address_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "address_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_address_address_id_response() {
    let data: wx_rust_store::bean::address::address_id_response::AddressIdResponse =
        serde_json::from_str(r##"{"address_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "address_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_address_address_info_response() {
    let data: wx_rust_store::bean::address::address_info_response::AddressInfoResponse =
        serde_json::from_str(r##"{"address_detail":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "address_detail";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_address_address_list_param() {
    let data: wx_rust_store::bean::address::address_list_param::AddressListParam =
        serde_json::from_str(r##"{"offset":null,"limit":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["offset", "limit"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_address_address_list_response() {
    let data: wx_rust_store::bean::address::address_list_response::AddressListResponse =
        serde_json::from_str(r##"{"address_id_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "address_id_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_address_offline_address_type() {
    let data: wx_rust_store::bean::address::offline_address_type::OfflineAddressType =
        serde_json::from_str(r##"{"same_city":null,"pickup":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["same_city", "pickup"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_accept_exchange_reship_param() {
    let data: wx_rust_store::bean::after::after_sale_accept_exchange_reship_param::AfterSaleAcceptExchangeReshipParam = serde_json::from_str(r##"{"after_sale_order_id":null,"waybill_id":null,"delivery_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["after_sale_order_id", "waybill_id", "delivery_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_accept_param() {
    let data: wx_rust_store::bean::after::after_sale_accept_param::AfterSaleAcceptParam =
        serde_json::from_str(
            r##"{"after_sale_order_id":null,"address_id":null,"accept_type":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["after_sale_order_id", "address_id", "accept_type"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_detail() {
    let data: wx_rust_store::bean::after::after_sale_detail::AfterSaleDetail = serde_json::from_str(r##"{"desc":null,"receive_product":null,"cancel_time":null,"prove_imgs":null,"tel_number":null,"media_id_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "desc",
        "receive_product",
        "cancel_time",
        "prove_imgs",
        "tel_number",
        "media_id_list",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_exchange_delivery_info() {
    let data: wx_rust_store::bean::after::after_sale_exchange_delivery_info::AfterSaleExchangeDeliveryInfo = serde_json::from_str(r##"{"waybill_id":null,"delivery_id":null,"delivery_name":null,"address_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["waybill_id", "delivery_id", "delivery_name", "address_info"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_exchange_product_info() {
    let data: wx_rust_store::bean::after::after_sale_exchange_product_info::AfterSaleExchangeProductInfo = serde_json::from_str(r##"{"product_id":null,"old_sku_id":null,"new_sku_id":null,"product_cnt":null,"old_sku_price":null,"new_sku_price":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_id",
        "old_sku_id",
        "new_sku_id",
        "product_cnt",
        "old_sku_price",
        "new_sku_price",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_id_param() {
    let data: wx_rust_store::bean::after::after_sale_id_param::AfterSaleIdParam =
        serde_json::from_str(r##"{"after_sale_order_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "after_sale_order_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_info() {
    let data: wx_rust_store::bean::after::after_sale_info::AfterSaleInfo = serde_json::from_str(r##"{"after_sale_order_id":null,"status":null,"order_id":null,"openid":null,"unionid":null,"product_info":null,"details":null,"refund_info":null,"return_info":null,"merchant_upload_info":null,"create_time":null,"update_time":null,"reason":null,"reason_text":null,"refund_resp":null,"type":null,"complaint_id":null,"deadline":null,"exchange_product_info":null,"exchange_delivery_info":null,"virtual_tel_num_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "after_sale_order_id",
        "status",
        "order_id",
        "openid",
        "unionid",
        "product_info",
        "details",
        "refund_info",
        "return_info",
        "merchant_upload_info",
        "create_time",
        "update_time",
        "reason",
        "reason_text",
        "refund_resp",
        "type",
        "complaint_id",
        "deadline",
        "exchange_product_info",
        "exchange_delivery_info",
        "virtual_tel_num_info",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_info_response() {
    let data: wx_rust_store::bean::after::after_sale_info_response::AfterSaleInfoResponse =
        serde_json::from_str(r##"{"after_sale_order":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "after_sale_order";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_list_param() {
    let data: wx_rust_store::bean::after::after_sale_list_param::AfterSaleListParam = serde_json::from_str(r##"{"begin_create_time":null,"end_create_time":null,"begin_update_time":null,"end_update_time":null,"next_key":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "begin_create_time",
        "end_create_time",
        "begin_update_time",
        "end_update_time",
        "next_key",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_list_response() {
    let data: wx_rust_store::bean::after::after_sale_list_response::AfterSaleListResponse =
        serde_json::from_str(
            r##"{"after_sale_order_id_list":null,"next_key":null,"has_more":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["after_sale_order_id_list", "next_key", "has_more"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_merchant_update_param() {
    let data: wx_rust_store::bean::after::after_sale_merchant_update_param::AfterSaleMerchantUpdateParam = serde_json::from_str(r##"{"after_sale_order_id":null,"type":null,"amount":null,"merchant_update_desc":null,"update_reason_type":null,"merchant_update_type":null,"media_ids":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "after_sale_order_id",
        "type",
        "amount",
        "merchant_update_desc",
        "update_reason_type",
        "merchant_update_type",
        "media_ids",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_product_info() {
    let data: wx_rust_store::bean::after::after_sale_product_info::AfterSaleProductInfo =
        serde_json::from_str(r##"{"product_id":null,"sku_id":null,"count":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "sku_id", "count"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_reason() {
    let data: wx_rust_store::bean::after::after_sale_reason::AfterSaleReason =
        serde_json::from_str(r##"{"reason":null,"reason_text":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["reason", "reason_text"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_reason_response() {
    let data: wx_rust_store::bean::after::after_sale_reason_response::AfterSaleReasonResponse =
        serde_json::from_str(r##"{"reason_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "reason_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_reject_exchange_reship_param() {
    let data: wx_rust_store::bean::after::after_sale_reject_exchange_reship_param::AfterSaleRejectExchangeReshipParam = serde_json::from_str(r##"{"after_sale_order_id":null,"reject_reason":null,"reject_reason_type":null,"reject_certificates":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "after_sale_order_id",
        "reject_reason",
        "reject_reason_type",
        "reject_certificates",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_reject_param() {
    let data: wx_rust_store::bean::after::after_sale_reject_param::AfterSaleRejectParam = serde_json::from_str(r##"{"after_sale_order_id":null,"reject_reason":null,"reject_reason_type":null,"reject_certificates":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "after_sale_order_id",
        "reject_reason",
        "reject_reason_type",
        "reject_certificates",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_reject_reason() {
    let data: wx_rust_store::bean::after::after_sale_reject_reason::AfterSaleRejectReason = serde_json::from_str(r##"{"reject_reason_type":null,"reject_reason_type_text":null,"reject_reason":null,"reject_scene":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "reject_reason_type",
        "reject_reason_type_text",
        "reject_reason",
        "reject_scene",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_reject_reason_response() {
    let data: wx_rust_store::bean::after::after_sale_reject_reason_response::AfterSaleRejectReasonResponse = serde_json::from_str(r##"{"reject_reason_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "reject_reason_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_return_param() {
    let data: wx_rust_store::bean::after::after_sale_return_param::AfterSaleReturnParam =
        serde_json::from_str(
            r##"{"aftersale_id":null,"out_aftersale_id":null,"address_info":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["aftersale_id", "out_aftersale_id", "address_info"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_after_sale_virtual_number_info() {
    let data: wx_rust_store::bean::after::after_sale_virtual_number_info::AfterSaleVirtualNumberInfo = serde_json::from_str(r##"{"virtual_tel_number":null,"virtual_tel_expire_time":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["virtual_tel_number", "virtual_tel_expire_time"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_guarantee_modify_request() {
    let data: wx_rust_store::bean::after::guarantee_modify_request::GuaranteeModifyRequest =
        serde_json::from_str(
            r##"{"guarantee_order_id":null,"bad_level":null,"merchant_remark":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["guarantee_order_id", "bad_level", "merchant_remark"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_guarantee_order_id_param() {
    let data: wx_rust_store::bean::after::guarantee_order_id_param::GuaranteeOrderIdParam =
        serde_json::from_str(r##"{"guarantee_order_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "guarantee_order_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_guarantee_order_info_response() {
    let data: wx_rust_store::bean::after::guarantee_order_info_response::GuaranteeOrderInfoResponse = serde_json::from_str(r##"{"guarantee_order":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "guarantee_order";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_guarantee_order_list_param() {
    let data: wx_rust_store::bean::after::guarantee_order_list_param::GuaranteeOrderListParam = serde_json::from_str(r##"{"guarantee_order_id_list":null,"order_id_list":null,"type":null,"begin_time":null,"end_time":null,"status_list":null,"offset":null,"limit":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "guarantee_order_id_list",
        "order_id_list",
        "type",
        "begin_time",
        "end_time",
        "status_list",
        "offset",
        "limit",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_guarantee_order_list_response() {
    let data: wx_rust_store::bean::after::guarantee_order_list_response::GuaranteeOrderListResponse = serde_json::from_str(r##"{"total_num":null,"guarantee_order_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["total_num", "guarantee_order_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_guarantee_proof_request() {
    let data: wx_rust_store::bean::after::guarantee_proof_request::GuaranteeProofRequest =
        serde_json::from_str(r##"{"guarantee_order_id":null,"content":null,"pic_list":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["guarantee_order_id", "content", "pic_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_guarantee_refuse_request() {
    let data: wx_rust_store::bean::after::guarantee_refuse_request::GuaranteeRefuseRequest =
        serde_json::from_str(r##"{"guarantee_order_id":null,"reason":null,"pic_list":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["guarantee_order_id", "reason", "pic_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_merchant_upload_info() {
    let data: wx_rust_store::bean::after::merchant_upload_info::MerchantUploadInfo =
        serde_json::from_str(r##"{"reject_reason":null,"refund_certificates":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["reject_reason", "refund_certificates"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_refund_evidence_param() {
    let data: wx_rust_store::bean::after::refund_evidence_param::RefundEvidenceParam =
        serde_json::from_str(
            r##"{"after_sale_order_id":null,"desc":null,"refund_certificates":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["after_sale_order_id", "desc", "refund_certificates"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_refund_info() {
    let data: wx_rust_store::bean::after::refund_info::RefundInfo =
        serde_json::from_str(r##"{"amount":null,"refund_reason":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["amount", "refund_reason"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_refund_resp() {
    let data: wx_rust_store::bean::after::refund_resp::RefundResp =
        serde_json::from_str(r##"{"code":null,"ret":null,"message":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["code", "ret", "message"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_after_return_info() {
    let data: wx_rust_store::bean::after::return_info::ReturnInfo =
        serde_json::from_str(r##"{"waybill_id":null,"delivery_id":null,"delivery_name":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["waybill_id", "delivery_id", "delivery_name"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_audit_audit_apply_response() {
    let data: wx_rust_store::bean::audit::audit_apply_response::AuditApplyResponse =
        serde_json::from_str(r##"{"audit_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "audit_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_audit_audit_response() {
    let data: wx_rust_store::bean::audit::audit_response::AuditResponse =
        serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_audit_audit_result() {
    let data: wx_rust_store::bean::audit::audit_result::AuditResult =
        serde_json::from_str(r##"{"status":null,"reject_reason":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["status", "reject_reason"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_audit_category_audit_info() {
    let data: wx_rust_store::bean::audit::category_audit_info::CategoryAuditInfo = serde_json::from_str(r##"{"level1":null,"level2":null,"level3":null,"cats_v2":null,"certificate":null,"baobeihan":null,"jingyingzhengming":null,"daihuokoubei":null,"ruzhuzhizhi":null,"jingyingliushui":null,"buchongcailiao":null,"jingyingpingtai":null,"zhanghaomingcheng":null,"brand_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "level1",
        "level2",
        "level3",
        "cats_v2",
        "certificate",
        "baobeihan",
        "jingyingzhengming",
        "daihuokoubei",
        "ruzhuzhizhi",
        "jingyingliushui",
        "buchongcailiao",
        "jingyingpingtai",
        "zhanghaomingcheng",
        "brand_list",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_audit_category_audit_request() {
    let data: wx_rust_store::bean::audit::category_audit_request::CategoryAuditRequest =
        serde_json::from_str(r##"{"category_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "category_info";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_audit_category_brand() {
    let data: wx_rust_store::bean::audit::category_brand::CategoryBrand =
        serde_json::from_str(r##"{"brand_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "brand_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_audit_cats_v2() {
    let data: wx_rust_store::bean::audit::cats_v2::CatsV2 =
        serde_json::from_str(r##"{"cat_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "cat_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_audit_product_audit_info() {
    let data: wx_rust_store::bean::audit::product_audit_info::ProductAuditInfo = serde_json::from_str(r##"{"audit_id":null,"submit_time":null,"audit_time":null,"reject_reason":null,"func_type":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "audit_id",
        "submit_time",
        "audit_time",
        "reject_reason",
        "func_type",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_base_address_info() {
    let data: wx_rust_store::bean::base::address_info::AddressInfo = serde_json::from_str(r##"{"user_name":null,"tel_number":null,"postal_code":null,"province_name":null,"city_name":null,"county_name":null,"detail_info":null,"national_code":null,"house_number":null,"lat":null,"lng":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "user_name",
        "tel_number",
        "postal_code",
        "province_name",
        "city_name",
        "county_name",
        "detail_info",
        "national_code",
        "house_number",
        "lat",
        "lng",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_base_attr_info() {
    let data: wx_rust_store::bean::base::attr_info::AttrInfo =
        serde_json::from_str(r##"{"attr_key":null,"attr_value":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["attr_key", "attr_value"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_base_offset_param() {
    let data: wx_rust_store::bean::base::offset_param::OffsetParam =
        serde_json::from_str(r##"{"offset":null,"limit":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["offset", "limit"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_base_page_param() {
    let data: wx_rust_store::bean::base::page_param::PageParam =
        serde_json::from_str(r##"{"page":null,"page_size":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["page", "page_size"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_base_stream_page_param() {
    let data: wx_rust_store::bean::base::stream_page_param::StreamPageParam =
        serde_json::from_str(r##"{"page_size":null,"next_key":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["page_size", "next_key"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_base_time_range() {
    let data: wx_rust_store::bean::base::time_range::TimeRange =
        serde_json::from_str(r##"{"start_time":null,"end_time":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["start_time", "end_time"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_brand_basic_brand() {
    let data: wx_rust_store::bean::brand::basic_brand::BasicBrand =
        serde_json::from_str(r##"{"brand_id":null,"ch_name":null,"en_name":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["brand_id", "ch_name", "en_name"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_brand_brand() {
    let data: wx_rust_store::bean::brand::brand::Brand = serde_json::from_str(r##"{"brand_id":null,"ch_name":null,"en_name":null,"classification_no":null,"trade_mark_symbol":null,"register_details":null,"application_details":null,"grant_type":null,"grant_details":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "brand_id",
        "ch_name",
        "en_name",
        "classification_no",
        "trade_mark_symbol",
        "register_details",
        "application_details",
        "grant_type",
        "grant_details",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_brand_brand_application_detail() {
    let data: wx_rust_store::bean::brand::brand_application_detail::BrandApplicationDetail =
        serde_json::from_str(
            r##"{"acceptance_time":null,"acceptance_certification":null,"acceptance_no":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "acceptance_time",
        "acceptance_certification",
        "acceptance_no",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_brand_brand_apply_list_response() {
    let data: wx_rust_store::bean::brand::brand_apply_list_response::BrandApplyListResponse =
        serde_json::from_str(r##"{"brands":null,"next_key":null,"total_num":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["brands", "next_key", "total_num"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_brand_brand_grant_detail() {
    let data: wx_rust_store::bean::brand::brand_grant_detail::BrandGrantDetail = serde_json::from_str(r##"{"grant_certifications":null,"grant_level":null,"start_time":null,"end_time":null,"brand_owner_id_photos":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "grant_certifications",
        "grant_level",
        "start_time",
        "end_time",
        "brand_owner_id_photos",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_brand_brand_info() {
    let data: wx_rust_store::bean::brand::brand_info::BrandInfo = serde_json::from_str(r##"{"brand_id":null,"ch_name":null,"en_name":null,"classification_no":null,"trade_mark_symbol":null,"register_details":null,"application_details":null,"grant_type":null,"grant_details":null,"status":null,"create_time":null,"update_time":null,"audit_result":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "brand_id",
        "ch_name",
        "en_name",
        "classification_no",
        "trade_mark_symbol",
        "register_details",
        "application_details",
        "grant_type",
        "grant_details",
        "status",
        "create_time",
        "update_time",
        "audit_result",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_brand_brand_info_response() {
    let data: wx_rust_store::bean::brand::brand_info_response::BrandInfoResponse =
        serde_json::from_str(r##"{"brand":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "brand";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_brand_brand_list_response() {
    let data: wx_rust_store::bean::brand::brand_list_response::BrandListResponse =
        serde_json::from_str(r##"{"brands":null,"next_key":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["brands", "next_key"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_brand_brand_param() {
    let data: wx_rust_store::bean::brand::brand_param::BrandParam =
        serde_json::from_str(r##"{"brand":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "brand";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_brand_brand_register_detail() {
    let data: wx_rust_store::bean::brand::brand_register_detail::BrandRegisterDetail = serde_json::from_str(r##"{"registrant":null,"register_no":null,"start_time":null,"end_time":null,"register_certifications":null,"renew_certifications":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "registrant",
        "register_no",
        "start_time",
        "end_time",
        "register_certifications",
        "renew_certifications",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_brand_brand_search_param() {
    let data: wx_rust_store::bean::brand::brand_search_param::BrandSearchParam =
        serde_json::from_str(r##"{"page_size":null,"next_key":null,"status":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["page_size", "next_key", "status"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_account_category_response() {
    let data: wx_rust_store::bean::category::account_category_response::AccountCategoryResponse =
        serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_category_and_qualification_list() {
    let data: wx_rust_store::bean::category::category_and_qualification_list::CategoryAndQualificationList = serde_json::from_str(r##"{"cat_and_qua":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "cat_and_qua";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_category_detail_result() {
    let data: wx_rust_store::bean::category::category_detail_result::CategoryDetailResult =
        serde_json::from_str(r##"{"info":null,"attr":null,"product_qua_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["info", "attr", "product_qua_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_category_qualification() {
    let data: wx_rust_store::bean::category::category_qualification::CategoryQualification = serde_json::from_str(r##"{"cat":null,"qua":null,"product_qua":null,"brand_qua":null,"product_qua_list":null,"is_confidence_require_bad_must_pay":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "cat",
        "qua",
        "product_qua",
        "brand_qua",
        "product_qua_list",
        "is_confidence_require_bad_must_pay",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_category_qualification_response() {
    let data: wx_rust_store::bean::category::category_qualification_response::CategoryQualificationResponse = serde_json::from_str(r##"{"cats":null,"cats_v2":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["cats", "cats_v2"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_pass_category_info() {
    let data: wx_rust_store::bean::category::pass_category_info::PassCategoryInfo =
        serde_json::from_str(r##"{"cat_id":null,"qua_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["cat_id", "qua_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_pass_category_response() {
    let data: wx_rust_store::bean::category::pass_category_response::PassCategoryResponse =
        serde_json::from_str(r##"{"list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_qualification_info() {
    let data: wx_rust_store::bean::category::qualification_info::QualificationInfo =
        serde_json::from_str(
            r##"{"qua_id":null,"need_to_apply":null,"tips":null,"mandatory":null,"name":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["qua_id", "need_to_apply", "tips", "mandatory", "name"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_relation_category_item() {
    let data: wx_rust_store::bean::category::relation_category_item::RelationCategoryItem = serde_json::from_str(r##"{"id":null,"status":null,"uneffective_reason":null,"effective_time":null,"uneffective_time":null,"qua_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "id",
        "status",
        "uneffective_reason",
        "effective_time",
        "uneffective_time",
        "qua_id",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_relation_category_request() {
    let data: wx_rust_store::bean::category::relation_category_request::RelationCategoryRequest =
        serde_json::from_str(r##"{"is_filter_status":null,"status":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["is_filter_status", "status"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_relation_category_response() {
    let data: wx_rust_store::bean::category::relation_category_response::RelationCategoryResponse =
        serde_json::from_str(r##"{"list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_shop_category() {
    let data: wx_rust_store::bean::category::shop_category::ShopCategory = serde_json::from_str(
        r##"{"cat_id":null,"f_cat_id":null,"name":null,"level":null,"leaf":null}"##,
    )
    .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["cat_id", "f_cat_id", "name", "level", "leaf"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_category_shop_category_response() {
    let data: wx_rust_store::bean::category::shop_category_response::ShopCategoryResponse =
        serde_json::from_str(r##"{"cat_list":null,"cat_list_v2":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["cat_list", "cat_list_v2"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_compass_finder_base_param() {
    let data: wx_rust_store::bean::compass::compass_finder_base_param::CompassFinderBaseParam =
        serde_json::from_str(r##"{"ds":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "ds";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_compass_finder_id_param() {
    let data: wx_rust_store::bean::compass::shop::compass_finder_id_param::CompassFinderIdParam =
        serde_json::from_str(r##"{"ds":null,"finder_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["ds", "finder_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_finder_auth_list_response() {
    let data: wx_rust_store::bean::compass::shop::finder_auth_list_response::FinderAuthListResponse = serde_json::from_str(r##"{"main_finder_id":null,"authorized_finder_id_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["main_finder_id", "authorized_finder_id_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_finder_gmv_data() {
    let data: wx_rust_store::bean::compass::shop::finder_gmv_data::FinderGmvData = serde_json::from_str(r##"{"pay_gmv":null,"pay_product_id_cnt":null,"pay_uv":null,"refund_gmv":null,"pay_refund_gmv":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "pay_gmv",
        "pay_product_id_cnt",
        "pay_uv",
        "refund_gmv",
        "pay_refund_gmv",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_finder_gmv_item() {
    let data: wx_rust_store::bean::compass::shop::finder_gmv_item::FinderGmvItem =
        serde_json::from_str(r##"{"finder_id":null,"finder_nickname":null,"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["finder_id", "finder_nickname", "data"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_finder_list_response() {
    let data: wx_rust_store::bean::compass::shop::finder_list_response::FinderListResponse =
        serde_json::from_str(r##"{"finder_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "finder_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_finder_overall_data() {
    let data: wx_rust_store::bean::compass::shop::finder_overall_data::FinderOverallData = serde_json::from_str(r##"{"pay_gmv":null,"pay_sales_finder_cnt":null,"pay_product_id_cnt":null,"click_to_pay_uv_ratio":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "pay_gmv",
        "pay_sales_finder_cnt",
        "pay_product_id_cnt",
        "click_to_pay_uv_ratio",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_finder_overall_response() {
    let data: wx_rust_store::bean::compass::shop::finder_overall_response::FinderOverallResponse =
        serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_finder_product_list_item() {
    let data: wx_rust_store::bean::compass::shop::finder_product_list_item::FinderProductListItem = serde_json::from_str(r##"{"product_id":null,"head_img_url":null,"title":null,"price":null,"first_category_id":null,"second_category_id":null,"third_category_id":null,"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_id",
        "head_img_url",
        "title",
        "price",
        "first_category_id",
        "second_category_id",
        "third_category_id",
        "data",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_finder_product_list_response() {
    let data: wx_rust_store::bean::compass::shop::finder_product_list_response::FinderProductListResponse = serde_json::from_str(r##"{"product_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_finder_product_overall_response() {
    let data: wx_rust_store::bean::compass::shop::finder_product_overall_response::FinderProductOverallResponse = serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_finder_product_simple_gmv_data() {
    let data: wx_rust_store::bean::compass::shop::finder_product_simple_gmv_data::FinderProductSimpleGmvData = serde_json::from_str(r##"{"commission_ratio":null,"pay_gmv":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["commission_ratio", "pay_gmv"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_field() {
    let data: wx_rust_store::bean::compass::shop::shop_field::ShopField =
        serde_json::from_str(r##"{"field_name":null,"data_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["field_name", "data_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_live_data() {
    let data: wx_rust_store::bean::compass::shop::shop_live_data::ShopLiveData = serde_json::from_str(r##"{"live_id":null,"live_title":null,"live_time":null,"live_duration":null,"live_cover_img_url":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "live_id",
        "live_title",
        "live_time",
        "live_duration",
        "live_cover_img_url",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_live_list_response() {
    let data: wx_rust_store::bean::compass::shop::shop_live_list_response::ShopLiveListResponse =
        serde_json::from_str(r##"{"live_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "live_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_overall() {
    let data: wx_rust_store::bean::compass::shop::shop_overall::ShopOverall = serde_json::from_str(r##"{"pay_gmv":null,"pay_uv":null,"pay_refund_gmv":null,"pay_order_cnt":null,"live_pay_gmv":null,"feed_pay_gmv":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "pay_gmv",
        "pay_uv",
        "pay_refund_gmv",
        "pay_order_cnt",
        "live_pay_gmv",
        "feed_pay_gmv",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_overall_response() {
    let data: wx_rust_store::bean::compass::shop::shop_overall_response::ShopOverallResponse =
        serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_product_compass_data() {
    let data: wx_rust_store::bean::compass::shop::shop_product_compass_data::ShopProductCompassData = serde_json::from_str(r##"{"pay_gmv":null,"create_gmv":null,"create_cnt":null,"create_uv":null,"create_product_cnt":null,"pay_cnt":null,"pay_uv":null,"pay_product_cnt":null,"pure_pay_gmv":null,"pay_gmv_per_uv":null,"seller_actual_settle_amount":null,"platform_actual_commission":null,"finderuin_actual_commission":null,"captain_actual_commission":null,"seller_predict_settle_amount":null,"platform_predict_commission":null,"finderuin_predict_commission":null,"captain_predict_commission":null,"product_click_uv":null,"product_click_cnt":null,"pay_refund_gmv":null,"pay_refund_uv":null,"pay_refund_ratio":null,"pay_refund_after_send_ratio":null,"pay_refund_cnt":null,"pay_refund_product_cnt":null,"pay_refund_before_send_ratio":null,"refund_gmv":null,"refund_product_cnt":null,"refund_cnt":null,"refund_uv":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "pay_gmv",
        "create_gmv",
        "create_cnt",
        "create_uv",
        "create_product_cnt",
        "pay_cnt",
        "pay_uv",
        "pay_product_cnt",
        "pure_pay_gmv",
        "pay_gmv_per_uv",
        "seller_actual_settle_amount",
        "platform_actual_commission",
        "finderuin_actual_commission",
        "captain_actual_commission",
        "seller_predict_settle_amount",
        "platform_predict_commission",
        "finderuin_predict_commission",
        "captain_predict_commission",
        "product_click_uv",
        "product_click_cnt",
        "pay_refund_gmv",
        "pay_refund_uv",
        "pay_refund_ratio",
        "pay_refund_after_send_ratio",
        "pay_refund_cnt",
        "pay_refund_product_cnt",
        "pay_refund_before_send_ratio",
        "refund_gmv",
        "refund_product_cnt",
        "refund_cnt",
        "refund_uv",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_product_data_param() {
    let data: wx_rust_store::bean::compass::shop::shop_product_data_param::ShopProductDataParam =
        serde_json::from_str(r##"{"ds":null,"product_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["ds", "product_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_product_data_response() {
    let data: wx_rust_store::bean::compass::shop::shop_product_data_response::ShopProductDataResponse = serde_json::from_str(r##"{"product_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product_info";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_product_info() {
    let data: wx_rust_store::bean::compass::shop::shop_product_info::ShopProductInfo = serde_json::from_str(r##"{"product_id":null,"head_img_url":null,"title":null,"price":null,"first_category_id":null,"second_category_id":null,"third_category_id":null,"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_id",
        "head_img_url",
        "title",
        "price",
        "first_category_id",
        "second_category_id",
        "third_category_id",
        "data",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_product_list_response() {
    let data: wx_rust_store::bean::compass::shop::shop_product_list_response::ShopProductListResponse = serde_json::from_str(r##"{"product_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_sale_profile_data() {
    let data: wx_rust_store::bean::compass::shop::shop_sale_profile_data::ShopSaleProfileData =
        serde_json::from_str(r##"{"field_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "field_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_sale_profile_data_param() {
    let data: wx_rust_store::bean::compass::shop::shop_sale_profile_data_param::ShopSaleProfileDataParam = serde_json::from_str(r##"{"ds":null,"type":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["ds", "type"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_compass_shop_shop_sale_profile_data_response() {
    let data: wx_rust_store::bean::compass::shop::shop_sale_profile_data_response::ShopSaleProfileDataResponse = serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_complaint_complaint_history() {
    let data: wx_rust_store::bean::complaint::complaint_history::ComplaintHistory = serde_json::from_str(r##"{"item_type":null,"time":null,"phone_number":null,"content":null,"media_id_list":null,"after_sale_type":null,"after_sale_reason":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "item_type",
        "time",
        "phone_number",
        "content",
        "media_id_list",
        "after_sale_type",
        "after_sale_reason",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_complaint_complaint_order_response() {
    let data: wx_rust_store::bean::complaint::complaint_order_response::ComplaintOrderResponse =
        serde_json::from_str(
            r##"{"after_sale_order_id":null,"order_id":null,"history":null,"status":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["after_sale_order_id", "order_id", "history", "status"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_complaint_complaint_param() {
    let data: wx_rust_store::bean::complaint::complaint_param::ComplaintParam =
        serde_json::from_str(r##"{"complaint_id":null,"content":null,"media_id_list":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["complaint_id", "content", "media_id_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_cooperation_cooperation_data() {
    let data: wx_rust_store::bean::cooperation::cooperation_data::CooperationData = serde_json::from_str(r##"{"sharer_id":null,"status":null,"sharer_name":null,"sharer_type":null,"bind_time":null,"reject_time":null,"cancel_time":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "sharer_id",
        "status",
        "sharer_name",
        "sharer_type",
        "bind_time",
        "reject_time",
        "cancel_time",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_cooperation_cooperation_list_response() {
    let data: wx_rust_store::bean::cooperation::cooperation_list_response::CooperationListResponse =
        serde_json::from_str(r##"{"data_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_cooperation_cooperation_qr_code() {
    let data: wx_rust_store::bean::cooperation::cooperation_qr_code::CooperationQrCode =
        serde_json::from_str(r##"{"qrcode_base64":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "qrcode_base64";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_cooperation_cooperation_qr_code_response() {
    let data: wx_rust_store::bean::cooperation::cooperation_qr_code_response::CooperationQrCodeResponse = serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_cooperation_cooperation_sharer_param() {
    let data: wx_rust_store::bean::cooperation::cooperation_sharer_param::CooperationSharerParam =
        serde_json::from_str(r##"{"sharer_id":null,"sharer_type":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["sharer_id", "sharer_type"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_cooperation_cooperation_status() {
    let data: wx_rust_store::bean::cooperation::cooperation_status::CooperationStatus =
        serde_json::from_str(r##"{"status":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "status";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_cooperation_cooperation_status_response() {
    let data: wx_rust_store::bean::cooperation::cooperation_status_response::CooperationStatusResponse = serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_auto_valid_info() {
    let data: wx_rust_store::bean::coupon::auto_valid_info::AutoValidInfo =
        serde_json::from_str(r##"{"auto_valid_type":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "auto_valid_type";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_coupon_detail_info() {
    let data: wx_rust_store::bean::coupon::coupon_detail_info::CouponDetailInfo = serde_json::from_str(r##"{"name":null,"valid_info":null,"promote_info":null,"discount_info":null,"ext_info":null,"receive_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "name",
        "valid_info",
        "promote_info",
        "discount_info",
        "ext_info",
        "receive_info",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_coupon_id_info() {
    let data: wx_rust_store::bean::coupon::coupon_id_info::CouponIdInfo =
        serde_json::from_str(r##"{"coupon_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "coupon_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_coupon_id_response() {
    let data: wx_rust_store::bean::coupon::coupon_id_response::CouponIdResponse =
        serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_coupon_info() {
    let data: wx_rust_store::bean::coupon::coupon_info::CouponInfo = serde_json::from_str(r##"{"coupon_id":null,"type":null,"status":null,"create_time":null,"update_time":null,"coupon_info":null,"stock_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "coupon_id",
        "type",
        "status",
        "create_time",
        "update_time",
        "coupon_info",
        "stock_info",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_coupon_info_response() {
    let data: wx_rust_store::bean::coupon::coupon_info_response::CouponInfoResponse =
        serde_json::from_str(r##"{"coupon":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "coupon";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_coupon_list_param() {
    let data: wx_rust_store::bean::coupon::coupon_list_param::CouponListParam =
        serde_json::from_str(r##"{"status":null,"page":null,"page_size":null,"page_ctx":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["status", "page", "page_size", "page_ctx"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_coupon_list_response() {
    let data: wx_rust_store::bean::coupon::coupon_list_response::CouponListResponse =
        serde_json::from_str(r##"{"coupons":null,"total_num":null,"page_ctx":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["coupons", "total_num", "page_ctx"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_coupon_param() {
    let data: wx_rust_store::bean::coupon::coupon_param::CouponParam = serde_json::from_str(r##"{"coupon_id":null,"type":null,"name":null,"discount_info":null,"ext_info":null,"promote_info":null,"receive_info":null,"valid_info":null,"auto_valid_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "coupon_id",
        "type",
        "name",
        "discount_info",
        "ext_info",
        "promote_info",
        "receive_info",
        "valid_info",
        "auto_valid_info",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_coupon_status_param() {
    let data: wx_rust_store::bean::coupon::coupon_status_param::CouponStatusParam =
        serde_json::from_str(r##"{"coupon_id":null,"status":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["coupon_id", "status"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_discount_condition() {
    let data: wx_rust_store::bean::coupon::discount_condition::DiscountCondition =
        serde_json::from_str(r##"{"product_cnt":null,"product_price":null,"product_ids":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_cnt", "product_price", "product_ids"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_discount_info() {
    let data: wx_rust_store::bean::coupon::discount_info::DiscountInfo = serde_json::from_str(
        r##"{"discount_num":null,"discount_fee":null,"discount_condition":null}"##,
    )
    .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["discount_num", "discount_fee", "discount_condition"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_ext_info() {
    let data: wx_rust_store::bean::coupon::ext_info::ExtInfo = serde_json::from_str(
        r##"{"jump_product_id":null,"notes":null,"valid_time":null,"invalid_time":null}"##,
    )
    .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["jump_product_id", "notes", "valid_time", "invalid_time"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_promote_info() {
    let data: wx_rust_store::bean::coupon::promote_info::PromoteInfo =
        serde_json::from_str(r##"{"promote_type":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "promote_type";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_receive_info() {
    let data: wx_rust_store::bean::coupon::receive_info::ReceiveInfo = serde_json::from_str(
        r##"{"end_time":null,"limit_num_one_person":null,"start_time":null,"total_num":null}"##,
    )
    .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "end_time",
        "limit_num_one_person",
        "start_time",
        "total_num",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_stock_info() {
    let data: wx_rust_store::bean::coupon::stock_info::StockInfo =
        serde_json::from_str(r##"{"issued_num":null,"receive_num":null,"used_num":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["issued_num", "receive_num", "used_num"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_user_coupon() {
    let data: wx_rust_store::bean::coupon::user_coupon::UserCoupon = serde_json::from_str(r##"{"coupon_id":null,"user_coupon_id":null,"status":null,"create_time":null,"update_time":null,"start_time":null,"end_time":null,"ext_info":null,"order_id":null,"discount_fee":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "coupon_id",
        "user_coupon_id",
        "status",
        "create_time",
        "update_time",
        "start_time",
        "end_time",
        "ext_info",
        "order_id",
        "discount_fee",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_user_coupon_id_info() {
    let data: wx_rust_store::bean::coupon::user_coupon_id_info::UserCouponIdInfo =
        serde_json::from_str(r##"{"coupon_id":null,"user_coupon_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["coupon_id", "user_coupon_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_user_coupon_id_param() {
    let data: wx_rust_store::bean::coupon::user_coupon_id_param::UserCouponIdParam =
        serde_json::from_str(r##"{"openid":null,"user_coupon_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["openid", "user_coupon_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_user_coupon_list_param() {
    let data: wx_rust_store::bean::coupon::user_coupon_list_param::UserCouponListParam =
        serde_json::from_str(
            r##"{"status":null,"page":null,"page_size":null,"page_ctx":null,"openid":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["status", "page", "page_size", "page_ctx", "openid"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_user_coupon_list_response() {
    let data: wx_rust_store::bean::coupon::user_coupon_list_response::UserCouponListResponse =
        serde_json::from_str(r##"{"user_coupon_list":null,"total_num":null,"page_ctx":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["user_coupon_list", "total_num", "page_ctx"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_user_coupon_response() {
    let data: wx_rust_store::bean::coupon::user_coupon_response::UserCouponResponse =
        serde_json::from_str(r##"{"user_coupon":null,"openid":null,"unionid":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["user_coupon", "openid", "unionid"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_user_ext_info() {
    let data: wx_rust_store::bean::coupon::user_ext_info::UserExtInfo =
        serde_json::from_str(r##"{"use_time":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "use_time";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_coupon_valid_info() {
    let data: wx_rust_store::bean::coupon::valid_info::ValidInfo = serde_json::from_str(
        r##"{"valid_type":null,"valid_day_num":null,"start_time":null,"end_time":null}"##,
    )
    .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["valid_type", "valid_day_num", "start_time", "end_time"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_delivery_delivery_company_info() {
    let data: wx_rust_store::bean::delivery::delivery_company_info::DeliveryCompanyInfo =
        serde_json::from_str(r##"{"delivery_id":null,"delivery_name":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["delivery_id", "delivery_name"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_delivery_delivery_company_response() {
    let data: wx_rust_store::bean::delivery::delivery_company_response::DeliveryCompanyResponse =
        serde_json::from_str(r##"{"company_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "company_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_delivery_delivery_info() {
    let data: wx_rust_store::bean::delivery::delivery_info::DeliveryInfo = serde_json::from_str(
        r##"{"waybill_id":null,"delivery_id":null,"deliver_type":null,"product_infos":null}"##,
    )
    .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["waybill_id", "delivery_id", "deliver_type", "product_infos"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_delivery_delivery_send_param() {
    let data: wx_rust_store::bean::delivery::delivery_send_param::DeliverySendParam =
        serde_json::from_str(r##"{"order_id":null,"delivery_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "delivery_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_delivery_freight_product_info() {
    let data: wx_rust_store::bean::delivery::freight_product_info::FreightProductInfo =
        serde_json::from_str(r##"{"product_id":null,"sku_id":null,"product_cnt":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "sku_id", "product_cnt"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_delivery_fresh_inspect_param() {
    let data: wx_rust_store::bean::delivery::fresh_inspect_param::FreshInspectParam =
        serde_json::from_str(r##"{"order_id":null,"audit_items":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "audit_items"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_delivery_package_audit_info() {
    let data: wx_rust_store::bean::delivery::package_audit_info::PackageAuditInfo =
        serde_json::from_str(r##"{"item_name":null,"item_value":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["item_name", "item_value"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_ewaybill_batch_print_order_request() {
    let data: wx_rust_store::bean::ewaybill::batch_print_order_request::BatchPrintOrderRequest =
        serde_json::from_str(r##"{"req_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "req_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_ewaybill_print_order_request() {
    let data: wx_rust_store::bean::ewaybill::print_order_request::PrintOrderRequest =
        serde_json::from_str(
            r##"{"ewaybill_order_id":null,"delivery_id":null,"waybill_id":null,"re_print":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["ewaybill_order_id", "delivery_id", "waybill_id", "re_print"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_ewaybill_template_id_response() {
    let data: wx_rust_store::bean::ewaybill::template_id_response::TemplateIdResponse =
        serde_json::from_str(r##"{"template_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "template_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_favorite_favorite_count_response() {
    let data: wx_rust_store::bean::favorite::favorite_count_response::FavoriteCountResponse = serde_json::from_str(r##"{"favor_uv_acc_shop_homepage":null,"favor_uv_acc_order_detail":null,"favor_uv_acc_product_detail":null,"favor_uv_acc_other_scene":null,"favor_uv_acc_all":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "favor_uv_acc_shop_homepage",
        "favor_uv_acc_order_detail",
        "favor_uv_acc_product_detail",
        "favor_uv_acc_other_scene",
        "favor_uv_acc_all",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_freight_address_info_list() {
    let data: wx_rust_store::bean::freight::address_info_list::AddressInfoList =
        serde_json::from_str(r##"{"address_infos":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "address_infos";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_freight_all_condition_free_detail() {
    let data: wx_rust_store::bean::freight::all_condition_free_detail::AllConditionFreeDetail =
        serde_json::from_str(r##"{"condition_free_detail_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "condition_free_detail_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_freight_all_freight_calc_method() {
    let data: wx_rust_store::bean::freight::all_freight_calc_method::AllFreightCalcMethod =
        serde_json::from_str(r##"{"freight_calc_method_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "freight_calc_method_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_freight_condition_free_detail() {
    let data: wx_rust_store::bean::freight::condition_free_detail::ConditionFreeDetail = serde_json::from_str(r##"{"address_infos":null,"min_piece":null,"min_weight":null,"min_amount":null,"valuation_flag":null,"amount_flag":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "address_infos",
        "min_piece",
        "min_weight",
        "min_amount",
        "valuation_flag",
        "amount_flag",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_freight_freight_calc_method() {
    let data: wx_rust_store::bean::freight::freight_calc_method::FreightCalcMethod = serde_json::from_str(r##"{"address_infos":null,"is_default":null,"delivery_id":null,"first_val_amount":null,"first_price":null,"second_val_amount":null,"second_price":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "address_infos",
        "is_default",
        "delivery_id",
        "first_val_amount",
        "first_price",
        "second_val_amount",
        "second_price",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_freight_freight_template() {
    let data: wx_rust_store::bean::freight::freight_template::FreightTemplate = serde_json::from_str(r##"{"template_id":null,"name":null,"valuation_type":null,"send_time":null,"address_info":null,"delivery_type":null,"shipping_method":null,"all_condition_free_detail":null,"all_freight_calc_method":null,"create_time":null,"update_time":null,"is_default":null,"not_send_area":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "template_id",
        "name",
        "valuation_type",
        "send_time",
        "address_info",
        "delivery_type",
        "shipping_method",
        "all_condition_free_detail",
        "all_freight_calc_method",
        "create_time",
        "update_time",
        "is_default",
        "not_send_area",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_freight_not_send_area() {
    let data: wx_rust_store::bean::freight::not_send_area::NotSendArea =
        serde_json::from_str(r##"{"address_infos":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "address_infos";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_freight_template_add_param() {
    let data: wx_rust_store::bean::freight::template_add_param::TemplateAddParam =
        serde_json::from_str(r##"{"freight_template":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "freight_template";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_freight_template_id_response() {
    let data: wx_rust_store::bean::freight::template_id_response::TemplateIdResponse =
        serde_json::from_str(r##"{"template_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "template_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_freight_template_info_response() {
    let data: wx_rust_store::bean::freight::template_info_response::TemplateInfoResponse =
        serde_json::from_str(r##"{"freight_template":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "freight_template";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_freight_template_list_param() {
    let data: wx_rust_store::bean::freight::template_list_param::TemplateListParam =
        serde_json::from_str(r##"{"offset":null,"limit":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["offset", "limit"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_freight_template_list_response() {
    let data: wx_rust_store::bean::freight::template_list_response::TemplateListResponse =
        serde_json::from_str(r##"{"template_id_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "template_id_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_account_info() {
    let data: wx_rust_store::bean::fund::account_info::AccountInfo = serde_json::from_str(r##"{"bank_account_type":null,"account_bank":null,"bank_address_code":null,"bank_branch_id":null,"bank_name":null,"account_number":null,"account_bank4show":null,"account_name":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "bank_account_type",
        "account_bank",
        "bank_address_code",
        "bank_branch_id",
        "bank_name",
        "account_number",
        "account_bank4show",
        "account_name",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_account_info_param() {
    let data: wx_rust_store::bean::fund::account_info_param::AccountInfoParam =
        serde_json::from_str(r##"{"account_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "account_info";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_account_info_response() {
    let data: wx_rust_store::bean::fund::account_info_response::AccountInfoResponse =
        serde_json::from_str(r##"{"account_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "account_info";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_balance_info_response() {
    let data: wx_rust_store::bean::fund::balance_info_response::BalanceInfoResponse =
        serde_json::from_str(
            r##"{"available_amount":null,"pending_amount":null,"sub_mchid":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["available_amount", "pending_amount", "sub_mchid"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_flow_list_response() {
    let data: wx_rust_store::bean::fund::flow_list_response::FlowListResponse =
        serde_json::from_str(r##"{"flow_ids":null,"next_key":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["flow_ids", "next_key"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_flow_related_info() {
    let data: wx_rust_store::bean::fund::flow_related_info::FlowRelatedInfo = serde_json::from_str(r##"{"related_type":null,"order_id":null,"aftersale_id":null,"withdraw_id":null,"bookkeeping_time":null,"insurance_id":null,"transaction_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "related_type",
        "order_id",
        "aftersale_id",
        "withdraw_id",
        "bookkeeping_time",
        "insurance_id",
        "transaction_id",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_funds_flow() {
    let data: wx_rust_store::bean::fund::funds_flow::FundsFlow = serde_json::from_str(r##"{"flow_id":null,"funds_type":null,"flow_type":null,"amount":null,"balance":null,"related_info_list":null,"bookkeeping_time":null,"remark":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "flow_id",
        "funds_type",
        "flow_type",
        "amount",
        "balance",
        "related_info_list",
        "bookkeeping_time",
        "remark",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_funds_flow_response() {
    let data: wx_rust_store::bean::fund::funds_flow_response::FundsFlowResponse =
        serde_json::from_str(r##"{"funds_flow":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "funds_flow";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_funds_list_param() {
    let data: wx_rust_store::bean::fund::funds_list_param::FundsListParam = serde_json::from_str(r##"{"page":null,"page_size":null,"start_time":null,"end_time":null,"flow_type":null,"transaction_id":null,"next_key":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "page",
        "page_size",
        "start_time",
        "end_time",
        "flow_type",
        "transaction_id",
        "next_key",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_withdraw_detail_response() {
    let data: wx_rust_store::bean::fund::withdraw_detail_response::WithdrawDetailResponse = serde_json::from_str(r##"{"amount":null,"create_time":null,"update_time":null,"reason":null,"remark":null,"bank_memo":null,"bank_name":null,"bank_num":null,"status":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "amount",
        "create_time",
        "update_time",
        "reason",
        "remark",
        "bank_memo",
        "bank_name",
        "bank_num",
        "status",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_withdraw_list_param() {
    let data: wx_rust_store::bean::fund::withdraw_list_param::WithdrawListParam =
        serde_json::from_str(
            r##"{"page_num":null,"page_size":null,"start_time":null,"end_time":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["page_num", "page_size", "start_time", "end_time"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_withdraw_list_response() {
    let data: wx_rust_store::bean::fund::withdraw_list_response::WithdrawListResponse =
        serde_json::from_str(r##"{"withdraw_ids":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "withdraw_ids";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_withdraw_submit_param() {
    let data: wx_rust_store::bean::fund::withdraw_submit_param::WithdrawSubmitParam =
        serde_json::from_str(r##"{"amount":null,"remark":null,"bank_memo":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["amount", "remark", "bank_memo"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_withdraw_submit_response() {
    let data: wx_rust_store::bean::fund::withdraw_submit_response::WithdrawSubmitResponse =
        serde_json::from_str(r##"{"qrcode_ticket":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "qrcode_ticket";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_bank_bank_city_info() {
    let data: wx_rust_store::bean::fund::bank::bank_city_info::BankCityInfo =
        serde_json::from_str(r##"{"city_name":null,"city_code":null,"bank_address_code":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["city_name", "city_code", "bank_address_code"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_bank_bank_city_response() {
    let data: wx_rust_store::bean::fund::bank::bank_city_response::BankCityResponse =
        serde_json::from_str(r##"{"data":null,"total_count":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["data", "total_count"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_bank_bank_info() {
    let data: wx_rust_store::bean::fund::bank::bank_info::BankInfo = serde_json::from_str(r##"{"account_bank":null,"bank_code":null,"bank_id":null,"bank_name":null,"bank_type":null,"need_branch":null,"branch_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "account_bank",
        "bank_code",
        "bank_id",
        "bank_name",
        "bank_type",
        "need_branch",
        "branch_id",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_bank_bank_info_response() {
    let data: wx_rust_store::bean::fund::bank::bank_info_response::BankInfoResponse =
        serde_json::from_str(r##"{"data":null,"total_count":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["data", "total_count"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_bank_bank_list_response() {
    let data: wx_rust_store::bean::fund::bank::bank_list_response::BankListResponse =
        serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_bank_bank_province_info() {
    let data: wx_rust_store::bean::fund::bank::bank_province_info::BankProvinceInfo =
        serde_json::from_str(r##"{"province_name":null,"province_code":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["province_name", "province_code"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_bank_bank_province_response() {
    let data: wx_rust_store::bean::fund::bank::bank_province_response::BankProvinceResponse =
        serde_json::from_str(r##"{"data":null,"total_count":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["data", "total_count"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_bank_bank_search_param() {
    let data: wx_rust_store::bean::fund::bank::bank_search_param::BankSearchParam =
        serde_json::from_str(r##"{"offset":null,"limit":null,"key_words":null,"bank_type":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["offset", "limit", "key_words", "bank_type"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_bank_branch_info() {
    let data: wx_rust_store::bean::fund::bank::branch_info::BranchInfo =
        serde_json::from_str(r##"{"branch_id":null,"branch_name":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["branch_id", "branch_name"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_bank_branch_info_response() {
    let data: wx_rust_store::bean::fund::bank::branch_info_response::BranchInfoResponse = serde_json::from_str(r##"{"total_count":null,"count":null,"account_bank":null,"account_bank_code":null,"bank_alias":null,"bank_alias_code":null,"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "total_count",
        "count",
        "account_bank",
        "account_bank_code",
        "bank_alias",
        "bank_alias_code",
        "data",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_bank_branch_search_param() {
    let data: wx_rust_store::bean::fund::bank::branch_search_param::BranchSearchParam =
        serde_json::from_str(r##"{"bank_code":null,"city_code":null,"offset":null,"limit":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["bank_code", "city_code", "offset", "limit"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_qrcode_qr_check_response() {
    let data: wx_rust_store::bean::fund::qrcode::qr_check_response::QrCheckResponse = serde_json::from_str(r##"{"status":null,"self_check_err_code":null,"self_check_err_msg":null,"scan_user_type":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "status",
        "self_check_err_code",
        "self_check_err_msg",
        "scan_user_type",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_fund_qrcode_qr_code_response() {
    let data: wx_rust_store::bean::fund::qrcode::qr_code_response::QrCodeResponse =
        serde_json::from_str(r##"{"qrcode_buf":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "qrcode_buf";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_background_background_apply_response() {
    let data: wx_rust_store::bean::home::background::background_apply_response::BackgroundApplyResponse = serde_json::from_str(r##"{"apply_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "apply_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_background_background_apply_result() {
    let data: wx_rust_store::bean::home::background::background_apply_result::BackgroundApplyResult = serde_json::from_str(r##"{"apply_id":null,"state":null,"audit_desc":null,"img_url":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["apply_id", "state", "audit_desc", "img_url"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_background_background_get_response() {
    let data: wx_rust_store::bean::home::background::background_get_response::BackgroundGetResponse = serde_json::from_str(r##"{"img_url":null,"apply":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["img_url", "apply"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_banner_banner_apply_detail() {
    let data: wx_rust_store::bean::home::banner::banner_apply_detail::BannerApplyDetail =
        serde_json::from_str(r##"{"audit_state":null,"audit_desc":null,"banner":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["audit_state", "audit_desc", "banner"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_banner_banner_apply_info() {
    let data: wx_rust_store::bean::home::banner::banner_apply_info::BannerApplyInfo =
        serde_json::from_str(r##"{"apply_id":null,"state":null,"scale":null,"banner":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["apply_id", "state", "scale", "banner"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_banner_banner_apply_param() {
    let data: wx_rust_store::bean::home::banner::banner_apply_param::BannerApplyParam =
        serde_json::from_str(r##"{"banner":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "banner";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_banner_banner_apply_response() {
    let data: wx_rust_store::bean::home::banner::banner_apply_response::BannerApplyResponse =
        serde_json::from_str(r##"{"apply_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "apply_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_banner_banner_get_response() {
    let data: wx_rust_store::bean::home::banner::banner_get_response::BannerGetResponse =
        serde_json::from_str(r##"{"banner":null,"apply":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["banner", "apply"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_banner_banner_info() {
    let data: wx_rust_store::bean::home::banner::banner_info::BannerInfo =
        serde_json::from_str(r##"{"scale":null,"banner":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["scale", "banner"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_banner_banner_item() {
    let data: wx_rust_store::bean::home::banner::banner_item::BannerItem = serde_json::from_str(
        r##"{"type":null,"banner":null,"product":null,"finder":null,"official_account":null}"##,
    )
    .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["type", "banner", "product", "finder", "official_account"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_banner_banner_item_detail() {
    let data: wx_rust_store::bean::home::banner::banner_item_detail::BannerItemDetail =
        serde_json::from_str(r##"{"img_url":null,"title":null,"description":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["img_url", "title", "description"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_banner_banner_item_finder() {
    let data: wx_rust_store::bean::home::banner::banner_item_finder::BannerItemFinder =
        serde_json::from_str(r##"{"finder_user_name":null,"feed_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["finder_user_name", "feed_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_banner_banner_item_official_account() {
    let data: wx_rust_store::bean::home::banner::banner_item_official_account::BannerItemOfficialAccount = serde_json::from_str(r##"{"url":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "url";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_banner_banner_item_product() {
    let data: wx_rust_store::bean::home::banner::banner_item_product::BannerItemProduct =
        serde_json::from_str(r##"{"product_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_cat_tree_node() {
    let data: wx_rust_store::bean::home::tree::cat_tree_node::CatTreeNode =
        serde_json::from_str(r##"{"id":null,"name":null,"is_displayed":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["id", "name", "is_displayed"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_level_tree_info() {
    let data: wx_rust_store::bean::home::tree::level_tree_info::LevelTreeInfo =
        serde_json::from_str(r##"{"level_1":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "level_1";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_one_level_tree_node() {
    let data: wx_rust_store::bean::home::tree::one_level_tree_node::OneLevelTreeNode =
        serde_json::from_str(r##"{"id":null,"name":null,"is_displayed":null,"level_2":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["id", "name", "is_displayed", "level_2"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_tree_audit_result() {
    let data: wx_rust_store::bean::home::tree::tree_audit_result::TreeAuditResult =
        serde_json::from_str(r##"{"version":null,"audit_results":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["version", "audit_results"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_tree_audit_result_detail() {
    let data: wx_rust_store::bean::home::tree::tree_audit_result_detail::TreeAuditResultDetail =
        serde_json::from_str(r##"{"level_id":null,"result_code":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["level_id", "result_code"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_tree_product_edit_info() {
    let data: wx_rust_store::bean::home::tree::tree_product_edit_info::TreeProductEditInfo =
        serde_json::from_str(r##"{"level_1_id":null,"level_2_id":null,"product_ids":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["level_1_id", "level_2_id", "product_ids"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_tree_product_edit_param() {
    let data: wx_rust_store::bean::home::tree::tree_product_edit_param::TreeProductEditParam =
        serde_json::from_str(r##"{"req":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "req";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_tree_product_list_info() {
    let data: wx_rust_store::bean::home::tree::tree_product_list_info::TreeProductListInfo =
        serde_json::from_str(
            r##"{"level_1_id":null,"level_2_id":null,"page_size":null,"page_context":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["level_1_id", "level_2_id", "page_size", "page_context"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_tree_product_list_param() {
    let data: wx_rust_store::bean::home::tree::tree_product_list_param::TreeProductListParam =
        serde_json::from_str(r##"{"req":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "req";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_tree_product_list_response() {
    let data: wx_rust_store::bean::home::tree::tree_product_list_response::TreeProductListResponse =
        serde_json::from_str(r##"{"resp":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "resp";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_tree_product_list_result() {
    let data: wx_rust_store::bean::home::tree::tree_product_list_result::TreeProductListResult =
        serde_json::from_str(r##"{"product_ids":null,"total_count":null,"page_context":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_ids", "total_count", "page_context"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_tree_show_get_response() {
    let data: wx_rust_store::bean::home::tree::tree_show_get_response::TreeShowGetResponse =
        serde_json::from_str(r##"{"resp":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "resp";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_tree_show_info() {
    let data: wx_rust_store::bean::home::tree::tree_show_info::TreeShowInfo =
        serde_json::from_str(r##"{"tree":null,"version":null,"classification_id_deleted":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["tree", "version", "classification_id_deleted"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_tree_show_param() {
    let data: wx_rust_store::bean::home::tree::tree_show_param::TreeShowParam =
        serde_json::from_str(r##"{"req":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "req";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_tree_tree_show_set_response() {
    let data: wx_rust_store::bean::home::tree::tree_show_set_response::TreeShowSetResponse =
        serde_json::from_str(r##"{"resp":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "resp";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_window_window_product_index_param() {
    let data: wx_rust_store::bean::home::window::window_product_index_param::WindowProductIndexParam = serde_json::from_str(r##"{"product_id":null,"index_num":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "index_num"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_window_window_product_list_param() {
    let data: wx_rust_store::bean::home::window::window_product_list_param::WindowProductListParam =
        serde_json::from_str(r##"{"page_size":null,"next_key":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["page_size", "next_key"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_window_window_product_setting() {
    let data: wx_rust_store::bean::home::window::window_product_setting::WindowProductSetting =
        serde_json::from_str(r##"{"product_id":null,"is_set_hide":null,"is_set_top":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "is_set_hide", "is_set_top"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_home_window_window_product_setting_response() {
    let data: wx_rust_store::bean::home::window::window_product_setting_response::WindowProductSettingResponse = serde_json::from_str(r##"{"products":null,"next_key":null,"total_num":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["products", "next_key", "total_num"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_image_qualification_file_id() {
    let data: wx_rust_store::bean::image::qualification_file_id::QualificationFileId =
        serde_json::from_str(r##"{"file_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "file_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_image_qualification_file_response() {
    let data: wx_rust_store::bean::image::qualification_file_response::QualificationFileResponse =
        serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_image_store_image_info() {
    let data: wx_rust_store::bean::image::store_image_info::StoreImageInfo =
        serde_json::from_str(r##"{"media_id":null,"img_url":null,"pay_media_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["media_id", "img_url", "pay_media_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_image_store_image_response() {
    let data: wx_rust_store::bean::image::store_image_response::StoreImageResponse =
        serde_json::from_str(r##"{"contentType":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "contentType";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_image_upload_image_response() {
    let data: wx_rust_store::bean::image::upload_image_response::UploadImageResponse =
        serde_json::from_str(r##"{"pic_file":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "pic_file";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_kf_wx_store_kf_cos_upload_response() {
    let data: wx_rust_store::bean::kf::wx_store_kf_cos_upload_response::WxStoreKfCosUploadResponse =
        serde_json::from_str(r##"{"cos_url":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "cos_url";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_kf_wx_store_kf_send_msg_param() {
    let data: wx_rust_store::bean::kf::wx_store_kf_send_msg_param::WxStoreKfSendMsgParam = serde_json::from_str(r##"{"request_id":null,"open_id":null,"msg_type":null,"text":null,"image":null,"video":null,"file":null,"product_share":null,"order_share":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "request_id",
        "open_id",
        "msg_type",
        "text",
        "image",
        "video",
        "file",
        "product_share",
        "order_share",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_kf_wx_store_kf_send_msg_response() {
    let data: wx_rust_store::bean::kf::wx_store_kf_send_msg_response::WxStoreKfSendMsgResponse =
        serde_json::from_str(r##"{"msg_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "msg_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_limit_limit_sku() {
    let data: wx_rust_store::bean::limit::limit_sku::LimitSku =
        serde_json::from_str(r##"{"sku_id":null,"sale_price":null,"sale_stock":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["sku_id", "sale_price", "sale_stock"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_limit_limit_task_add_response() {
    let data: wx_rust_store::bean::limit::limit_task_add_response::LimitTaskAddResponse =
        serde_json::from_str(r##"{"task_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "task_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_limit_limit_task_info() {
    let data: wx_rust_store::bean::limit::limit_task_info::LimitTaskInfo = serde_json::from_str(r##"{"task_id":null,"product_id":null,"status":null,"create_time":null,"start_time":null,"end_time":null,"limited_discount_skus":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "task_id",
        "product_id",
        "status",
        "create_time",
        "start_time",
        "end_time",
        "limited_discount_skus",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_limit_limit_task_list_param() {
    let data: wx_rust_store::bean::limit::limit_task_list_param::LimitTaskListParam =
        serde_json::from_str(r##"{"page_size":null,"next_key":null,"status":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["page_size", "next_key", "status"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_limit_limit_task_list_response() {
    let data: wx_rust_store::bean::limit::limit_task_list_response::LimitTaskListResponse =
        serde_json::from_str(
            r##"{"limited_discount_tasks":null,"next_key":null,"total_num":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["limited_discount_tasks", "next_key", "total_num"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_limit_limit_task_param() {
    let data: wx_rust_store::bean::limit::limit_task_param::LimitTaskParam = serde_json::from_str(
        r##"{"product_id":null,"start_time":null,"end_time":null,"limited_discount_skus":null}"##,
    )
    .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_id",
        "start_time",
        "end_time",
        "limited_discount_skus",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_limit_limit_task_update_param() {
    let data: wx_rust_store::bean::limit::limit_task_update_param::LimitTaskUpdateParam = serde_json::from_str(r##"{"task_id":null,"status":null,"start_time":null,"end_time":null,"title":null,"limited_discount_skus":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "task_id",
        "status",
        "start_time",
        "end_time",
        "title",
        "limited_discount_skus",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_limit_limit_task_update_response() {
    let data: wx_rust_store::bean::limit::limit_task_update_response::LimitTaskUpdateResponse =
        serde_json::from_str(r##"{"task_id":null,"title":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["task_id", "title"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_after_sale_detail() {
    let data: wx_rust_store::bean::order::after_sale_detail::AfterSaleDetail =
        serde_json::from_str(r##"{"on_aftersale_order_cnt":null,"aftersale_order_list":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["on_aftersale_order_cnt", "aftersale_order_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_after_sale_order_info() {
    let data: wx_rust_store::bean::order::after_sale_order_info::AfterSaleOrderInfo =
        serde_json::from_str(r##"{"aftersale_order_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "aftersale_order_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_change_order_info() {
    let data: wx_rust_store::bean::order::change_order_info::ChangeOrderInfo =
        serde_json::from_str(r##"{"product_id":null,"sku_id":null,"change_price":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "sku_id", "change_price"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_change_sku_info() {
    let data: wx_rust_store::bean::order::change_sku_info::ChangeSkuInfo = serde_json::from_str(r##"{"preshipment_change_sku_state":null,"old_sku_id":null,"new_sku_id":null,"ddl_time_stamp":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "preshipment_change_sku_state",
        "old_sku_id",
        "new_sku_id",
        "ddl_time_stamp",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_decode_address_info() {
    let data: wx_rust_store::bean::order::decode_address_info::DecodeAddressInfo = serde_json::from_str(r##"{"user_name":null,"tel_number":null,"postal_code":null,"province_name":null,"city_name":null,"county_name":null,"detail_info":null,"national_code":null,"house_number":null,"lat":null,"lng":null,"virtual_order_tel_number":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "user_name",
        "tel_number",
        "postal_code",
        "province_name",
        "city_name",
        "county_name",
        "detail_info",
        "national_code",
        "house_number",
        "lat",
        "lng",
        "virtual_order_tel_number",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_decode_sensitive_info_response() {
    let data: wx_rust_store::bean::order::decode_sensitive_info_response::DecodeSensitiveInfoResponse = serde_json::from_str(r##"{"address_info":null,"virtual_number_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["address_info", "virtual_number_info"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_delivery_product_info() {
    let data: wx_rust_store::bean::order::delivery_product_info::DeliveryProductInfo = serde_json::from_str(r##"{"waybill_id":null,"delivery_id":null,"product_infos":null,"delivery_name":null,"delivery_time":null,"deliver_type":null,"delivery_address":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "waybill_id",
        "delivery_id",
        "product_infos",
        "delivery_name",
        "delivery_time",
        "deliver_type",
        "delivery_address",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_delivery_update_param() {
    let data: wx_rust_store::bean::order::delivery_update_param::DeliveryUpdateParam =
        serde_json::from_str(r##"{"order_id":null,"delivery_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "delivery_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_dropship_info() {
    let data: wx_rust_store::bean::order::dropship_info::DropshipInfo =
        serde_json::from_str(r##"{"ds_order_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "ds_order_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_free_gift_info() {
    let data: wx_rust_store::bean::order::free_gift_info::FreeGiftInfo =
        serde_json::from_str(r##"{"main_product_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "main_product_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_main_product_info() {
    let data: wx_rust_store::bean::order::main_product_info::MainProductInfo =
        serde_json::from_str(
            r##"{"gift_cnt":null,"task_id":null,"product_id":null,"sku_id":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["gift_cnt", "task_id", "product_id", "sku_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_address_info() {
    let data: wx_rust_store::bean::order::order_address_info::OrderAddressInfo = serde_json::from_str(r##"{"user_name":null,"tel_number":null,"postal_code":null,"province_name":null,"city_name":null,"county_name":null,"detail_info":null,"national_code":null,"house_number":null,"lat":null,"lng":null,"virtual_order_tel_number":null,"tel_number_ext_info":null,"use_tel_number":null,"hash_code":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "user_name",
        "tel_number",
        "postal_code",
        "province_name",
        "city_name",
        "county_name",
        "detail_info",
        "national_code",
        "house_number",
        "lat",
        "lng",
        "virtual_order_tel_number",
        "tel_number_ext_info",
        "use_tel_number",
        "hash_code",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_address_param() {
    let data: wx_rust_store::bean::order::order_address_param::OrderAddressParam =
        serde_json::from_str(r##"{"order_id":null,"user_address":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "user_address"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_agent_info() {
    let data: wx_rust_store::bean::order::order_agent_info::OrderAgentInfo =
        serde_json::from_str(r##"{"agent_finder_id":null,"agent_finder_nickname":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["agent_finder_id", "agent_finder_nickname"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_commission_info() {
    let data: wx_rust_store::bean::order::order_commission_info::OrderCommissionInfo = serde_json::from_str(r##"{"sku_id":null,"nickname":null,"type":null,"status":null,"amount":null,"finder_id":null,"openfinderid":null,"talent_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "sku_id",
        "nickname",
        "type",
        "status",
        "amount",
        "finder_id",
        "openfinderid",
        "talent_id",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_compensation_delivery_param() {
    let data: wx_rust_store::bean::order::order_compensation_delivery_param::OrderCompensationDeliveryParam = serde_json::from_str(r##"{"order_id":null,"delivery_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "delivery_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_coupon_info() {
    let data: wx_rust_store::bean::order::order_coupon_info::OrderCouponInfo = serde_json::from_str(r##"{"user_coupon_id":null,"coupon_type":null,"discounted_price":null,"coupon_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "user_coupon_id",
        "coupon_type",
        "discounted_price",
        "coupon_id",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_custom_info() {
    let data: wx_rust_store::bean::order::order_custom_info::OrderCustomInfo = serde_json::from_str(r##"{"custom_img_url":null,"custom_word":null,"custom_type":null,"custom_preview_img_url":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "custom_img_url",
        "custom_word",
        "custom_type",
        "custom_preview_img_url",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_detail_info() {
    let data: wx_rust_store::bean::order::order_detail_info::OrderDetailInfo = serde_json::from_str(r##"{"product_infos":null,"pay_info":null,"price_info":null,"delivery_info":null,"coupon_info":null,"ext_info":null,"commission_infos":null,"sharer_info":null,"settle_info":null,"sku_sharer_infos":null,"agent_info":null,"source_infos":null,"refund_info":null,"greeting_card_info":null,"custom_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_infos",
        "pay_info",
        "price_info",
        "delivery_info",
        "coupon_info",
        "ext_info",
        "commission_infos",
        "sharer_info",
        "settle_info",
        "sku_sharer_infos",
        "agent_info",
        "source_infos",
        "refund_info",
        "greeting_card_info",
        "custom_info",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_greeting_card_info() {
    let data: wx_rust_store::bean::order::order_greeting_card_info::OrderGreetingCardInfo =
        serde_json::from_str(
            r##"{"giver_name":null,"receiver_name":null,"greeting_message":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["giver_name", "receiver_name", "greeting_message"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_id_param() {
    let data: wx_rust_store::bean::order::order_id_param::OrderIdParam =
        serde_json::from_str(r##"{"order_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "order_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_info() {
    let data: wx_rust_store::bean::order::order_info::OrderInfo = serde_json::from_str(r##"{"order_id":null,"status":null,"openid":null,"unionid":null,"order_detail":null,"aftersale_detail":null,"is_present":null,"present_order_id_str":null,"present_note":null,"present_giver_openid":null,"present_giver_unionid":null,"create_time":null,"update_time":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "order_id",
        "status",
        "openid",
        "unionid",
        "order_detail",
        "aftersale_detail",
        "is_present",
        "present_order_id_str",
        "present_note",
        "present_giver_openid",
        "present_giver_unionid",
        "create_time",
        "update_time",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_info_param() {
    let data: wx_rust_store::bean::order::order_info_param::OrderInfoParam =
        serde_json::from_str(r##"{"order_id":null,"encode_sensitive_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "encode_sensitive_info"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_info_response() {
    let data: wx_rust_store::bean::order::order_info_response::OrderInfoResponse =
        serde_json::from_str(r##"{"order":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "order";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_list_param() {
    let data: wx_rust_store::bean::order::order_list_param::OrderListParam = serde_json::from_str(r##"{"page_size":null,"next_key":null,"create_time_range":null,"update_time_range":null,"status":null,"openid":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "page_size",
        "next_key",
        "create_time_range",
        "update_time_range",
        "status",
        "openid",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_list_response() {
    let data: wx_rust_store::bean::order::order_list_response::OrderListResponse =
        serde_json::from_str(r##"{"order_id_list":null,"next_key":null,"has_more":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id_list", "next_key", "has_more"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_price_info() {
    let data: wx_rust_store::bean::order::order_price_info::OrderPriceInfo = serde_json::from_str(r##"{"product_price":null,"order_price":null,"freight":null,"discounted_price":null,"is_discounted":null,"original_order_price":null,"estimate_product_price":null,"change_down_price":null,"change_freight":null,"is_change_freight":null,"use_deduction":null,"deduction_price":null,"merchant_receieve_price":null,"merchant_discounted_price":null,"finder_discounted_price":null,"vip_discounted_price":null,"bulkbuy_discounted_price":null,"national_subsidy_discounted_price":null,"cash_coupon_discounted_price":null,"national_subsidy_merchant_discounted_price":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_price",
        "order_price",
        "freight",
        "discounted_price",
        "is_discounted",
        "original_order_price",
        "estimate_product_price",
        "change_down_price",
        "change_freight",
        "is_change_freight",
        "use_deduction",
        "deduction_price",
        "merchant_receieve_price",
        "merchant_discounted_price",
        "finder_discounted_price",
        "vip_discounted_price",
        "bulkbuy_discounted_price",
        "national_subsidy_discounted_price",
        "cash_coupon_discounted_price",
        "national_subsidy_merchant_discounted_price",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_price_param() {
    let data: wx_rust_store::bean::order::order_price_param::OrderPriceParam = serde_json::from_str(r##"{"order_id":null,"change_express":null,"express_fee":null,"change_order_infos":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "order_id",
        "change_express",
        "express_fee",
        "change_order_infos",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_product_extra_service() {
    let data: wx_rust_store::bean::order::order_product_extra_service::OrderProductExtraService =
        serde_json::from_str(r##"{"seven_day_return":null,"freight_insurance":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["seven_day_return", "freight_insurance"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_product_info() {
    let data: wx_rust_store::bean::order::order_product_info::OrderProductInfo = serde_json::from_str(r##"{"product_id":null,"sku_id":null,"thumb_img":null,"sku_cnt":null,"sale_price":null,"title":null,"on_aftersale_sku_cnt":null,"finish_aftersale_sku_cnt":null,"sku_code":null,"market_price":null,"sku_attrs":null,"real_price":null,"out_product_id":null,"out_sku_id":null,"is_discounted":null,"estimate_price":null,"is_change_price":null,"change_price":null,"out_warehouse_id":null,"sku_deliver_info":null,"extra_service":null,"use_deduction":null,"deduction_price":null,"order_product_coupon_info_list":null,"delivery_deadline":null,"merchant_discounted_price":null,"finder_discounted_price":null,"is_free_gift":null,"vip_discounted_price":null,"product_unique_id":null,"change_sku_info":null,"free_gift_info":null,"bulkbuy_discounted_price":null,"national_subsidy_discounted_price":null,"dropship_info":null,"is_flash_sale":null,"national_subsidy_merchant_discounted_price":null,"platform_activity_merchant_discounted_price":null,"cash_coupon_discounted_price":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_id",
        "sku_id",
        "thumb_img",
        "sku_cnt",
        "sale_price",
        "title",
        "on_aftersale_sku_cnt",
        "finish_aftersale_sku_cnt",
        "sku_code",
        "market_price",
        "sku_attrs",
        "real_price",
        "out_product_id",
        "out_sku_id",
        "is_discounted",
        "estimate_price",
        "is_change_price",
        "change_price",
        "out_warehouse_id",
        "sku_deliver_info",
        "extra_service",
        "use_deduction",
        "deduction_price",
        "order_product_coupon_info_list",
        "delivery_deadline",
        "merchant_discounted_price",
        "finder_discounted_price",
        "is_free_gift",
        "vip_discounted_price",
        "product_unique_id",
        "change_sku_info",
        "free_gift_info",
        "bulkbuy_discounted_price",
        "national_subsidy_discounted_price",
        "dropship_info",
        "is_flash_sale",
        "national_subsidy_merchant_discounted_price",
        "platform_activity_merchant_discounted_price",
        "cash_coupon_discounted_price",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_refund_info() {
    let data: wx_rust_store::bean::order::order_refund_info::OrderRefundInfo =
        serde_json::from_str(r##"{"refund_freight":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "refund_freight";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_remark_param() {
    let data: wx_rust_store::bean::order::order_remark_param::OrderRemarkParam =
        serde_json::from_str(r##"{"order_id":null,"merchant_notes":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "merchant_notes"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_search_condition() {
    let data: wx_rust_store::bean::order::order_search_condition::OrderSearchCondition = serde_json::from_str(r##"{"title":null,"sku_code":null,"user_name":null,"tel_number":null,"tel_number_last4":null,"order_id":null,"merchant_notes":null,"customer_notes":null,"address_under_review":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "title",
        "sku_code",
        "user_name",
        "tel_number",
        "tel_number_last4",
        "order_id",
        "merchant_notes",
        "customer_notes",
        "address_under_review",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_search_param() {
    let data: wx_rust_store::bean::order::order_search_param::OrderSearchParam = serde_json::from_str(r##"{"page_size":null,"next_key":null,"search_condition":null,"on_aftersale_order_exist":null,"status":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "page_size",
        "next_key",
        "search_condition",
        "on_aftersale_order_exist",
        "status",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_sharer_info() {
    let data: wx_rust_store::bean::order::order_sharer_info::OrderSharerInfo = serde_json::from_str(r##"{"sharer_openid":null,"sharer_unionid":null,"sharer_type":null,"share_scene":null,"handling_progress":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "sharer_openid",
        "sharer_unionid",
        "sharer_type",
        "share_scene",
        "handling_progress",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_sku_deliver_info() {
    let data: wx_rust_store::bean::order::order_sku_deliver_info::OrderSkuDeliverInfo =
        serde_json::from_str(r##"{"stock_type":null,"predict_delivery_time":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["stock_type", "predict_delivery_time"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_sku_share_info() {
    let data: wx_rust_store::bean::order::order_sku_share_info::OrderSkuShareInfo = serde_json::from_str(r##"{"sharer_openid":null,"sharer_unionid":null,"sharer_type":null,"share_scene":null,"sku_id":null,"from_wecom":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "sharer_openid",
        "sharer_unionid",
        "sharer_type",
        "share_scene",
        "sku_id",
        "from_wecom",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_order_source_info() {
    let data: wx_rust_store::bean::order::order_source_info::OrderSourceInfo = serde_json::from_str(r##"{"sku_id":null,"account_type":null,"account_id":null,"sale_channel":null,"account_nickname":null,"content_type":null,"content_id":null,"promoter_head_supplier_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "sku_id",
        "account_type",
        "account_id",
        "sale_channel",
        "account_nickname",
        "content_type",
        "content_id",
        "promoter_head_supplier_id",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_pre_shipment_change_sku_reject_param() {
    let data: wx_rust_store::bean::order::pre_shipment_change_sku_reject_param::PreShipmentChangeSkuRejectParam = serde_json::from_str(r##"{"order_id":null,"reject_reason":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "reject_reason"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_pre_shipment_change_sku_response() {
    let data: wx_rust_store::bean::order::pre_shipment_change_sku_response::PreShipmentChangeSkuResponse = serde_json::from_str(r##"{"change_sku_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "change_sku_info";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_present_note_add_param() {
    let data: wx_rust_store::bean::order::present_note_add_param::PresentNoteAddParam =
        serde_json::from_str(r##"{"order_id":null,"note":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "note"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_present_sub_order_response() {
    let data: wx_rust_store::bean::order::present_sub_order_response::PresentSubOrderResponse =
        serde_json::from_str(r##"{"sub_order_ids":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "sub_order_ids";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_private_number_add_phone_param() {
    let data: wx_rust_store::bean::order::private_number_add_phone_param::PrivateNumberAddPhoneParam = serde_json::from_str(r##"{"phone":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "phone";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_private_number_get_phone_response() {
    let data: wx_rust_store::bean::order::private_number_get_phone_response::PrivateNumberGetPhoneResponse = serde_json::from_str(r##"{"phone_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "phone_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_private_number_phone_info() {
    let data: wx_rust_store::bean::order::private_number_phone_info::PrivateNumberPhoneInfo =
        serde_json::from_str(r##"{"phone":null,"auth_status":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["phone", "auth_status"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_private_number_send_verify_code_param() {
    let data: wx_rust_store::bean::order::private_number_send_verify_code_param::PrivateNumberSendVerifyCodeParam = serde_json::from_str(r##"{"phone":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "phone";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_quality_insepct_info() {
    let data: wx_rust_store::bean::order::quality_insepct_info::QualityInsepctInfo =
        serde_json::from_str(r##"{"inspect_status":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "inspect_status";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_real_number_view_audit_response() {
    let data: wx_rust_store::bean::order::real_number_view_audit_response::RealNumberViewAuditResponse = serde_json::from_str(r##"{"audit_status":null,"real_number":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["audit_status", "real_number"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_recharge_info() {
    let data: wx_rust_store::bean::order::recharge_info::RechargeInfo =
        serde_json::from_str(r##"{"account_no":null,"account_type":null,"wx_openid":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["account_no", "account_type", "wx_openid"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_tel_number_ext_info() {
    let data: wx_rust_store::bean::order::tel_number_ext_info::TelNumberExtInfo = serde_json::from_str(r##"{"real_tel_number":null,"virtual_tel_number":null,"virtual_tel_expire_time":null,"get_virtual_tel_cnt":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "real_tel_number",
        "virtual_tel_number",
        "virtual_tel_expire_time",
        "get_virtual_tel_cnt",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_virtual_number_info() {
    let data: wx_rust_store::bean::order::virtual_number_info::VirtualNumberInfo =
        serde_json::from_str(r##"{"virtual_number":null,"extension":null,"expiration":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["virtual_number", "extension", "expiration"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_order_virtual_tel_number_response() {
    let data: wx_rust_store::bean::order::virtual_tel_number_response::VirtualTelNumberResponse = serde_json::from_str(r##"{"virtual_tel_number":null,"virtual_tel_expire_time":null,"get_virtual_tel_cnt":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "virtual_tel_number",
        "virtual_tel_expire_time",
        "get_virtual_tel_cnt",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_add_product_third_party_source_param() {
    let data: wx_rust_store::bean::product::add_product_third_party_source_param::AddProductThirdPartySourceParam = serde_json::from_str(r##"{"scene_value":null,"publish_method":null,"supplier":null,"supplier_shop_performance":null,"product_source_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "scene_value",
        "publish_method",
        "supplier",
        "supplier_shop_performance",
        "product_source_info",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_add_product_third_party_source_response() {
    let data: wx_rust_store::bean::product::add_product_third_party_source_response::AddProductThirdPartySourceResponse = serde_json::from_str(r##"{"third_party_source_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "third_party_source_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_after_sale_info() {
    let data: wx_rust_store::bean::product::after_sale_info::AfterSaleInfo =
        serde_json::from_str(r##"{"after_sale_address_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "after_sale_address_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_description_info() {
    let data: wx_rust_store::bean::product::description_info::DescriptionInfo =
        serde_json::from_str(r##"{"desc":null,"imgs":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["desc", "imgs"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_express_info() {
    let data: wx_rust_store::bean::product::express_info::ExpressInfo =
        serde_json::from_str(r##"{"template_id":null,"weight":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["template_id", "weight"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_assistant_external_product_mapping_new_param() {
    let data: wx_rust_store::bean::product::assistant::external_product_mapping_new_param::ExternalProductMappingNewParam = serde_json::from_str(r##"{"cat_id":null,"external_category_name":null,"head_imgs":null,"detail_imgs":null,"title":null,"external_attributes":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "cat_id",
        "external_category_name",
        "head_imgs",
        "detail_imgs",
        "title",
        "external_attributes",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_assistant_external_product_mapping_new_response() {
    let data: wx_rust_store::bean::product::assistant::external_product_mapping_new_response::ExternalProductMappingNewResponse = serde_json::from_str(r##"{"attributes":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "attributes";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_assistant_external_product_mapping_param() {
    let data: wx_rust_store::bean::product::assistant::external_product_mapping_param::ExternalProductMappingParam = serde_json::from_str(r##"{"cat_id":null,"external_attribute_name":null,"external_attribute_value":null,"external_category_name":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "cat_id",
        "external_attribute_name",
        "external_attribute_value",
        "external_category_name",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_assistant_external_product_mapping_response() {
    let data: wx_rust_store::bean::product::assistant::external_product_mapping_response::ExternalProductMappingResponse = serde_json::from_str(r##"{"external_attribute_name":null,"external_attribute_value":null,"internal_attribute_name":null,"internal_attribute_value":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "external_attribute_name",
        "external_attribute_value",
        "internal_attribute_name",
        "internal_attribute_value",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_extra_service_info() {
    let data: wx_rust_store::bean::product::extra_service_info::ExtraServiceInfo = serde_json::from_str(r##"{"seven_day_return":null,"pay_after_use":null,"freight_insurance":null,"fake_one_pay_three":null,"damage_guarantee":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "seven_day_return",
        "pay_after_use",
        "freight_insurance",
        "fake_one_pay_three",
        "damage_guarantee",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_gift_activity_add_param() {
    let data: wx_rust_store::bean::product::gift_activity_add_param::GiftActivityAddParam =
        serde_json::from_str(r##"{"gift_activity":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "gift_activity";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_gift_activity_add_response() {
    let data: wx_rust_store::bean::product::gift_activity_add_response::GiftActivityAddResponse =
        serde_json::from_str(r##"{"activity_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "activity_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_gift_activity_info() {
    let data: wx_rust_store::bean::product::gift_activity_info::GiftActivityInfo = serde_json::from_str(r##"{"activity_id":null,"title":null,"start_time":null,"end_time":null,"detail":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["activity_id", "title", "start_time", "end_time", "detail"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_gift_product_add_response() {
    let data: wx_rust_store::bean::product::gift_product_add_response::GiftProductAddResponse =
        serde_json::from_str(r##"{"product_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_gift_product_get_response() {
    let data: wx_rust_store::bean::product::gift_product_get_response::GiftProductGetResponse =
        serde_json::from_str(r##"{"product":null,"edit_product":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product", "edit_product"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_gift_product_info() {
    let data: wx_rust_store::bean::product::gift_product_info::GiftProductInfo = serde_json::from_str(r##"{"product_id":null,"out_product_id":null,"skus":null,"title":null,"sub_title":null,"head_imgs":null,"deliver_method":null,"deliver_acct_type":null,"desc_info":null,"cats":null,"cats_v2":null,"attrs":null,"spu_code":null,"brand_id":null,"qualifications":null,"express_info":null,"aftersale_desc":null,"limited_info":null,"extra_service":null,"status":null,"edit_status":null,"min_price":null,"create_time":null,"edit_time":null,"product_type":null,"after_sale_info":null,"src_product_id":null,"product_qua_infos":null,"size_chart":null,"short_title":null,"total_sold_num":null,"release_mode":null,"timing_onsale_info":null,"listing":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_id",
        "out_product_id",
        "skus",
        "title",
        "sub_title",
        "head_imgs",
        "deliver_method",
        "deliver_acct_type",
        "desc_info",
        "cats",
        "cats_v2",
        "attrs",
        "spu_code",
        "brand_id",
        "qualifications",
        "express_info",
        "aftersale_desc",
        "limited_info",
        "extra_service",
        "status",
        "edit_status",
        "min_price",
        "create_time",
        "edit_time",
        "product_type",
        "after_sale_info",
        "src_product_id",
        "product_qua_infos",
        "size_chart",
        "short_title",
        "total_sold_num",
        "release_mode",
        "timing_onsale_info",
        "listing",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_gift_product_list_param() {
    let data: wx_rust_store::bean::product::gift_product_list_param::GiftProductListParam =
        serde_json::from_str(r##"{"page_size":null,"next_key":null,"status":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["page_size", "next_key", "status"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_gift_product_list_response() {
    let data: wx_rust_store::bean::product::gift_product_list_response::GiftProductListResponse =
        serde_json::from_str(r##"{"total_num":null,"next_key":null,"product_ids":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["total_num", "next_key", "product_ids"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_limit_info() {
    let data: wx_rust_store::bean::product::limit_info::LimitInfo =
        serde_json::from_str(r##"{"period_type":null,"limited_buy_num":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["period_type", "limited_buy_num"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_product_audit_quota_response() {
    let data: wx_rust_store::bean::product::product_audit_quota_response::ProductAuditQuotaResponse = serde_json::from_str(r##"{"audit_quota":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "audit_quota";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_product_audit_strategy_info() {
    let data: wx_rust_store::bean::product::product_audit_strategy_info::ProductAuditStrategyInfo = serde_json::from_str(r##"{"hide_err_field_flag":null,"hit_duplicated_flag":null,"hit_low_risk_rule_flag":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "hide_err_field_flag",
        "hit_duplicated_flag",
        "hit_low_risk_rule_flag",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_product_audit_strategy_response() {
    let data: wx_rust_store::bean::product::product_audit_strategy_response::ProductAuditStrategyResponse = serde_json::from_str(r##"{"audit_strategy":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "audit_strategy";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_product_audit_strategy_set_param() {
    let data: wx_rust_store::bean::product::product_audit_strategy_set_param::ProductAuditStrategySetParam = serde_json::from_str(r##"{"audit_strategy":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "audit_strategy";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_assistant_product_brand_recommend_param() {
    let data: wx_rust_store::bean::product::assistant::product_brand_recommend_param::ProductBrandRecommendParam = serde_json::from_str(r##"{"cat_id":null,"head_imgs":null,"detail_imgs":null,"title":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["cat_id", "head_imgs", "detail_imgs", "title"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_assistant_product_brand_recommend_response() {
    let data: wx_rust_store::bean::product::assistant::product_brand_recommend_response::ProductBrandRecommendResponse = serde_json::from_str(r##"{"brand_id":null,"brand_name_chinese":null,"brand_name_english":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["brand_id", "brand_name_chinese", "brand_name_english"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_product_category_classify_param() {
    let data: wx_rust_store::bean::product::product_category_classify_param::ProductCategoryClassifyParam = serde_json::from_str(r##"{"req_type":null,"title":null,"head_imgs":null,"cat_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["req_type", "title", "head_imgs", "cat_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_product_category_classify_response() {
    let data: wx_rust_store::bean::product::product_category_classify_response::ProductCategoryClassifyResponse = serde_json::from_str(r##"{"categories":null,"wrong_cat":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["categories", "wrong_cat"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_product_qua_info() {
    let data: wx_rust_store::bean::product::product_qua_info::ProductQuaInfo =
        serde_json::from_str(r##"{"qua_id":null,"qua_url":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["qua_id", "qua_url"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_product_sale_limit_info() {
    let data: wx_rust_store::bean::product::product_sale_limit_info::ProductSaleLimitInfo =
        serde_json::from_str(r##"{"is_limited":null,"title":null,"sub_title":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["is_limited", "title", "sub_title"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_product_scheme_param() {
    let data: wx_rust_store::bean::product::product_scheme_param::ProductSchemeParam =
        serde_json::from_str(
            r##"{"product_id":null,"from_appid":null,"expire":null,"ext_info":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "from_appid", "expire", "ext_info"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_product_scheme_response() {
    let data: wx_rust_store::bean::product::product_scheme_response::ProductSchemeResponse =
        serde_json::from_str(r##"{"openlink":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "openlink";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_sku_deliver_info() {
    let data: wx_rust_store::bean::product::sku_deliver_info::SkuDeliverInfo = serde_json::from_str(r##"{"stock_type":null,"full_payment_presale_delivery_type":null,"presale_begin_time":null,"presale_end_time":null,"full_payment_presale_delivery_time":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "stock_type",
        "full_payment_presale_delivery_type",
        "presale_begin_time",
        "presale_end_time",
        "full_payment_presale_delivery_time",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_sku_fast_info() {
    let data: wx_rust_store::bean::product::sku_fast_info::SkuFastInfo = serde_json::from_str(r##"{"sku_id":null,"sale_price":null,"stock_info":null,"sku_deliver_info":null,"is_delete":null,"sku_code":null,"status":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "sku_id",
        "sale_price",
        "stock_info",
        "sku_deliver_info",
        "is_delete",
        "sku_code",
        "status",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_sku_info() {
    let data: wx_rust_store::bean::product::sku_info::SkuInfo = serde_json::from_str(r##"{"out_product_id":null,"out_sku_id":null,"thumb_img":null,"sale_price":null,"market_price":null,"stock_num":null,"sku_code":null,"sku_attrs":null,"sku_deliver_info":null,"sku_id":null,"bar_code":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "out_product_id",
        "out_sku_id",
        "thumb_img",
        "sale_price",
        "market_price",
        "stock_num",
        "sku_code",
        "sku_attrs",
        "sku_deliver_info",
        "sku_id",
        "bar_code",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_sku_stock_batch_list() {
    let data: wx_rust_store::bean::product::sku_stock_batch_list::SkuStockBatchList =
        serde_json::from_str(r##"{"spu_stock_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "spu_stock_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_sku_stock_batch_param() {
    let data: wx_rust_store::bean::product::sku_stock_batch_param::SkuStockBatchParam =
        serde_json::from_str(r##"{"product_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_sku_stock_batch_response() {
    let data: wx_rust_store::bean::product::sku_stock_batch_response::SkuStockBatchResponse =
        serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_sku_stock_info() {
    let data: wx_rust_store::bean::product::sku_stock_info::SkuStockInfo = serde_json::from_str(r##"{"normal_stock_num":null,"limited_discount_stock_num":null,"warehouse_stocks":null,"total_stock_num":null,"finder_stock_num":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "normal_stock_num",
        "limited_discount_stock_num",
        "warehouse_stocks",
        "total_stock_num",
        "finder_stock_num",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_sku_stock_param() {
    let data: wx_rust_store::bean::product::sku_stock_param::SkuStockParam =
        serde_json::from_str(r##"{"product_id":null,"sku_id":null,"diff_type":null,"num":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "sku_id", "diff_type", "num"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_sku_stock_response() {
    let data: wx_rust_store::bean::product::sku_stock_response::SkuStockResponse =
        serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_spu_category() {
    let data: wx_rust_store::bean::product::spu_category::SpuCategory =
        serde_json::from_str(r##"{"cat_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "cat_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_spu_fast_info() {
    let data: wx_rust_store::bean::product::spu_fast_info::SpuFastInfo = serde_json::from_str(r##"{"product_id":null,"skus":null,"spu_code":null,"limit_info":null,"express_info":null,"extra_service":null,"deliver_method":null,"timing_onsale_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_id",
        "skus",
        "spu_code",
        "limit_info",
        "express_info",
        "extra_service",
        "deliver_method",
        "timing_onsale_info",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_spu_get_response() {
    let data: wx_rust_store::bean::product::spu_get_response::SpuGetResponse =
        serde_json::from_str(r##"{"product":null,"edit_product":null,"sale_limit_info":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product", "edit_product", "sale_limit_info"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_spu_info() {
    let data: wx_rust_store::bean::product::spu_info::SpuInfo = serde_json::from_str(r##"{"product_id":null,"out_product_id":null,"skus":null,"title":null,"sub_title":null,"head_imgs":null,"deliver_method":null,"deliver_acct_type":null,"desc_info":null,"cats":null,"cats_v2":null,"attrs":null,"spu_code":null,"brand_id":null,"qualifications":null,"express_info":null,"aftersale_desc":null,"limited_info":null,"extra_service":null,"status":null,"edit_status":null,"min_price":null,"create_time":null,"edit_time":null,"product_type":null,"after_sale_info":null,"src_product_id":null,"product_qua_infos":null,"size_chart":null,"short_title":null,"total_sold_num":null,"release_mode":null,"timing_onsale_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_id",
        "out_product_id",
        "skus",
        "title",
        "sub_title",
        "head_imgs",
        "deliver_method",
        "deliver_acct_type",
        "desc_info",
        "cats",
        "cats_v2",
        "attrs",
        "spu_code",
        "brand_id",
        "qualifications",
        "express_info",
        "aftersale_desc",
        "limited_info",
        "extra_service",
        "status",
        "edit_status",
        "min_price",
        "create_time",
        "edit_time",
        "product_type",
        "after_sale_info",
        "src_product_id",
        "product_qua_infos",
        "size_chart",
        "short_title",
        "total_sold_num",
        "release_mode",
        "timing_onsale_info",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_spu_list_param() {
    let data: wx_rust_store::bean::product::spu_list_param::SpuListParam =
        serde_json::from_str(r##"{"page_size":null,"next_key":null,"status":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["page_size", "next_key", "status"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_spu_list_response() {
    let data: wx_rust_store::bean::product::spu_list_response::SpuListResponse =
        serde_json::from_str(r##"{"total_num":null,"next_key":null,"product_ids":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["total_num", "next_key", "product_ids"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_spu_simple_info() {
    let data: wx_rust_store::bean::product::spu_simple_info::SpuSimpleInfo =
        serde_json::from_str(r##"{"product_id":null,"out_product_id":null,"skus":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "out_product_id", "skus"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_spu_size_chart() {
    let data: wx_rust_store::bean::product::spu_size_chart::SpuSizeChart =
        serde_json::from_str(r##"{"enable":null,"specification_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["enable", "specification_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_spu_size_chart_item() {
    let data: wx_rust_store::bean::product::spu_size_chart_item::SpuSizeChartItem =
        serde_json::from_str(r##"{"name":null,"unit":null,"is_range":null,"value_list":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["name", "unit", "is_range", "value_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_spu_stock_info() {
    let data: wx_rust_store::bean::product::spu_stock_info::SpuStockInfo =
        serde_json::from_str(r##"{"product_id":null,"sku_stock":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "sku_stock"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_spu_update_info() {
    let data: wx_rust_store::bean::product::spu_update_info::SpuUpdateInfo = serde_json::from_str(r##"{"product_id":null,"out_product_id":null,"skus":null,"title":null,"sub_title":null,"head_imgs":null,"deliver_method":null,"deliver_acct_type":null,"desc_info":null,"cats":null,"cats_v2":null,"attrs":null,"spu_code":null,"brand_id":null,"qualifications":null,"express_info":null,"aftersale_desc":null,"limited_info":null,"extra_service":null,"status":null,"edit_status":null,"min_price":null,"create_time":null,"edit_time":null,"product_type":null,"after_sale_info":null,"src_product_id":null,"product_qua_infos":null,"size_chart":null,"short_title":null,"total_sold_num":null,"release_mode":null,"timing_onsale_info":null,"listing":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_id",
        "out_product_id",
        "skus",
        "title",
        "sub_title",
        "head_imgs",
        "deliver_method",
        "deliver_acct_type",
        "desc_info",
        "cats",
        "cats_v2",
        "attrs",
        "spu_code",
        "brand_id",
        "qualifications",
        "express_info",
        "aftersale_desc",
        "limited_info",
        "extra_service",
        "status",
        "edit_status",
        "min_price",
        "create_time",
        "edit_time",
        "product_type",
        "after_sale_info",
        "src_product_id",
        "product_qua_infos",
        "size_chart",
        "short_title",
        "total_sold_num",
        "release_mode",
        "timing_onsale_info",
        "listing",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_spu_update_response() {
    let data: wx_rust_store::bean::product::spu_update_response::SpuUpdateResponse =
        serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_timing_on_sale_info() {
    let data: wx_rust_store::bean::product::timing_on_sale_info::TimingOnSaleInfo =
        serde_json::from_str(
            r##"{"status":null,"onsale_time":null,"is_hide_price":null,"task_id":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["status", "onsale_time", "is_hide_price", "task_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_warehouse_stock_info() {
    let data: wx_rust_store::bean::product::warehouse_stock_info::WarehouseStockInfo =
        serde_json::from_str(r##"{"out_warehouse_id":null,"num":null,"lock_stock":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["out_warehouse_id", "num", "lock_stock"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_assistant_begin_timing_sale_param() {
    let data: wx_rust_store::bean::product::assistant::begin_timing_sale_param::BeginTimingSaleParam = serde_json::from_str(r##"{"product_id":null,"task_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "task_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_assistant_cancel_timing_sale_param() {
    let data: wx_rust_store::bean::product::assistant::cancel_timing_sale_param::CancelTimingSaleParam = serde_json::from_str(r##"{"product_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_assistant_category_pre_check_param() {
    let data: wx_rust_store::bean::product::assistant::category_pre_check_param::CategoryPreCheckParam = serde_json::from_str(r##"{"cat_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "cat_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_assistant_category_pre_check_response() {
    let data: wx_rust_store::bean::product::assistant::category_pre_check_response::CategoryPreCheckResponse = serde_json::from_str(r##"{"all_pass":null,"fail_reasons":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["all_pass", "fail_reasons"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_link_product_h5_url_response() {
    let data: wx_rust_store::bean::product::link::product_h5_url_response::ProductH5UrlResponse =
        serde_json::from_str(r##"{"product_h5url":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product_h5url";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_link_product_qr_code_response() {
    let data: wx_rust_store::bean::product::link::product_qr_code_response::ProductQrCodeResponse =
        serde_json::from_str(r##"{"product_qrcode":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product_qrcode";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_link_product_tag_link_response() {
    let data: wx_rust_store::bean::product::link::product_tag_link_response::ProductTagLinkResponse = serde_json::from_str(r##"{"product_taglink":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product_taglink";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_stock_stock_flow_ext_info() {
    let data: wx_rust_store::bean::product::stock::stock_flow_ext_info::StockFlowExtInfo = serde_json::from_str(r##"{"unmove_from_stock_sub_type":null,"move_to_stock_sub_type":null,"upload_source":null,"order_id":null,"out_warehouse_id":null,"limited_discount_id":null,"finder_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "unmove_from_stock_sub_type",
        "move_to_stock_sub_type",
        "upload_source",
        "order_id",
        "out_warehouse_id",
        "limited_discount_id",
        "finder_id",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_stock_stock_flow_info() {
    let data: wx_rust_store::bean::product::stock::stock_flow_info::StockFlowInfo = serde_json::from_str(r##"{"amount":null,"beginning_amount":null,"ending_amount":null,"stock_sub_type":null,"op_type":null,"update_time":null,"ext_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "amount",
        "beginning_amount",
        "ending_amount",
        "stock_sub_type",
        "op_type",
        "update_time",
        "ext_info",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_product_stock_stock_flow_param() {
    let data: wx_rust_store::bean::product::stock::stock_flow_param::StockFlowParam = serde_json::from_str(r##"{"product_id":null,"sku_id":null,"stock_type":null,"finder_id":null,"begin_time":null,"end_time":null,"op_type_list":null,"page_size":null,"next_key":null,"stock_type_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_id",
        "sku_id",
        "stock_type",
        "finder_id",
        "begin_time",
        "end_time",
        "op_type_list",
        "page_size",
        "next_key",
        "stock_type_id",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_qic_inspect_code_response() {
    let data: wx_rust_store::bean::qic::inspect_code_response::InspectCodeResponse =
        serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_qic_inspect_config_response() {
    let data: wx_rust_store::bean::qic::inspect_config_response::InspectConfigResponse =
        serde_json::from_str(r##"{"inspect_config":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "inspect_config";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_qic_register_logistics_request() {
    let data: wx_rust_store::bean::qic::register_logistics_request::RegisterLogisticsRequest =
        serde_json::from_str(r##"{"order_id_list":null,"logistics_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id_list", "logistics_info"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_qic_submit_config_response() {
    let data: wx_rust_store::bean::qic::submit_config_response::SubmitConfigResponse =
        serde_json::from_str(r##"{"submit_config":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "submit_config";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_qic_submit_inspect_request() {
    let data: wx_rust_store::bean::qic::submit_inspect_request::SubmitInspectRequest =
        serde_json::from_str(r##"{"order_id":null,"inspect_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "inspect_info"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_sharer_finder_scene_info() {
    let data: wx_rust_store::bean::sharer::finder_scene_info::FinderSceneInfo = serde_json::from_str(r##"{"promoter_id":null,"finder_nickname":null,"live_export_id":null,"video_export_id":null,"video_title":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "promoter_id",
        "finder_nickname",
        "live_export_id",
        "video_export_id",
        "video_title",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_sharer_sharer_bind_response() {
    let data: wx_rust_store::bean::sharer::sharer_bind_response::SharerBindResponse =
        serde_json::from_str(r##"{"qrcode_img_base64":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "qrcode_img_base64";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_sharer_sharer_info() {
    let data: wx_rust_store::bean::sharer::sharer_info::SharerInfo = serde_json::from_str(
        r##"{"openid":null,"unionid":null,"nickname":null,"bind_time":null,"sharer_type":null}"##,
    )
    .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["openid", "unionid", "nickname", "bind_time", "sharer_type"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_sharer_sharer_info_response() {
    let data: wx_rust_store::bean::sharer::sharer_info_response::SharerInfoResponse =
        serde_json::from_str(r##"{"sharer_info_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "sharer_info_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_sharer_sharer_list_param() {
    let data: wx_rust_store::bean::sharer::sharer_list_param::SharerListParam =
        serde_json::from_str(r##"{"page":null,"page_size":null,"sharer_type":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["page", "page_size", "sharer_type"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_sharer_sharer_order() {
    let data: wx_rust_store::bean::sharer::sharer_order::SharerOrder = serde_json::from_str(r##"{"order_id":null,"share_scene":null,"sharer_openid":null,"sharer_type":null,"sku_id":null,"product_id":null,"from_wecom":null,"finder_scene_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "order_id",
        "share_scene",
        "sharer_openid",
        "sharer_type",
        "sku_id",
        "product_id",
        "from_wecom",
        "finder_scene_info",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_sharer_sharer_order_param() {
    let data: wx_rust_store::bean::sharer::sharer_order_param::SharerOrderParam = serde_json::from_str(r##"{"page":null,"page_size":null,"openid":null,"share_scene":null,"start_time":null,"end_time":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "page",
        "page_size",
        "openid",
        "share_scene",
        "start_time",
        "end_time",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_sharer_sharer_order_response() {
    let data: wx_rust_store::bean::sharer::sharer_order_response::SharerOrderResponse =
        serde_json::from_str(r##"{"order_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "order_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_sharer_sharer_search_param() {
    let data: wx_rust_store::bean::sharer::sharer_search_param::SharerSearchParam =
        serde_json::from_str(r##"{"openid":null,"username":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["openid", "username"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_sharer_sharer_search_response() {
    let data: wx_rust_store::bean::sharer::sharer_search_response::SharerSearchResponse = serde_json::from_str(r##"{"openid":null,"unionid":null,"nickname":null,"bind_time":null,"sharer_type":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["openid", "unionid", "nickname", "bind_time", "sharer_type"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_sharer_sharer_unbind_param() {
    let data: wx_rust_store::bean::sharer::sharer_unbind_param::SharerUnbindParam =
        serde_json::from_str(r##"{"openid_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "openid_list";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_sharer_sharer_unbind_response() {
    let data: wx_rust_store::bean::sharer::sharer_unbind_response::SharerUnbindResponse =
        serde_json::from_str(
            r##"{"success_openid":null,"fail_openid":null,"refuse_openid":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["success_openid", "fail_openid", "refuse_openid"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_shop_shop_h5_url_response() {
    let data: wx_rust_store::bean::shop::shop_h5_url_response::ShopH5UrlResponse =
        serde_json::from_str(r##"{"shop_h5url":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "shop_h5url";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_shop_shop_info() {
    let data: wx_rust_store::bean::shop::shop_info::ShopInfo = serde_json::from_str(r##"{"nickname":null,"headimg_url":null,"subject_type":null,"status":null,"username":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "nickname",
        "headimg_url",
        "subject_type",
        "status",
        "username",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_shop_shop_info_response() {
    let data: wx_rust_store::bean::shop::shop_info_response::ShopInfoResponse =
        serde_json::from_str(r##"{"info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "info";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_shop_shop_qr_code_response() {
    let data: wx_rust_store::bean::shop::shop_qr_code_response::ShopQrCodeResponse =
        serde_json::from_str(r##"{"shop_qrcode":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "shop_qrcode";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_shop_shop_tag_link_response() {
    let data: wx_rust_store::bean::shop::shop_tag_link_response::ShopTagLinkResponse =
        serde_json::from_str(r##"{"shop_taglink":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "shop_taglink";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_distribute_type_response() {
    let data: wx_rust_store::bean::supplier::distribute_type_response::DistributeTypeResponse =
        serde_json::from_str(r##"{"distribute_type":null,"supplier_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["distribute_type", "supplier_info"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_dropship_assign_request() {
    let data: wx_rust_store::bean::supplier::dropship_assign_request::DropshipAssignRequest =
        serde_json::from_str(r##"{"order_id":null,"supplier_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "supplier_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_dropship_detail_response() {
    let data: wx_rust_store::bean::supplier::dropship_detail_response::DropshipDetailResponse =
        serde_json::from_str(r##"{"dropship_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "dropship_info";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_dropship_info() {
    let data: wx_rust_store::bean::supplier::dropship_info::DropshipInfo = serde_json::from_str(r##"{"order_id":null,"supplier_id":null,"ds_order_id":null,"status":null,"create_time":null,"update_time":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "order_id",
        "supplier_id",
        "ds_order_id",
        "status",
        "create_time",
        "update_time",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_dropship_list_request() {
    let data: wx_rust_store::bean::supplier::dropship_list_request::DropshipListRequest = serde_json::from_str(r##"{"supplier_id":null,"status":null,"create_time_start":null,"create_time_end":null,"page_size":null,"next_key":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "supplier_id",
        "status",
        "create_time_start",
        "create_time_end",
        "page_size",
        "next_key",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_dropship_list_response() {
    let data: wx_rust_store::bean::supplier::dropship_list_response::DropshipListResponse =
        serde_json::from_str(r##"{"dropship_list":null,"next_key":null,"has_more":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["dropship_list", "next_key", "has_more"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_dropship_response() {
    let data: wx_rust_store::bean::supplier::dropship_response::DropshipResponse =
        serde_json::from_str(r##"{"order_id":null,"supplier_id":null,"dropship_id":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "supplier_id", "dropship_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_dropship_search_request() {
    let data: wx_rust_store::bean::supplier::dropship_search_request::DropshipSearchRequest = serde_json::from_str(r##"{"supplier_id":null,"status":null,"create_time_start":null,"create_time_end":null,"page_size":null,"next_key":null,"order_id":null,"dropship_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "supplier_id",
        "status",
        "create_time_start",
        "create_time_end",
        "page_size",
        "next_key",
        "order_id",
        "dropship_id",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_product_distribute_request() {
    let data: wx_rust_store::bean::supplier::product_distribute_request::ProductDistributeRequest =
        serde_json::from_str(r##"{"supplier_id":null,"product_id_list":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["supplier_id", "product_id_list"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_product_list_response() {
    let data: wx_rust_store::bean::supplier::product_list_response::ProductListResponse =
        serde_json::from_str(r##"{"product_list":null,"next_key":null,"has_more":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_list", "next_key", "has_more"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_supplier_info() {
    let data: wx_rust_store::bean::supplier::supplier_info::SupplierInfo =
        serde_json::from_str(r##"{"supplier_id":null,"supplier_name":null,"status":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["supplier_id", "supplier_name", "status"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_supplier_info_response() {
    let data: wx_rust_store::bean::supplier::supplier_info_response::SupplierInfoResponse =
        serde_json::from_str(r##"{"supplier_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "supplier_info";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_supplier_supplier_list_response() {
    let data: wx_rust_store::bean::supplier::supplier_list_response::SupplierListResponse =
        serde_json::from_str(r##"{"supplier_list":null,"next_key":null,"has_more":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["supplier_list", "next_key", "has_more"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_talent_talent_order_detail_param() {
    let data: wx_rust_store::bean::talent::talent_order_detail_param::TalentOrderDetailParam =
        serde_json::from_str(r##"{"order_id":null,"sku_id":null,"special_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_id", "sku_id", "special_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_talent_talent_order_detail_response() {
    let data: wx_rust_store::bean::talent::talent_order_detail_response::TalentOrderDetailResponse = serde_json::from_str(r##"{"base_info":null,"commission_info":null,"channel_info":null,"promotion_head_supplier_info":null,"product_info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "base_info",
        "commission_info",
        "channel_info",
        "promotion_head_supplier_info",
        "product_info",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_talent_talent_order_list_param() {
    let data: wx_rust_store::bean::talent::talent_order_list_param::TalentOrderListParam = serde_json::from_str(r##"{"create_time_gt":null,"create_time_lt":null,"order_id":null,"spu_id":null,"update_time_gt":null,"update_time_lt":null,"page_size":null,"next_key":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "create_time_gt",
        "create_time_lt",
        "order_id",
        "spu_id",
        "update_time_gt",
        "update_time_lt",
        "page_size",
        "next_key",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_talent_talent_order_list_response() {
    let data: wx_rust_store::bean::talent::talent_order_list_response::TalentOrderListResponse =
        serde_json::from_str(r##"{"order_list":null,"has_more":null,"next_key":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["order_list", "has_more", "next_key"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_talent_talent_window_product_detail_param() {
    let data: wx_rust_store::bean::talent::talent_window_product_detail_param::TalentWindowProductDetailParam = serde_json::from_str(r##"{"product_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product_id";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_talent_talent_window_product_detail_response() {
    let data: wx_rust_store::bean::talent::talent_window_product_detail_response::TalentWindowProductDetailResponse = serde_json::from_str(r##"{"product":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_talent_talent_window_product_list_param() {
    let data: wx_rust_store::bean::talent::talent_window_product_list_param::TalentWindowProductListParam = serde_json::from_str(r##"{"page_size":null,"page_index":null,"last_buffer":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["page_size", "page_index", "last_buffer"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_talent_talent_window_product_list_response() {
    let data: wx_rust_store::bean::talent::talent_window_product_list_response::TalentWindowProductListResponse = serde_json::from_str(r##"{"products":null,"last_buffer":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["products", "last_buffer"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_token_stable_token_param() {
    let data: wx_rust_store::bean::token::stable_token_param::StableTokenParam =
        serde_json::from_str(
            r##"{"grant_type":null,"appid":null,"secret":null,"force_refresh":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["grant_type", "appid", "secret", "force_refresh"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_vip_score_info() {
    let data: wx_rust_store::bean::vip::score_info::ScoreInfo =
        serde_json::from_str(r##"{"score":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "score";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_vip_user_grade_info() {
    let data: wx_rust_store::bean::vip::user_grade_info::UserGradeInfo =
        serde_json::from_str(r##"{"grade":null,"experience_value":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["grade", "experience_value"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_vip_vip_grade_param() {
    let data: wx_rust_store::bean::vip::vip_grade_param::VipGradeParam =
        serde_json::from_str(r##"{"openid":null,"grade":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["openid", "grade"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_vip_vip_info() {
    let data: wx_rust_store::bean::vip::vip_info::VipInfo = serde_json::from_str(
        r##"{"openid":null,"union_id":null,"user_info":null,"user_grade_info":null}"##,
    )
    .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["openid", "union_id", "user_info", "user_grade_info"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_vip_vip_info_param() {
    let data: wx_rust_store::bean::vip::vip_info_param::VipInfoParam =
        serde_json::from_str(r##"{"openid":null,"need_phone_number":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["openid", "need_phone_number"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_vip_vip_info_response() {
    let data: wx_rust_store::bean::vip::vip_info_response::VipInfoResponse =
        serde_json::from_str(r##"{"info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "info";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_vip_vip_list_param() {
    let data: wx_rust_store::bean::vip::vip_list_param::VipListParam =
        serde_json::from_str(r##"{"need_phone_number":null,"page_num":null,"page_size":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["need_phone_number", "page_num", "page_size"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_vip_vip_list_response() {
    let data: wx_rust_store::bean::vip::vip_list_response::VipListResponse =
        serde_json::from_str(r##"{"list":null,"total_num":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["list", "total_num"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_vip_vip_open_id_param() {
    let data: wx_rust_store::bean::vip::vip_open_id_param::VipOpenIdParam =
        serde_json::from_str(r##"{"openid":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "openid";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_vip_vip_score_param() {
    let data: wx_rust_store::bean::vip::vip_score_param::VipScoreParam =
        serde_json::from_str(r##"{"openid":null,"score":null,"remark":null,"request_id":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["openid", "score", "remark", "request_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_vip_vip_score_response() {
    let data: wx_rust_store::bean::vip::vip_score_response::VipScoreResponse =
        serde_json::from_str(r##"{"info":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "info";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_warehouse_location_priority_response() {
    let data: wx_rust_store::bean::warehouse::location_priority_response::LocationPriorityResponse =
        serde_json::from_str(r##"{"priority_sort":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "priority_sort";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_warehouse_priority_location_param() {
    let data: wx_rust_store::bean::warehouse::priority_location_param::PriorityLocationParam = serde_json::from_str(r##"{"address_id1":null,"address_id2":null,"address_id3":null,"address_id4":null,"priority_sort":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "address_id1",
        "address_id2",
        "address_id3",
        "address_id4",
        "priority_sort",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_warehouse_stock_get_param() {
    let data: wx_rust_store::bean::warehouse::stock_get_param::StockGetParam =
        serde_json::from_str(r##"{"product_id":null,"sku_id":null,"out_warehouse_id":null}"##)
            .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "sku_id", "out_warehouse_id"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_warehouse_update_location_param() {
    let data: wx_rust_store::bean::warehouse::update_location_param::UpdateLocationParam =
        serde_json::from_str(r##"{"out_warehouse_id":null,"cover_locations":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["out_warehouse_id", "cover_locations"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_warehouse_warehouse() {
    let data: wx_rust_store::bean::warehouse::warehouse::Warehouse = serde_json::from_str(
        r##"{"out_warehouse_id":null,"name":null,"intro":null,"cover_locations":null}"##,
    )
    .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["out_warehouse_id", "name", "intro", "cover_locations"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_warehouse_warehouse_location() {
    let data: wx_rust_store::bean::warehouse::warehouse_location::WarehouseLocation =
        serde_json::from_str(
            r##"{"address_id1":null,"address_id2":null,"address_id3":null,"address_id4":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["address_id1", "address_id2", "address_id3", "address_id4"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_warehouse_warehouse_location_param() {
    let data: wx_rust_store::bean::warehouse::warehouse_location_param::WarehouseLocationParam =
        serde_json::from_str(
            r##"{"address_id1":null,"address_id2":null,"address_id3":null,"address_id4":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["address_id1", "address_id2", "address_id3", "address_id4"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_warehouse_warehouse_param() {
    let data: wx_rust_store::bean::warehouse::warehouse_param::WarehouseParam =
        serde_json::from_str(
            r##"{"out_warehouse_id":null,"name":null,"intro":null,"cover_locations":null}"##,
        )
        .unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["out_warehouse_id", "name", "intro", "cover_locations"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_warehouse_warehouse_response() {
    let data: wx_rust_store::bean::warehouse::warehouse_response::WarehouseResponse =
        serde_json::from_str(r##"{"data":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "data";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_warehouse_warehouse_stock_param() {
    let data: wx_rust_store::bean::warehouse::warehouse_stock_param::WarehouseStockParam = serde_json::from_str(r##"{"product_id":null,"sku_id":null,"diff_type":null,"num":null,"out_warehouse_id":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in [
        "product_id",
        "sku_id",
        "diff_type",
        "num",
        "out_warehouse_id",
    ] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_window_request_add_window_product_request() {
    let data: wx_rust_store::bean::window::request::add_window_product_request::AddWindowProductRequest = serde_json::from_str(r##"{"product_id":null,"appid":null,"is_hide_for_window":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "appid", "is_hide_for_window"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_window_request_get_window_product_list_request() {
    let data: wx_rust_store::bean::window::request::get_window_product_list_request::GetWindowProductListRequest = serde_json::from_str(r##"{"appid":null,"last_buffer":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["appid", "last_buffer"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_window_request_window_product_request() {
    let data: wx_rust_store::bean::window::request::window_product_request::WindowProductRequest =
        serde_json::from_str(r##"{"product_id":null,"appid":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["product_id", "appid"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_window_response_get_window_product_list_response() {
    let data: wx_rust_store::bean::window::response::get_window_product_list_response::GetWindowProductListResponse = serde_json::from_str(r##"{"products":null,"last_buffer":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    for key in ["products", "last_buffer"] {
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}

#[test]
fn bean_window_response_get_window_product_response() {
    let data: wx_rust_store::bean::window::response::get_window_product_response::GetWindowProductResponse = serde_json::from_str(r##"{"product":null}"##).unwrap();
    let value = serde_json::to_value(data).unwrap();
    {
        let key = "product";
        assert!(value.get(key).is_none(), "{key}: {value}");
    }
}
