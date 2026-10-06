//! 固定上游 230ed0a Java 字段生成的协议 fixture，独立于 Rust serde 声明。
fn assert_subset(expected: &serde_json::Value, actual: &serde_json::Value, path: &str) {
    match expected {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                assert_subset(v, &actual[k], &format!("{path}.{k}"));
            }
        }
        serde_json::Value::Array(items) => {
            assert_eq!(actual.as_array().map(Vec::len), Some(items.len()), "{path}");
            for (i, v) in items.iter().enumerate() {
                assert_subset(v, &actual[i], &format!("{path}[{i}]"));
            }
        }
        _ => assert_eq!(expected, actual, "{path}"),
    }
}
#[test]
fn bean_address_address_add_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"address_detail":{"address_id":"sample","name":"sample","address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5},"landline":"sample","send_addr":true,"recv_addr":true,"default_send":true,"default_recv":true,"create_time":3000000000,"update_time":3000000000,"address_type":{"same_city":17,"pickup":17}}}"##).unwrap();
    let data: wx_rust_store::bean::address::address_add_param::AddressAddParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_address_address_add_param",
    );
}

#[test]
fn bean_address_address_code() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"name":"sample","code":17,"level":17}"##).unwrap();
    let data: wx_rust_store::bean::address::address_code::AddressCode =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_address_address_code",
    );
}

#[test]
fn bean_address_address_code_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","addrs_msg":{"name":"sample","code":17,"level":17},"next_level_addrs":[{"name":"sample","code":17,"level":17}]}"##).unwrap();
    let data: wx_rust_store::bean::address::address_code_response::AddressCodeResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_address_address_code_response",
    );
}

#[test]
fn bean_address_address_detail() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"address_id":"sample","name":"sample","address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5},"landline":"sample","send_addr":true,"recv_addr":true,"default_send":true,"default_recv":true,"create_time":3000000000,"update_time":3000000000,"address_type":{"same_city":17,"pickup":17}}"##).unwrap();
    let data: wx_rust_store::bean::address::address_detail::AddressDetail =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_address_address_detail",
    );
}

#[test]
fn bean_address_address_id_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"address_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::address::address_id_param::AddressIdParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_address_address_id_param",
    );
}

#[test]
fn bean_address_address_id_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","address_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::address::address_id_response::AddressIdResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_address_address_id_response",
    );
}

#[test]
fn bean_address_address_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","address_detail":{"address_id":"sample","name":"sample","address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5},"landline":"sample","send_addr":true,"recv_addr":true,"default_send":true,"default_recv":true,"create_time":3000000000,"update_time":3000000000,"address_type":{"same_city":17,"pickup":17}}}"##).unwrap();
    let data: wx_rust_store::bean::address::address_info_response::AddressInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_address_address_info_response",
    );
}

#[test]
fn bean_address_address_list_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"offset":17,"limit":17}"##).unwrap();
    let data: wx_rust_store::bean::address::address_list_param::AddressListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_address_address_list_param",
    );
}

#[test]
fn bean_address_address_list_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","address_id_list":["sample"]}"##)
            .unwrap();
    let data: wx_rust_store::bean::address::address_list_response::AddressListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_address_address_list_response",
    );
}

#[test]
fn bean_address_offline_address_type() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"same_city":17,"pickup":17}"##).unwrap();
    let data: wx_rust_store::bean::address::offline_address_type::OfflineAddressType =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_address_offline_address_type",
    );
}

#[test]
fn bean_after_after_sale_accept_exchange_reship_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"after_sale_order_id":"sample","waybill_id":"sample","delivery_id":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::after::after_sale_accept_exchange_reship_param::AfterSaleAcceptExchangeReshipParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_accept_exchange_reship_param",
    );
}

#[test]
fn bean_after_after_sale_accept_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"after_sale_order_id":"sample","address_id":"sample","accept_type":17}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::after::after_sale_accept_param::AfterSaleAcceptParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_accept_param",
    );
}

#[test]
fn bean_after_after_sale_detail() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"desc":"sample","receive_product":true,"cancel_time":3000000000,"prove_imgs":["sample"],"tel_number":"sample","media_id_list":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_detail::AfterSaleDetail =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_detail",
    );
}

#[test]
fn bean_after_after_sale_exchange_delivery_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"waybill_id":"sample","delivery_id":"sample","delivery_name":"sample","address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_exchange_delivery_info::AfterSaleExchangeDeliveryInfo = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_exchange_delivery_info",
    );
}

#[test]
fn bean_after_after_sale_exchange_product_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","old_sku_id":"sample","new_sku_id":"sample","product_cnt":"sample","old_sku_price":17,"new_sku_price":17}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_exchange_product_info::AfterSaleExchangeProductInfo = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_exchange_product_info",
    );
}

#[test]
fn bean_after_after_sale_id_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"after_sale_order_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_id_param::AfterSaleIdParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_id_param",
    );
}

#[test]
fn bean_after_after_sale_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"after_sale_order_id":"sample","status":"sample","order_id":"sample","openid":"sample","unionid":"sample","product_info":{"product_id":"sample","sku_id":"sample","count":17},"details":{"desc":"sample","receive_product":true,"cancel_time":3000000000,"prove_imgs":["sample"],"tel_number":"sample","media_id_list":["sample"]},"refund_info":{"amount":17,"refund_reason":17},"return_info":{"waybill_id":"sample","delivery_id":"sample","delivery_name":"sample"},"merchant_upload_info":{"reject_reason":"sample","refund_certificates":["sample"]},"create_time":3000000000,"update_time":3000000000,"reason":"sample","reason_text":"sample","refund_resp":{"code":"sample","ret":17,"message":"sample"},"type":"sample","complaint_id":"sample","deadline":3000000000,"exchange_product_info":{"product_id":"sample","old_sku_id":"sample","new_sku_id":"sample","product_cnt":"sample","old_sku_price":17,"new_sku_price":17},"exchange_delivery_info":{"waybill_id":"sample","delivery_id":"sample","delivery_name":"sample","address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}},"virtual_tel_num_info":{"virtual_tel_number":"sample","virtual_tel_expire_time":3000000000}}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_info::AfterSaleInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_info",
    );
}

#[test]
fn bean_after_after_sale_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","after_sale_order":{"after_sale_order_id":"sample","status":"sample","order_id":"sample","openid":"sample","unionid":"sample","product_info":{"product_id":"sample","sku_id":"sample","count":17},"details":{"desc":"sample","receive_product":true,"cancel_time":3000000000,"prove_imgs":["sample"],"tel_number":"sample","media_id_list":["sample"]},"refund_info":{"amount":17,"refund_reason":17},"return_info":{"waybill_id":"sample","delivery_id":"sample","delivery_name":"sample"},"merchant_upload_info":{"reject_reason":"sample","refund_certificates":["sample"]},"create_time":3000000000,"update_time":3000000000,"reason":"sample","reason_text":"sample","refund_resp":{"code":"sample","ret":17,"message":"sample"},"type":"sample","complaint_id":"sample","deadline":3000000000,"exchange_product_info":{"product_id":"sample","old_sku_id":"sample","new_sku_id":"sample","product_cnt":"sample","old_sku_price":17,"new_sku_price":17},"exchange_delivery_info":{"waybill_id":"sample","delivery_id":"sample","delivery_name":"sample","address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}},"virtual_tel_num_info":{"virtual_tel_number":"sample","virtual_tel_expire_time":3000000000}}}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_info_response::AfterSaleInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_info_response",
    );
}

#[test]
fn bean_after_after_sale_list_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"begin_create_time":3000000000,"end_create_time":3000000000,"begin_update_time":3000000000,"end_update_time":3000000000,"next_key":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_list_param::AfterSaleListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_list_param",
    );
}

#[test]
fn bean_after_after_sale_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","after_sale_order_id_list":["sample"],"next_key":"sample","has_more":true}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_list_response::AfterSaleListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_list_response",
    );
}

#[test]
fn bean_after_after_sale_merchant_update_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"after_sale_order_id":"sample","type":17,"amount":17,"merchant_update_desc":"sample","update_reason_type":17,"merchant_update_type":17,"media_ids":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_merchant_update_param::AfterSaleMerchantUpdateParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_merchant_update_param",
    );
}

#[test]
fn bean_after_after_sale_product_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"product_id":"sample","sku_id":"sample","count":17}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_product_info::AfterSaleProductInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_product_info",
    );
}

#[test]
fn bean_after_after_sale_reason() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"reason":17,"reason_text":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_reason::AfterSaleReason =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_reason",
    );
}

#[test]
fn bean_after_after_sale_reason_response() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"errcode":0,"errmsg":"sample","reason_list":[{"reason":17,"reason_text":"sample"}]}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::after::after_sale_reason_response::AfterSaleReasonResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_reason_response",
    );
}

#[test]
fn bean_after_after_sale_reject_exchange_reship_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"after_sale_order_id":"sample","reject_reason":"sample","reject_reason_type":17,"reject_certificates":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_reject_exchange_reship_param::AfterSaleRejectExchangeReshipParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_reject_exchange_reship_param",
    );
}

#[test]
fn bean_after_after_sale_reject_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"after_sale_order_id":"sample","reject_reason":"sample","reject_reason_type":17,"reject_certificates":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_reject_param::AfterSaleRejectParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_reject_param",
    );
}

#[test]
fn bean_after_after_sale_reject_reason() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"reject_reason_type":17,"reject_reason_type_text":"sample","reject_reason":"sample","reject_scene":17}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_reject_reason::AfterSaleRejectReason =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_reject_reason",
    );
}

#[test]
fn bean_after_after_sale_reject_reason_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","reject_reason_list":[{"reject_reason_type":17,"reject_reason_type_text":"sample","reject_reason":"sample","reject_scene":17}]}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_reject_reason_response::AfterSaleRejectReasonResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_reject_reason_response",
    );
}

#[test]
fn bean_after_after_sale_return_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"aftersale_id":3000000000,"out_aftersale_id":"sample","address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}}"##).unwrap();
    let data: wx_rust_store::bean::after::after_sale_return_param::AfterSaleReturnParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_return_param",
    );
}

#[test]
fn bean_after_after_sale_virtual_number_info() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"virtual_tel_number":"sample","virtual_tel_expire_time":3000000000}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::after::after_sale_virtual_number_info::AfterSaleVirtualNumberInfo = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_after_sale_virtual_number_info",
    );
}

#[test]
fn bean_after_guarantee_modify_request() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"guarantee_order_id":"sample","bad_level":17,"merchant_remark":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::after::guarantee_modify_request::GuaranteeModifyRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_guarantee_modify_request",
    );
}

#[test]
fn bean_after_guarantee_order_id_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"guarantee_order_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::after::guarantee_order_id_param::GuaranteeOrderIdParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_guarantee_order_id_param",
    );
}

#[test]
fn bean_after_guarantee_order_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","guarantee_order":{"guarantee_order_id":"sample","status":"sample","product_info":{"product_id":"sample"}}}"##).unwrap();
    let data: wx_rust_store::bean::after::guarantee_order_info_response::GuaranteeOrderInfoResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_guarantee_order_info_response",
    );
}

#[test]
fn bean_after_guarantee_order_list_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"guarantee_order_id_list":["sample"],"order_id_list":["sample"],"type":17,"begin_time":3000000000,"end_time":3000000000,"status_list":"sample","offset":17,"limit":17}"##).unwrap();
    let data: wx_rust_store::bean::after::guarantee_order_list_param::GuaranteeOrderListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_guarantee_order_list_param",
    );
}

#[test]
fn bean_after_guarantee_order_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","total_num":17,"guarantee_order_list":[{"guarantee_order_id":"sample","status":"sample","product_info":[{"product_id":"sample"}]}]}"##).unwrap();
    let data: wx_rust_store::bean::after::guarantee_order_list_response::GuaranteeOrderListResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_guarantee_order_list_response",
    );
}

#[test]
fn bean_after_guarantee_proof_request() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"guarantee_order_id":"sample","content":"sample","pic_list":["sample"]}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::after::guarantee_proof_request::GuaranteeProofRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_guarantee_proof_request",
    );
}

#[test]
fn bean_after_guarantee_refuse_request() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"guarantee_order_id":"sample","reason":"sample","pic_list":["sample"]}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::after::guarantee_refuse_request::GuaranteeRefuseRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_guarantee_refuse_request",
    );
}

#[test]
fn bean_after_merchant_upload_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"reject_reason":"sample","refund_certificates":["sample"]}"##)
            .unwrap();
    let data: wx_rust_store::bean::after::merchant_upload_info::MerchantUploadInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_merchant_upload_info",
    );
}

#[test]
fn bean_after_refund_evidence_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"after_sale_order_id":"sample","desc":"sample","refund_certificates":["sample"]}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::after::refund_evidence_param::RefundEvidenceParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_refund_evidence_param",
    );
}

#[test]
fn bean_after_refund_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"amount":17,"refund_reason":17}"##).unwrap();
    let data: wx_rust_store::bean::after::refund_info::RefundInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_refund_info",
    );
}

#[test]
fn bean_after_refund_resp() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"code":"sample","ret":17,"message":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::after::refund_resp::RefundResp =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_refund_resp",
    );
}

#[test]
fn bean_after_return_info() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"waybill_id":"sample","delivery_id":"sample","delivery_name":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::after::return_info::ReturnInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_after_return_info",
    );
}

#[test]
fn bean_audit_audit_apply_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","audit_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::audit::audit_apply_response::AuditApplyResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_audit_audit_apply_response",
    );
}

#[test]
fn bean_audit_audit_response() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"errcode":0,"errmsg":"sample","data":{"status":17,"reject_reason":"sample"}}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::audit::audit_response::AuditResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_audit_audit_response",
    );
}

#[test]
fn bean_audit_audit_result() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"status":17,"reject_reason":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::audit::audit_result::AuditResult =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_audit_audit_result",
    );
}

#[test]
fn bean_audit_category_audit_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"level1":3000000000,"level2":3000000000,"level3":3000000000,"cats_v2":[{"cat_id":"sample"}],"certificate":["sample"],"baobeihan":["sample"],"jingyingzhengming":["sample"],"daihuokoubei":["sample"],"ruzhuzhizhi":["sample"],"jingyingliushui":["sample"],"buchongcailiao":["sample"],"jingyingpingtai":"sample","zhanghaomingcheng":"sample","brand_list":[{"brand_id":"sample"}]}"##).unwrap();
    let data: wx_rust_store::bean::audit::category_audit_info::CategoryAuditInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_audit_category_audit_info",
    );
}

#[test]
fn bean_audit_category_audit_request() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"category_info":{"level1":3000000000,"level2":3000000000,"level3":3000000000,"cats_v2":[{"cat_id":"sample"}],"certificate":["sample"],"baobeihan":["sample"],"jingyingzhengming":["sample"],"daihuokoubei":["sample"],"ruzhuzhizhi":["sample"],"jingyingliushui":["sample"],"buchongcailiao":["sample"],"jingyingpingtai":"sample","zhanghaomingcheng":"sample","brand_list":[{"brand_id":"sample"}]}}"##).unwrap();
    let data: wx_rust_store::bean::audit::category_audit_request::CategoryAuditRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_audit_category_audit_request",
    );
}

#[test]
fn bean_audit_category_brand() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"brand_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::audit::category_brand::CategoryBrand =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_audit_category_brand",
    );
}

#[test]
fn bean_audit_cats_v2() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"cat_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::audit::cats_v2::CatsV2 =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_audit_cats_v2",
    );
}

#[test]
fn bean_audit_product_audit_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"audit_id":"sample","submit_time":"sample","audit_time":"sample","reject_reason":"sample","func_type":17}"##).unwrap();
    let data: wx_rust_store::bean::audit::product_audit_info::ProductAuditInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_audit_product_audit_info",
    );
}

#[test]
fn bean_base_address_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}"##).unwrap();
    let data: wx_rust_store::bean::base::address_info::AddressInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_base_address_info",
    );
}

#[test]
fn bean_base_attr_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"attr_key":"sample","attr_value":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::base::attr_info::AttrInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_base_attr_info",
    );
}

#[test]
fn bean_base_offset_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"offset":17,"limit":17}"##).unwrap();
    let data: wx_rust_store::bean::base::offset_param::OffsetParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_base_offset_param",
    );
}

#[test]
fn bean_base_page_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"page":17,"page_size":17}"##).unwrap();
    let data: wx_rust_store::bean::base::page_param::PageParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_base_page_param",
    );
}

#[test]
fn bean_base_stream_page_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"page_size":17,"next_key":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::base::stream_page_param::StreamPageParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_base_stream_page_param",
    );
}

#[test]
fn bean_base_time_range() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"start_time":3000000000,"end_time":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::base::time_range::TimeRange =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_base_time_range",
    );
}

#[test]
fn bean_base_wx_store_base_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::base::wx_store_base_response::WxStoreBaseResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_base_wx_store_base_response",
    );
}

#[test]
fn bean_brand_basic_brand() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"brand_id":"sample","ch_name":"sample","en_name":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::brand::basic_brand::BasicBrand =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_brand_basic_brand",
    );
}

#[test]
fn bean_brand_brand() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"brand_id":"sample","ch_name":"sample","en_name":"sample","classification_no":"sample","trade_mark_symbol":17,"register_details":{"registrant":"sample","register_no":"sample","start_time":3000000000,"end_time":3000000000,"is_permanent":true,"register_certifications":["sample"],"renew_certifications":["sample"]},"application_details":{"acceptance_time":3000000000,"acceptance_certification":["sample"],"acceptance_no":"sample"},"grant_type":17,"grant_details":{"grant_certifications":["sample"],"grant_level":17,"start_time":3000000000,"end_time":3000000000,"is_permanent":true,"brand_owner_id_photos":["sample"]}}"##).unwrap();
    let data: wx_rust_store::bean::brand::brand::Brand =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_brand_brand",
    );
}

#[test]
fn bean_brand_brand_application_detail() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"acceptance_time":3000000000,"acceptance_certification":["sample"],"acceptance_no":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::brand::brand_application_detail::BrandApplicationDetail =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_brand_brand_application_detail",
    );
}

#[test]
fn bean_brand_brand_apply_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","brands":[{"brand_id":"sample","ch_name":"sample","en_name":"sample"}],"next_key":"sample","total_num":17}"##).unwrap();
    let data: wx_rust_store::bean::brand::brand_apply_list_response::BrandApplyListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_brand_brand_apply_list_response",
    );
}

#[test]
fn bean_brand_brand_grant_detail() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"grant_certifications":["sample"],"grant_level":17,"start_time":3000000000,"end_time":3000000000,"is_permanent":true,"brand_owner_id_photos":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::brand::brand_grant_detail::BrandGrantDetail =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_brand_brand_grant_detail",
    );
}

#[test]
fn bean_brand_brand_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"brand_id":"sample","ch_name":"sample","en_name":"sample","classification_no":"sample","trade_mark_symbol":17,"register_details":{"registrant":"sample","register_no":"sample","start_time":3000000000,"end_time":3000000000,"is_permanent":true,"register_certifications":["sample"],"renew_certifications":["sample"]},"application_details":{"acceptance_time":3000000000,"acceptance_certification":["sample"],"acceptance_no":"sample"},"grant_type":17,"grant_details":{"grant_certifications":["sample"],"grant_level":17,"start_time":3000000000,"end_time":3000000000,"is_permanent":true,"brand_owner_id_photos":["sample"]},"status":17,"create_time":3000000000,"update_time":3000000000,"audit_result":{"audit_id":"sample","reject_reason":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::brand::brand_info::BrandInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_brand_brand_info",
    );
}

#[test]
fn bean_brand_brand_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","brand":{"brand_id":"sample","ch_name":"sample","en_name":"sample","classification_no":"sample","trade_mark_symbol":17,"register_details":{"registrant":"sample","register_no":"sample","start_time":3000000000,"end_time":3000000000,"is_permanent":true,"register_certifications":["sample"],"renew_certifications":["sample"]},"application_details":{"acceptance_time":3000000000,"acceptance_certification":["sample"],"acceptance_no":"sample"},"grant_type":17,"grant_details":{"grant_certifications":["sample"],"grant_level":17,"start_time":3000000000,"end_time":3000000000,"is_permanent":true,"brand_owner_id_photos":["sample"]},"status":17,"create_time":3000000000,"update_time":3000000000,"audit_result":{"audit_id":"sample","reject_reason":"sample"}}}"##).unwrap();
    let data: wx_rust_store::bean::brand::brand_info_response::BrandInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_brand_brand_info_response",
    );
}

#[test]
fn bean_brand_brand_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","brands":[{"brand_id":"sample","ch_name":"sample","en_name":"sample"}],"next_key":"sample","continue_flag":true}"##).unwrap();
    let data: wx_rust_store::bean::brand::brand_list_response::BrandListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_brand_brand_list_response",
    );
}

#[test]
fn bean_brand_brand_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"brand":{"brand_id":"sample","ch_name":"sample","en_name":"sample","classification_no":"sample","trade_mark_symbol":17,"register_details":{"registrant":"sample","register_no":"sample","start_time":3000000000,"end_time":3000000000,"is_permanent":true,"register_certifications":["sample"],"renew_certifications":["sample"]},"application_details":{"acceptance_time":3000000000,"acceptance_certification":["sample"],"acceptance_no":"sample"},"grant_type":17,"grant_details":{"grant_certifications":["sample"],"grant_level":17,"start_time":3000000000,"end_time":3000000000,"is_permanent":true,"brand_owner_id_photos":["sample"]}}}"##).unwrap();
    let data: wx_rust_store::bean::brand::brand_param::BrandParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_brand_brand_param",
    );
}

#[test]
fn bean_brand_brand_register_detail() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"registrant":"sample","register_no":"sample","start_time":3000000000,"end_time":3000000000,"is_permanent":true,"register_certifications":["sample"],"renew_certifications":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::brand::brand_register_detail::BrandRegisterDetail =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_brand_brand_register_detail",
    );
}

#[test]
fn bean_brand_brand_search_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"page_size":17,"next_key":"sample","status":17}"##).unwrap();
    let data: wx_rust_store::bean::brand::brand_search_param::BrandSearchParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_brand_brand_search_param",
    );
}

#[test]
fn bean_category_account_category_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":[{"cat_id":"sample","f_cat_id":"sample","name":"sample","level":17,"leaf":true}]}"##).unwrap();
    let data: wx_rust_store::bean::category::account_category_response::AccountCategoryResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_account_category_response",
    );
}

#[test]
fn bean_category_category_and_qualification_list() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"cat_and_qua":[{"cat":{"cat_id":"sample","f_cat_id":"sample","name":"sample","level":17,"leaf":true},"qua":{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"},"product_qua":{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"},"brand_qua":{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"},"product_qua_list":[{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"}],"is_confidence_require_bad_must_pay":true}]}"##).unwrap();
    let data: wx_rust_store::bean::category::category_and_qualification_list::CategoryAndQualificationList = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_category_and_qualification_list",
    );
}

#[test]
fn bean_category_category_detail_result() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","info":{"cat_id":"sample","name":"sample"},"attr":{"shop_no_shipment":true,"access_permit_required":true,"pre_sale":true,"seven_day_return":true,"brand_list":[{"brand_id":"sample"}],"deposit":3000000000,"product_attr_list":[{"name":"sample","type":"sample","type_v2":"sample","value":"sample","is_required":true,"hint":"sample","append_allowed":true}],"sale_attr_list":[{"name":"sample","type":"sample","type_v2":"sample","value":"sample","is_required":true,"hint":"sample","append_allowed":true}],"transactionfee_info":{"basis_point":17,"original_basis_point":17,"incentive_type":17},"coupon_rule":{"discount_ratio_limit":17,"discount_limit":17},"floor_price":3000000000,"confirm_receipt_days":["sample"],"is_limit_brand":true,"product_requirement":{"product_title_requirement":"sample","product_img_requirement":"sample","product_desc_requirement":"sample"},"size_chart":{"is_support":true,"item_list":[{"name":"sample","unit":"sample","type":"sample","format":"sample","limit":"sample","is_required":true}]},"is_confidence_require_bad_must_pay":true,"product_qua_list":[{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"}]},"product_qua_list":[{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"}]}"##).unwrap();
    let data: wx_rust_store::bean::category::category_detail_result::CategoryDetailResult =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_category_detail_result",
    );
}

#[test]
fn bean_category_category_qualification() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"cat":{"cat_id":"sample","f_cat_id":"sample","name":"sample","level":17,"leaf":true},"qua":{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"},"product_qua":{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"},"brand_qua":{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"},"product_qua_list":[{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"}],"is_confidence_require_bad_must_pay":true}"##).unwrap();
    let data: wx_rust_store::bean::category::category_qualification::CategoryQualification =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_category_qualification",
    );
}

#[test]
fn bean_category_category_qualification_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","cats":[{"cat_and_qua":[{"cat":{"cat_id":"sample","f_cat_id":"sample","name":"sample","level":17,"leaf":true},"qua":{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"},"product_qua":{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"},"brand_qua":{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"},"product_qua_list":[{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"}],"is_confidence_require_bad_must_pay":true}]}],"cats_v2":[{"cat_and_qua":[{"cat":{"cat_id":"sample","f_cat_id":"sample","name":"sample","level":17,"leaf":true},"qua":{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"},"product_qua":{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"},"brand_qua":{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"},"product_qua_list":[{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"}],"is_confidence_require_bad_must_pay":true}]}]}"##).unwrap();
    let data: wx_rust_store::bean::category::category_qualification_response::CategoryQualificationResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_category_qualification_response",
    );
}

#[test]
fn bean_category_pass_category_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"cat_id":"sample","qua_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::category::pass_category_info::PassCategoryInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_pass_category_info",
    );
}

#[test]
fn bean_category_pass_category_response() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"errcode":0,"errmsg":"sample","list":[{"cat_id":"sample","qua_id":"sample"}]}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::category::pass_category_response::PassCategoryResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_pass_category_response",
    );
}

#[test]
fn bean_category_qualification_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"qua_id":"sample","need_to_apply":true,"tips":"sample","mandatory":true,"name":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::category::qualification_info::QualificationInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_qualification_info",
    );
}

#[test]
fn bean_category_relation_category_item() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"id":3000000000,"status":17,"uneffective_reason":"sample","effective_time":3000000000,"uneffective_time":3000000000,"qua_id":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::category::relation_category_item::RelationCategoryItem =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_relation_category_item",
    );
}

#[test]
fn bean_category_relation_category_request() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"is_filter_status":true,"status":17}"##).unwrap();
    let data: wx_rust_store::bean::category::relation_category_request::RelationCategoryRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_relation_category_request",
    );
}

#[test]
fn bean_category_relation_category_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","list":[{"id":3000000000,"status":17,"uneffective_reason":"sample","effective_time":3000000000,"uneffective_time":3000000000,"qua_id":3000000000}]}"##).unwrap();
    let data: wx_rust_store::bean::category::relation_category_response::RelationCategoryResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_relation_category_response",
    );
}

#[test]
fn bean_category_shop_category() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"cat_id":"sample","f_cat_id":"sample","name":"sample","level":17,"leaf":true}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::category::shop_category::ShopCategory =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_shop_category",
    );
}

#[test]
fn bean_category_shop_category_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","cat_list":[{"cat_id":"sample","f_cat_id":"sample","name":"sample","level":17,"leaf":true}],"cat_list_v2":[{"cat_id":"sample","f_cat_id":"sample","name":"sample","level":17,"leaf":true}]}"##).unwrap();
    let data: wx_rust_store::bean::category::shop_category_response::ShopCategoryResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_category_shop_category_response",
    );
}

#[test]
fn bean_compass_compass_finder_base_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"ds":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::compass::compass_finder_base_param::CompassFinderBaseParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_compass_finder_base_param",
    );
}

#[test]
fn bean_compass_shop_compass_finder_id_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"ds":"sample","finder_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::compass_finder_id_param::CompassFinderIdParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_compass_finder_id_param",
    );
}

#[test]
fn bean_compass_shop_finder_auth_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","main_finder_id":"sample","authorized_finder_id_list":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::finder_auth_list_response::FinderAuthListResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_finder_auth_list_response",
    );
}

#[test]
fn bean_compass_shop_finder_gmv_data() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"pay_gmv":"sample","pay_product_id_cnt":"sample","pay_uv":"sample","refund_gmv":"sample","pay_refund_gmv":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::finder_gmv_data::FinderGmvData =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_finder_gmv_data",
    );
}

#[test]
fn bean_compass_shop_finder_gmv_item() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"finder_id":"sample","finder_nickname":"sample","data":{"pay_gmv":"sample","pay_product_id_cnt":"sample","pay_uv":"sample","refund_gmv":"sample","pay_refund_gmv":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::finder_gmv_item::FinderGmvItem =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_finder_gmv_item",
    );
}

#[test]
fn bean_compass_shop_finder_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","finder_list":[{"finder_id":"sample","finder_nickname":"sample","data":{"pay_gmv":"sample","pay_product_id_cnt":"sample","pay_uv":"sample","refund_gmv":"sample","pay_refund_gmv":"sample"}}]}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::finder_list_response::FinderListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_finder_list_response",
    );
}

#[test]
fn bean_compass_shop_finder_overall_data() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"pay_gmv":"sample","pay_sales_finder_cnt":"sample","pay_product_id_cnt":"sample","click_to_pay_uv_ratio":1.5}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::finder_overall_data::FinderOverallData =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_finder_overall_data",
    );
}

#[test]
fn bean_compass_shop_finder_overall_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"pay_gmv":"sample","pay_sales_finder_cnt":"sample","pay_product_id_cnt":"sample","click_to_pay_uv_ratio":1.5}}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::finder_overall_response::FinderOverallResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_finder_overall_response",
    );
}

#[test]
fn bean_compass_shop_finder_product_list_item() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","head_img_url":"sample","title":"sample","price":"sample","first_category_id":"sample","second_category_id":"sample","third_category_id":"sample","data":{"commission_ratio":1.5,"pay_gmv":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::finder_product_list_item::FinderProductListItem =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_finder_product_list_item",
    );
}

#[test]
fn bean_compass_shop_finder_product_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","product_list":[{"product_id":"sample","head_img_url":"sample","title":"sample","price":"sample","first_category_id":"sample","second_category_id":"sample","third_category_id":"sample","data":{"commission_ratio":1.5,"pay_gmv":"sample"}}]}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::finder_product_list_response::FinderProductListResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_finder_product_list_response",
    );
}

#[test]
fn bean_compass_shop_finder_product_overall_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"pay_gmv":"sample","pay_product_id_cnt":"sample","pay_uv":"sample","refund_gmv":"sample","pay_refund_gmv":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::finder_product_overall_response::FinderProductOverallResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_finder_product_overall_response",
    );
}

#[test]
fn bean_compass_shop_finder_product_simple_gmv_data() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"commission_ratio":1.5,"pay_gmv":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::finder_product_simple_gmv_data::FinderProductSimpleGmvData = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_finder_product_simple_gmv_data",
    );
}

#[test]
fn bean_compass_shop_shop_field() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"field_name":"sample","data_list":[{"dim_key":"sample","dim_value":"sample"}]}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_field::ShopField =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_field",
    );
}

#[test]
fn bean_compass_shop_shop_live_data() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"live_id":"sample","live_title":"sample","live_time":"sample","live_duration":"sample","live_cover_img_url":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_live_data::ShopLiveData =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_live_data",
    );
}

#[test]
fn bean_compass_shop_shop_live_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","live_list":[{"live_id":"sample","live_title":"sample","live_time":"sample","live_duration":"sample","live_cover_img_url":"sample"}]}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_live_list_response::ShopLiveListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_live_list_response",
    );
}

#[test]
fn bean_compass_shop_shop_overall() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"pay_gmv":"sample","pay_uv":"sample","pay_refund_gmv":"sample","pay_order_cnt":"sample","live_pay_gmv":"sample","feed_pay_gmv":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_overall::ShopOverall =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_overall",
    );
}

#[test]
fn bean_compass_shop_shop_overall_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"pay_gmv":"sample","pay_uv":"sample","pay_refund_gmv":"sample","pay_order_cnt":"sample","live_pay_gmv":"sample","feed_pay_gmv":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_overall_response::ShopOverallResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_overall_response",
    );
}

#[test]
fn bean_compass_shop_shop_product_compass_data() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"pay_gmv":"sample","create_gmv":"sample","create_cnt":"sample","create_uv":"sample","create_product_cnt":"sample","pay_cnt":"sample","pay_uv":"sample","pay_product_cnt":"sample","pure_pay_gmv":"sample","pay_gmv_per_uv":"sample","seller_actual_settle_amount":"sample","platform_actual_commission":"sample","finderuin_actual_commission":"sample","captain_actual_commission":"sample","seller_predict_settle_amount":"sample","platform_predict_commission":"sample","finderuin_predict_commission":"sample","captain_predict_commission":"sample","product_click_uv":"sample","product_click_cnt":"sample","pay_refund_gmv":"sample","pay_refund_uv":"sample","pay_refund_ratio":1.5,"pay_refund_after_send_ratio":1.5,"pay_refund_cnt":"sample","pay_refund_product_cnt":"sample","pay_refund_before_send_ratio":1.5,"refund_gmv":"sample","refund_product_cnt":"sample","refund_cnt":"sample","refund_uv":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_product_compass_data::ShopProductCompassData = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_product_compass_data",
    );
}

#[test]
fn bean_compass_shop_shop_product_data_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"ds":"sample","product_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_product_data_param::ShopProductDataParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_product_data_param",
    );
}

#[test]
fn bean_compass_shop_shop_product_data_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","product_info":{"product_id":"sample","head_img_url":"sample","title":"sample","price":"sample","first_category_id":"sample","second_category_id":"sample","third_category_id":"sample","data":{"pay_gmv":"sample","create_gmv":"sample","create_cnt":"sample","create_uv":"sample","create_product_cnt":"sample","pay_cnt":"sample","pay_uv":"sample","pay_product_cnt":"sample","pure_pay_gmv":"sample","pay_gmv_per_uv":"sample","seller_actual_settle_amount":"sample","platform_actual_commission":"sample","finderuin_actual_commission":"sample","captain_actual_commission":"sample","seller_predict_settle_amount":"sample","platform_predict_commission":"sample","finderuin_predict_commission":"sample","captain_predict_commission":"sample","product_click_uv":"sample","product_click_cnt":"sample","pay_refund_gmv":"sample","pay_refund_uv":"sample","pay_refund_ratio":1.5,"pay_refund_after_send_ratio":1.5,"pay_refund_cnt":"sample","pay_refund_product_cnt":"sample","pay_refund_before_send_ratio":1.5,"refund_gmv":"sample","refund_product_cnt":"sample","refund_cnt":"sample","refund_uv":"sample"}}}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_product_data_response::ShopProductDataResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_product_data_response",
    );
}

#[test]
fn bean_compass_shop_shop_product_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","head_img_url":"sample","title":"sample","price":"sample","first_category_id":"sample","second_category_id":"sample","third_category_id":"sample","data":{"pay_gmv":"sample","create_gmv":"sample","create_cnt":"sample","create_uv":"sample","create_product_cnt":"sample","pay_cnt":"sample","pay_uv":"sample","pay_product_cnt":"sample","pure_pay_gmv":"sample","pay_gmv_per_uv":"sample","seller_actual_settle_amount":"sample","platform_actual_commission":"sample","finderuin_actual_commission":"sample","captain_actual_commission":"sample","seller_predict_settle_amount":"sample","platform_predict_commission":"sample","finderuin_predict_commission":"sample","captain_predict_commission":"sample","product_click_uv":"sample","product_click_cnt":"sample","pay_refund_gmv":"sample","pay_refund_uv":"sample","pay_refund_ratio":1.5,"pay_refund_after_send_ratio":1.5,"pay_refund_cnt":"sample","pay_refund_product_cnt":"sample","pay_refund_before_send_ratio":1.5,"refund_gmv":"sample","refund_product_cnt":"sample","refund_cnt":"sample","refund_uv":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_product_info::ShopProductInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_product_info",
    );
}

#[test]
fn bean_compass_shop_shop_product_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","product_list":[{"product_id":"sample","head_img_url":"sample","title":"sample","price":"sample","first_category_id":"sample","second_category_id":"sample","third_category_id":"sample","data":{"pay_gmv":"sample","create_gmv":"sample","create_cnt":"sample","create_uv":"sample","create_product_cnt":"sample","pay_cnt":"sample","pay_uv":"sample","pay_product_cnt":"sample","pure_pay_gmv":"sample","pay_gmv_per_uv":"sample","seller_actual_settle_amount":"sample","platform_actual_commission":"sample","finderuin_actual_commission":"sample","captain_actual_commission":"sample","seller_predict_settle_amount":"sample","platform_predict_commission":"sample","finderuin_predict_commission":"sample","captain_predict_commission":"sample","product_click_uv":"sample","product_click_cnt":"sample","pay_refund_gmv":"sample","pay_refund_uv":"sample","pay_refund_ratio":1.5,"pay_refund_after_send_ratio":1.5,"pay_refund_cnt":"sample","pay_refund_product_cnt":"sample","pay_refund_before_send_ratio":1.5,"refund_gmv":"sample","refund_product_cnt":"sample","refund_cnt":"sample","refund_uv":"sample"}}]}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_product_list_response::ShopProductListResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_product_list_response",
    );
}

#[test]
fn bean_compass_shop_shop_sale_profile_data() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"field_list":[{"field_name":"sample","data_list":[{"dim_key":"sample","dim_value":"sample"}]}]}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_sale_profile_data::ShopSaleProfileData =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_sale_profile_data",
    );
}

#[test]
fn bean_compass_shop_shop_sale_profile_data_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"ds":"sample","type":17}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_sale_profile_data_param::ShopSaleProfileDataParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_sale_profile_data_param",
    );
}

#[test]
fn bean_compass_shop_shop_sale_profile_data_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"field_list":[{"field_name":"sample","data_list":[{"dim_key":"sample","dim_value":"sample"}]}]}}"##).unwrap();
    let data: wx_rust_store::bean::compass::shop::shop_sale_profile_data_response::ShopSaleProfileDataResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_compass_shop_shop_sale_profile_data_response",
    );
}

#[test]
fn bean_complaint_complaint_history() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"item_type":17,"time":3000000000,"phone_number":"sample","content":"sample","media_id_list":["sample"],"after_sale_type":17,"after_sale_reason":17}"##).unwrap();
    let data: wx_rust_store::bean::complaint::complaint_history::ComplaintHistory =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_complaint_complaint_history",
    );
}

#[test]
fn bean_complaint_complaint_order_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","after_sale_order_id":"sample","order_id":"sample","history":[{"item_type":17,"time":3000000000,"phone_number":"sample","content":"sample","media_id_list":["sample"],"after_sale_type":17,"after_sale_reason":17}],"status":17}"##).unwrap();
    let data: wx_rust_store::bean::complaint::complaint_order_response::ComplaintOrderResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_complaint_complaint_order_response",
    );
}

#[test]
fn bean_complaint_complaint_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"complaint_id":"sample","content":"sample","media_id_list":["sample"]}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::complaint::complaint_param::ComplaintParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_complaint_complaint_param",
    );
}

#[test]
fn bean_cooperation_cooperation_data() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"sharer_id":"sample","status":17,"sharer_name":"sample","sharer_type":17,"bind_time":3000000000,"reject_time":3000000000,"cancel_time":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::cooperation::cooperation_data::CooperationData =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_cooperation_cooperation_data",
    );
}

#[test]
fn bean_cooperation_cooperation_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data_list":[{"sharer_id":"sample","status":17,"sharer_name":"sample","sharer_type":17,"bind_time":3000000000,"reject_time":3000000000,"cancel_time":3000000000}]}"##).unwrap();
    let data: wx_rust_store::bean::cooperation::cooperation_list_response::CooperationListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_cooperation_cooperation_list_response",
    );
}

#[test]
fn bean_cooperation_cooperation_qr_code() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"qrcode_base64":17}"##).unwrap();
    let data: wx_rust_store::bean::cooperation::cooperation_qr_code::CooperationQrCode =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_cooperation_cooperation_qr_code",
    );
}

#[test]
fn bean_cooperation_cooperation_qr_code_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"qrcode_base64":17}}"##)
            .unwrap();
    let data: wx_rust_store::bean::cooperation::cooperation_qr_code_response::CooperationQrCodeResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_cooperation_cooperation_qr_code_response",
    );
}

#[test]
fn bean_cooperation_cooperation_sharer_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"sharer_id":"sample","sharer_type":17}"##).unwrap();
    let data: wx_rust_store::bean::cooperation::cooperation_sharer_param::CooperationSharerParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_cooperation_cooperation_sharer_param",
    );
}

#[test]
fn bean_cooperation_cooperation_status() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"status":17}"##).unwrap();
    let data: wx_rust_store::bean::cooperation::cooperation_status::CooperationStatus =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_cooperation_cooperation_status",
    );
}

#[test]
fn bean_cooperation_cooperation_status_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"status":17}}"##).unwrap();
    let data: wx_rust_store::bean::cooperation::cooperation_status_response::CooperationStatusResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_cooperation_cooperation_status_response",
    );
}

#[test]
fn bean_coupon_auto_valid_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"auto_valid_type":17}"##).unwrap();
    let data: wx_rust_store::bean::coupon::auto_valid_info::AutoValidInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_auto_valid_info",
    );
}

#[test]
fn bean_coupon_coupon_detail_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"name":"sample","valid_info":{"valid_type":17,"valid_day_num":17,"start_time":3000000000,"end_time":3000000000},"promote_info":{"promote_type":17},"discount_info":{"discount_num":17,"discount_fee":17,"discount_condition":{"product_cnt":17,"product_price":17,"product_ids":["sample"]}},"ext_info":{"jump_product_id":"sample","notes":"sample","valid_time":3000000000,"invalid_time":3000000000},"receive_info":{"end_time":3000000000,"limit_num_one_person":17,"start_time":3000000000,"total_num":17}}"##).unwrap();
    let data: wx_rust_store::bean::coupon::coupon_detail_info::CouponDetailInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_coupon_detail_info",
    );
}

#[test]
fn bean_coupon_coupon_id_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"coupon_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::coupon::coupon_id_info::CouponIdInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_coupon_id_info",
    );
}

#[test]
fn bean_coupon_coupon_id_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"coupon_id":"sample"}}"##)
            .unwrap();
    let data: wx_rust_store::bean::coupon::coupon_id_response::CouponIdResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_coupon_id_response",
    );
}

#[test]
fn bean_coupon_coupon_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"coupon_id":"sample","type":17,"status":17,"create_time":3000000000,"update_time":3000000000,"coupon_info":{"name":"sample","valid_info":{"valid_type":17,"valid_day_num":17,"start_time":3000000000,"end_time":3000000000},"promote_info":{"promote_type":17},"discount_info":{"discount_num":17,"discount_fee":17,"discount_condition":{"product_cnt":17,"product_price":17,"product_ids":["sample"]}},"ext_info":{"jump_product_id":"sample","notes":"sample","valid_time":3000000000,"invalid_time":3000000000},"receive_info":{"end_time":3000000000,"limit_num_one_person":17,"start_time":3000000000,"total_num":17}},"stock_info":{"issued_num":17,"receive_num":17,"used_num":17}}"##).unwrap();
    let data: wx_rust_store::bean::coupon::coupon_info::CouponInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_coupon_info",
    );
}

#[test]
fn bean_coupon_coupon_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","coupon":{"coupon_id":"sample","type":17,"status":17,"create_time":3000000000,"update_time":3000000000,"coupon_info":{"name":"sample","valid_info":{"valid_type":17,"valid_day_num":17,"start_time":3000000000,"end_time":3000000000},"promote_info":{"promote_type":17},"discount_info":{"discount_num":17,"discount_fee":17,"discount_condition":{"product_cnt":17,"product_price":17,"product_ids":["sample"]}},"ext_info":{"jump_product_id":"sample","notes":"sample","valid_time":3000000000,"invalid_time":3000000000},"receive_info":{"end_time":3000000000,"limit_num_one_person":17,"start_time":3000000000,"total_num":17}},"stock_info":{"issued_num":17,"receive_num":17,"used_num":17}}}"##).unwrap();
    let data: wx_rust_store::bean::coupon::coupon_info_response::CouponInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_coupon_info_response",
    );
}

#[test]
fn bean_coupon_coupon_list_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"status":17,"page":17,"page_size":17,"page_ctx":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::coupon::coupon_list_param::CouponListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_coupon_list_param",
    );
}

#[test]
fn bean_coupon_coupon_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","coupons":[{"coupon_id":"sample"}],"total_num":17,"page_ctx":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::coupon::coupon_list_response::CouponListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_coupon_list_response",
    );
}

#[test]
fn bean_coupon_coupon_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"coupon_id":"sample","type":17,"name":"sample","discount_info":{"discount_num":17,"discount_fee":17,"discount_condition":{"product_cnt":17,"product_price":17,"product_ids":["sample"]}},"ext_info":{"jump_product_id":"sample","notes":"sample","valid_time":3000000000,"invalid_time":3000000000},"promote_info":{"promote_type":17},"receive_info":{"end_time":3000000000,"limit_num_one_person":17,"start_time":3000000000,"total_num":17},"valid_info":{"valid_type":17,"valid_day_num":17,"start_time":3000000000,"end_time":3000000000},"auto_valid_info":{"auto_valid_type":17}}"##).unwrap();
    let data: wx_rust_store::bean::coupon::coupon_param::CouponParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_coupon_param",
    );
}

#[test]
fn bean_coupon_coupon_status_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"coupon_id":"sample","status":17}"##).unwrap();
    let data: wx_rust_store::bean::coupon::coupon_status_param::CouponStatusParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_coupon_status_param",
    );
}

#[test]
fn bean_coupon_discount_condition() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"product_cnt":17,"product_price":17,"product_ids":["sample"]}"##)
            .unwrap();
    let data: wx_rust_store::bean::coupon::discount_condition::DiscountCondition =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_discount_condition",
    );
}

#[test]
fn bean_coupon_discount_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"discount_num":17,"discount_fee":17,"discount_condition":{"product_cnt":17,"product_price":17,"product_ids":["sample"]}}"##).unwrap();
    let data: wx_rust_store::bean::coupon::discount_info::DiscountInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_discount_info",
    );
}

#[test]
fn bean_coupon_ext_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"jump_product_id":"sample","notes":"sample","valid_time":3000000000,"invalid_time":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::coupon::ext_info::ExtInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_ext_info",
    );
}

#[test]
fn bean_coupon_promote_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"promote_type":17}"##).unwrap();
    let data: wx_rust_store::bean::coupon::promote_info::PromoteInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_promote_info",
    );
}

#[test]
fn bean_coupon_receive_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"end_time":3000000000,"limit_num_one_person":17,"start_time":3000000000,"total_num":17}"##).unwrap();
    let data: wx_rust_store::bean::coupon::receive_info::ReceiveInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_receive_info",
    );
}

#[test]
fn bean_coupon_stock_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"issued_num":17,"receive_num":17,"used_num":17}"##).unwrap();
    let data: wx_rust_store::bean::coupon::stock_info::StockInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_stock_info",
    );
}

#[test]
fn bean_coupon_user_coupon() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"coupon_id":"sample","user_coupon_id":"sample","status":17,"create_time":3000000000,"update_time":3000000000,"start_time":3000000000,"end_time":3000000000,"ext_info":{"use_time":3000000000},"order_id":"sample","discount_fee":17}"##).unwrap();
    let data: wx_rust_store::bean::coupon::user_coupon::UserCoupon =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_user_coupon",
    );
}

#[test]
fn bean_coupon_user_coupon_id_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"coupon_id":"sample","user_coupon_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::coupon::user_coupon_id_info::UserCouponIdInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_user_coupon_id_info",
    );
}

#[test]
fn bean_coupon_user_coupon_id_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"openid":"sample","user_coupon_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::coupon::user_coupon_id_param::UserCouponIdParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_user_coupon_id_param",
    );
}

#[test]
fn bean_coupon_user_coupon_list_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"status":17,"page":17,"page_size":17,"page_ctx":"sample","openid":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::coupon::user_coupon_list_param::UserCouponListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_user_coupon_list_param",
    );
}

#[test]
fn bean_coupon_user_coupon_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","user_coupon_list":[{"coupon_id":"sample","user_coupon_id":"sample"}],"total_num":17,"page_ctx":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::coupon::user_coupon_list_response::UserCouponListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_user_coupon_list_response",
    );
}

#[test]
fn bean_coupon_user_coupon_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","user_coupon":{"coupon_id":"sample","user_coupon_id":"sample","status":17,"create_time":3000000000,"update_time":3000000000,"start_time":3000000000,"end_time":3000000000,"ext_info":{"use_time":3000000000},"order_id":"sample","discount_fee":17},"openid":"sample","unionid":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::coupon::user_coupon_response::UserCouponResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_user_coupon_response",
    );
}

#[test]
fn bean_coupon_user_ext_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"use_time":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::coupon::user_ext_info::UserExtInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_user_ext_info",
    );
}

#[test]
fn bean_coupon_valid_info() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"valid_type":17,"valid_day_num":17,"start_time":3000000000,"end_time":3000000000}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::coupon::valid_info::ValidInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_coupon_valid_info",
    );
}

#[test]
fn bean_delivery_delivery_company_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"delivery_id":"sample","delivery_name":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::delivery::delivery_company_info::DeliveryCompanyInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_delivery_delivery_company_info",
    );
}

#[test]
fn bean_delivery_delivery_company_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","company_list":[{"delivery_id":"sample","delivery_name":"sample"}]}"##).unwrap();
    let data: wx_rust_store::bean::delivery::delivery_company_response::DeliveryCompanyResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_delivery_delivery_company_response",
    );
}

#[test]
fn bean_delivery_delivery_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"waybill_id":"sample","delivery_id":"sample","deliver_type":17,"product_infos":[{"product_id":"sample","sku_id":"sample","product_cnt":17}]}"##).unwrap();
    let data: wx_rust_store::bean::delivery::delivery_info::DeliveryInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_delivery_delivery_info",
    );
}

#[test]
fn bean_delivery_delivery_send_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"order_id":"sample","delivery_list":[{"waybill_id":"sample","delivery_id":"sample","deliver_type":17,"product_infos":[{"product_id":"sample","sku_id":"sample","product_cnt":17}]}]}"##).unwrap();
    let data: wx_rust_store::bean::delivery::delivery_send_param::DeliverySendParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_delivery_delivery_send_param",
    );
}

#[test]
fn bean_delivery_freight_product_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"product_id":"sample","sku_id":"sample","product_cnt":17}"##)
            .unwrap();
    let data: wx_rust_store::bean::delivery::freight_product_info::FreightProductInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_delivery_freight_product_info",
    );
}

#[test]
fn bean_delivery_fresh_inspect_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"order_id":"sample","audit_items":[{"item_name":"sample","item_value":"sample"}]}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::delivery::fresh_inspect_param::FreshInspectParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_delivery_fresh_inspect_param",
    );
}

#[test]
fn bean_delivery_package_audit_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"item_name":"sample","item_value":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::delivery::package_audit_info::PackageAuditInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_delivery_package_audit_info",
    );
}

#[test]
fn bean_ewaybill_account_info_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::ewaybill::account_info_response::AccountInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_ewaybill_account_info_response",
    );
}

#[test]
fn bean_ewaybill_batch_print_order_request() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"req_list":[{"ewaybill_order_id":"sample","delivery_id":"sample","waybill_id":"sample","re_print":true}]}"##).unwrap();
    let data: wx_rust_store::bean::ewaybill::batch_print_order_request::BatchPrintOrderRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_ewaybill_batch_print_order_request",
    );
}

#[test]
fn bean_ewaybill_create_order_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::ewaybill::create_order_response::CreateOrderResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_ewaybill_create_order_response",
    );
}

#[test]
fn bean_ewaybill_delivery_list_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::ewaybill::delivery_list_response::DeliveryListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_ewaybill_delivery_list_response",
    );
}

#[test]
fn bean_ewaybill_order_detail_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::ewaybill::order_detail_response::OrderDetailResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_ewaybill_order_detail_response",
    );
}

#[test]
fn bean_ewaybill_pre_create_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::ewaybill::pre_create_response::PreCreateResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_ewaybill_pre_create_response",
    );
}

#[test]
fn bean_ewaybill_print_content_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::ewaybill::print_content_response::PrintContentResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_ewaybill_print_content_response",
    );
}

#[test]
fn bean_ewaybill_print_order_request() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"ewaybill_order_id":"sample","delivery_id":"sample","waybill_id":"sample","re_print":true}"##).unwrap();
    let data: wx_rust_store::bean::ewaybill::print_order_request::PrintOrderRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_ewaybill_print_order_request",
    );
}

#[test]
fn bean_ewaybill_template_config_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::ewaybill::template_config_response::TemplateConfigResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_ewaybill_template_config_response",
    );
}

#[test]
fn bean_ewaybill_template_id_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","template_id":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::ewaybill::template_id_response::TemplateIdResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_ewaybill_template_id_response",
    );
}

#[test]
fn bean_ewaybill_template_info_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::ewaybill::template_info_response::TemplateInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_ewaybill_template_info_response",
    );
}

#[test]
fn bean_favorite_favorite_count_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","favor_uv_acc_shop_homepage":3000000000,"favor_uv_acc_order_detail":3000000000,"favor_uv_acc_product_detail":3000000000,"favor_uv_acc_other_scene":3000000000,"favor_uv_acc_all":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::favorite::favorite_count_response::FavoriteCountResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_favorite_favorite_count_response",
    );
}

#[test]
fn bean_freight_address_info_list() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}]}"##).unwrap();
    let data: wx_rust_store::bean::freight::address_info_list::AddressInfoList =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_freight_address_info_list",
    );
}

#[test]
fn bean_freight_all_condition_free_detail() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"condition_free_detail_list":[{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}],"min_piece":17,"min_weight":1.5,"min_amount":17,"valuation_flag":17,"amount_flag":17}]}"##).unwrap();
    let data: wx_rust_store::bean::freight::all_condition_free_detail::AllConditionFreeDetail =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_freight_all_condition_free_detail",
    );
}

#[test]
fn bean_freight_all_freight_calc_method() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"freight_calc_method_list":[{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}],"is_default":true,"delivery_id":"sample","first_val_amount":17,"first_price":17,"second_val_amount":17,"second_price":17}]}"##).unwrap();
    let data: wx_rust_store::bean::freight::all_freight_calc_method::AllFreightCalcMethod =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_freight_all_freight_calc_method",
    );
}

#[test]
fn bean_freight_condition_free_detail() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}],"min_piece":17,"min_weight":1.5,"min_amount":17,"valuation_flag":17,"amount_flag":17}"##).unwrap();
    let data: wx_rust_store::bean::freight::condition_free_detail::ConditionFreeDetail =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_freight_condition_free_detail",
    );
}

#[test]
fn bean_freight_freight_calc_method() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}],"is_default":true,"delivery_id":"sample","first_val_amount":17,"first_price":17,"second_val_amount":17,"second_price":17}"##).unwrap();
    let data: wx_rust_store::bean::freight::freight_calc_method::FreightCalcMethod =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_freight_freight_calc_method",
    );
}

#[test]
fn bean_freight_freight_template() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"template_id":"sample","name":"sample","valuation_type":"sample","send_time":"sample","address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5},"delivery_type":"sample","shipping_method":"sample","all_condition_free_detail":{"condition_free_detail_list":[{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}],"min_piece":17,"min_weight":1.5,"min_amount":17,"valuation_flag":17,"amount_flag":17}]},"all_freight_calc_method":{"freight_calc_method_list":[{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}],"is_default":true,"delivery_id":"sample","first_val_amount":17,"first_price":17,"second_val_amount":17,"second_price":17}]},"create_time":3000000000,"update_time":3000000000,"is_default":true,"not_send_area":{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}]}}"##).unwrap();
    let data: wx_rust_store::bean::freight::freight_template::FreightTemplate =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_freight_freight_template",
    );
}

#[test]
fn bean_freight_not_send_area() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}]}"##).unwrap();
    let data: wx_rust_store::bean::freight::not_send_area::NotSendArea =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_freight_not_send_area",
    );
}

#[test]
fn bean_freight_template_add_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"freight_template":{"template_id":"sample","name":"sample","valuation_type":"sample","send_time":"sample","address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5},"delivery_type":"sample","shipping_method":"sample","all_condition_free_detail":{"condition_free_detail_list":[{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}],"min_piece":17,"min_weight":1.5,"min_amount":17,"valuation_flag":17,"amount_flag":17}]},"all_freight_calc_method":{"freight_calc_method_list":[{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}],"is_default":true,"delivery_id":"sample","first_val_amount":17,"first_price":17,"second_val_amount":17,"second_price":17}]},"create_time":3000000000,"update_time":3000000000,"is_default":true,"not_send_area":{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}]}}}"##).unwrap();
    let data: wx_rust_store::bean::freight::template_add_param::TemplateAddParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_freight_template_add_param",
    );
}

#[test]
fn bean_freight_template_id_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","template_id":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::freight::template_id_response::TemplateIdResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_freight_template_id_response",
    );
}

#[test]
fn bean_freight_template_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","freight_template":{"template_id":"sample","name":"sample","valuation_type":"sample","send_time":"sample","address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5},"delivery_type":"sample","shipping_method":"sample","all_condition_free_detail":{"condition_free_detail_list":[{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}],"min_piece":17,"min_weight":1.5,"min_amount":17,"valuation_flag":17,"amount_flag":17}]},"all_freight_calc_method":{"freight_calc_method_list":[{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}],"is_default":true,"delivery_id":"sample","first_val_amount":17,"first_price":17,"second_val_amount":17,"second_price":17}]},"create_time":3000000000,"update_time":3000000000,"is_default":true,"not_send_area":{"address_infos":[{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}]}}}"##).unwrap();
    let data: wx_rust_store::bean::freight::template_info_response::TemplateInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_freight_template_info_response",
    );
}

#[test]
fn bean_freight_template_list_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"offset":17,"limit":17}"##).unwrap();
    let data: wx_rust_store::bean::freight::template_list_param::TemplateListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_freight_template_list_param",
    );
}

#[test]
fn bean_freight_template_list_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","template_id_list":["sample"]}"##)
            .unwrap();
    let data: wx_rust_store::bean::freight::template_list_response::TemplateListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_freight_template_list_response",
    );
}

#[test]
fn bean_fund_account_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"bank_account_type":"sample","account_bank":"sample","bank_address_code":"sample","bank_branch_id":"sample","bank_name":"sample","account_number":"sample","account_bank4show":"sample","account_name":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::fund::account_info::AccountInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_account_info",
    );
}

#[test]
fn bean_fund_account_info_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"account_info":{"bank_account_type":"sample","account_bank":"sample","bank_address_code":"sample","bank_branch_id":"sample","bank_name":"sample","account_number":"sample","account_bank4show":"sample","account_name":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::fund::account_info_param::AccountInfoParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_account_info_param",
    );
}

#[test]
fn bean_fund_account_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","account_info":{"bank_account_type":"sample","account_bank":"sample","bank_address_code":"sample","bank_branch_id":"sample","bank_name":"sample","account_number":"sample","account_bank4show":"sample","account_name":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::fund::account_info_response::AccountInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_account_info_response",
    );
}

#[test]
fn bean_fund_balance_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","available_amount":17,"pending_amount":17,"sub_mchid":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::fund::balance_info_response::BalanceInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_balance_info_response",
    );
}

#[test]
fn bean_fund_flow_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","flow_ids":["sample"],"has_more":true,"next_key":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::fund::flow_list_response::FlowListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_flow_list_response",
    );
}

#[test]
fn bean_fund_flow_related_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"related_type":17,"order_id":"sample","aftersale_id":"sample","withdraw_id":"sample","bookkeeping_time":"sample","insurance_id":"sample","transaction_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::fund::flow_related_info::FlowRelatedInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_flow_related_info",
    );
}

#[test]
fn bean_fund_funds_flow() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"flow_id":"sample","funds_type":17,"flow_type":17,"amount":17,"balance":17,"related_info_list":[{"related_type":17,"order_id":"sample","aftersale_id":"sample","withdraw_id":"sample","bookkeeping_time":"sample","insurance_id":"sample","transaction_id":"sample"}],"bookkeeping_time":"sample","remark":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::fund::funds_flow::FundsFlow =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_funds_flow",
    );
}

#[test]
fn bean_fund_funds_flow_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","funds_flow":{"flow_id":"sample","funds_type":17,"flow_type":17,"amount":17,"balance":17,"related_info_list":[{"related_type":17,"order_id":"sample","aftersale_id":"sample","withdraw_id":"sample","bookkeeping_time":"sample","insurance_id":"sample","transaction_id":"sample"}],"bookkeeping_time":"sample","remark":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::fund::funds_flow_response::FundsFlowResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_funds_flow_response",
    );
}

#[test]
fn bean_fund_funds_list_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"page":17,"page_size":17,"start_time":3000000000,"end_time":3000000000,"flow_type":17,"transaction_id":"sample","next_key":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::fund::funds_list_param::FundsListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_funds_list_param",
    );
}

#[test]
fn bean_fund_withdraw_detail_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","amount":17,"create_time":3000000000,"update_time":3000000000,"reason":"sample","remark":"sample","bank_memo":"sample","bank_name":"sample","bank_num":"sample","status":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::fund::withdraw_detail_response::WithdrawDetailResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_withdraw_detail_response",
    );
}

#[test]
fn bean_fund_withdraw_list_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"page_num":17,"page_size":17,"start_time":3000000000,"end_time":3000000000}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::fund::withdraw_list_param::WithdrawListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_withdraw_list_param",
    );
}

#[test]
fn bean_fund_withdraw_list_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","withdraw_ids":["sample"]}"##)
            .unwrap();
    let data: wx_rust_store::bean::fund::withdraw_list_response::WithdrawListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_withdraw_list_response",
    );
}

#[test]
fn bean_fund_withdraw_submit_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"amount":17,"remark":"sample","bank_memo":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::fund::withdraw_submit_param::WithdrawSubmitParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_withdraw_submit_param",
    );
}

#[test]
fn bean_fund_withdraw_submit_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","qrcode_ticket":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::fund::withdraw_submit_response::WithdrawSubmitResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_withdraw_submit_response",
    );
}

#[test]
fn bean_fund_bank_bank_city_info() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"city_name":"sample","city_code":17,"bank_address_code":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::fund::bank::bank_city_info::BankCityInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_bank_bank_city_info",
    );
}

#[test]
fn bean_fund_bank_bank_city_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":[{"city_name":"sample","city_code":17,"bank_address_code":"sample"}],"total_count":17}"##).unwrap();
    let data: wx_rust_store::bean::fund::bank::bank_city_response::BankCityResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_bank_bank_city_response",
    );
}

#[test]
fn bean_fund_bank_bank_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"account_bank":"sample","bank_code":"sample","bank_id":"sample","bank_name":"sample","bank_type":17,"need_branch":true,"branch_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::fund::bank::bank_info::BankInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_bank_bank_info",
    );
}

#[test]
fn bean_fund_bank_bank_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":[{"account_bank":"sample","bank_code":"sample","bank_id":"sample","bank_name":"sample","bank_type":17,"need_branch":true,"branch_id":"sample"}],"total_count":17}"##).unwrap();
    let data: wx_rust_store::bean::fund::bank::bank_info_response::BankInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_bank_bank_info_response",
    );
}

#[test]
fn bean_fund_bank_bank_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":[{"account_bank":"sample","bank_code":"sample","bank_id":"sample","bank_name":"sample","bank_type":17,"need_branch":true,"branch_id":"sample"}]}"##).unwrap();
    let data: wx_rust_store::bean::fund::bank::bank_list_response::BankListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_bank_bank_list_response",
    );
}

#[test]
fn bean_fund_bank_bank_province_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"province_name":"sample","province_code":17}"##).unwrap();
    let data: wx_rust_store::bean::fund::bank::bank_province_info::BankProvinceInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_bank_bank_province_info",
    );
}

#[test]
fn bean_fund_bank_bank_province_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":[{"province_name":"sample","province_code":17}],"total_count":17}"##).unwrap();
    let data: wx_rust_store::bean::fund::bank::bank_province_response::BankProvinceResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_bank_bank_province_response",
    );
}

#[test]
fn bean_fund_bank_bank_search_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"offset":17,"limit":17,"key_words":"sample","bank_type":17}"##)
            .unwrap();
    let data: wx_rust_store::bean::fund::bank::bank_search_param::BankSearchParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_bank_bank_search_param",
    );
}

#[test]
fn bean_fund_bank_branch_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"branch_id":17,"branch_name":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::fund::bank::branch_info::BranchInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_bank_branch_info",
    );
}

#[test]
fn bean_fund_bank_branch_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","total_count":17,"count":17,"account_bank":"sample","account_bank_code":"sample","bank_alias":"sample","bank_alias_code":"sample","data":[{"branch_id":17,"branch_name":"sample"}]}"##).unwrap();
    let data: wx_rust_store::bean::fund::bank::branch_info_response::BranchInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_bank_branch_info_response",
    );
}

#[test]
fn bean_fund_bank_branch_search_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"bank_code":"sample","city_code":"sample","offset":17,"limit":17}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::fund::bank::branch_search_param::BranchSearchParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_bank_branch_search_param",
    );
}

#[test]
fn bean_fund_qrcode_qr_check_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","status":17,"self_check_err_code":17,"self_check_err_msg":"sample","scan_user_type":17}"##).unwrap();
    let data: wx_rust_store::bean::fund::qrcode::qr_check_response::QrCheckResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_qrcode_qr_check_response",
    );
}

#[test]
fn bean_fund_qrcode_qr_code_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","qrcode_buf":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::fund::qrcode::qr_code_response::QrCodeResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_fund_qrcode_qr_code_response",
    );
}

#[test]
fn bean_home_background_background_apply_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","apply_id":17}"##).unwrap();
    let data: wx_rust_store::bean::home::background::background_apply_response::BackgroundApplyResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_background_background_apply_response",
    );
}

#[test]
fn bean_home_background_background_apply_result() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"apply_id":17,"state":17,"audit_desc":"sample","img_url":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::home::background::background_apply_result::BackgroundApplyResult = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_background_background_apply_result",
    );
}

#[test]
fn bean_home_background_background_get_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","img_url":"sample","apply":{"apply_id":17,"state":17,"audit_desc":"sample","img_url":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::home::background::background_get_response::BackgroundGetResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_background_background_get_response",
    );
}

#[test]
fn bean_home_banner_banner_apply_detail() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"audit_state":17,"audit_desc":"sample","banner":{"type":17,"banner":{"img_url":"sample","title":"sample","description":"sample"},"product":{"product_id":3000000000},"finder":{"finder_user_name":"sample","feed_id":"sample"},"official_account":{"url":"sample"}}}"##).unwrap();
    let data: wx_rust_store::bean::home::banner::banner_apply_detail::BannerApplyDetail =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_banner_banner_apply_detail",
    );
}

#[test]
fn bean_home_banner_banner_apply_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"apply_id":17,"state":17,"scale":17,"banner":[{"audit_state":17,"audit_desc":"sample","banner":{"type":17,"banner":{"img_url":"sample","title":"sample","description":"sample"},"product":{"product_id":3000000000},"finder":{"finder_user_name":"sample","feed_id":"sample"},"official_account":{"url":"sample"}}}]}"##).unwrap();
    let data: wx_rust_store::bean::home::banner::banner_apply_info::BannerApplyInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_banner_banner_apply_info",
    );
}

#[test]
fn bean_home_banner_banner_apply_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"banner":{"scale":17,"banner":[{"type":17,"banner":{"img_url":"sample","title":"sample","description":"sample"},"product":{"product_id":3000000000},"finder":{"finder_user_name":"sample","feed_id":"sample"},"official_account":{"url":"sample"}}]}}"##).unwrap();
    let data: wx_rust_store::bean::home::banner::banner_apply_param::BannerApplyParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_banner_banner_apply_param",
    );
}

#[test]
fn bean_home_banner_banner_apply_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","apply_id":17}"##).unwrap();
    let data: wx_rust_store::bean::home::banner::banner_apply_response::BannerApplyResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_banner_banner_apply_response",
    );
}

#[test]
fn bean_home_banner_banner_get_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","banner":{"scale":17,"banner":[{"type":17,"banner":{"img_url":"sample","title":"sample","description":"sample"},"product":{"product_id":3000000000},"finder":{"finder_user_name":"sample","feed_id":"sample"},"official_account":{"url":"sample"}}]},"apply":{"apply_id":17,"state":17,"scale":17,"banner":[{"audit_state":17,"audit_desc":"sample","banner":{"type":17,"banner":{"img_url":"sample","title":"sample","description":"sample"},"product":{"product_id":3000000000},"finder":{"finder_user_name":"sample","feed_id":"sample"},"official_account":{"url":"sample"}}}]}}"##).unwrap();
    let data: wx_rust_store::bean::home::banner::banner_get_response::BannerGetResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_banner_banner_get_response",
    );
}

#[test]
fn bean_home_banner_banner_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"scale":17,"banner":[{"type":17,"banner":{"img_url":"sample","title":"sample","description":"sample"},"product":{"product_id":3000000000},"finder":{"finder_user_name":"sample","feed_id":"sample"},"official_account":{"url":"sample"}}]}"##).unwrap();
    let data: wx_rust_store::bean::home::banner::banner_info::BannerInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_banner_banner_info",
    );
}

#[test]
fn bean_home_banner_banner_item() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"type":17,"banner":{"img_url":"sample","title":"sample","description":"sample"},"product":{"product_id":3000000000},"finder":{"finder_user_name":"sample","feed_id":"sample"},"official_account":{"url":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::home::banner::banner_item::BannerItem =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_banner_banner_item",
    );
}

#[test]
fn bean_home_banner_banner_item_detail() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"img_url":"sample","title":"sample","description":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::home::banner::banner_item_detail::BannerItemDetail =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_banner_banner_item_detail",
    );
}

#[test]
fn bean_home_banner_banner_item_finder() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"finder_user_name":"sample","feed_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::home::banner::banner_item_finder::BannerItemFinder =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_banner_banner_item_finder",
    );
}

#[test]
fn bean_home_banner_banner_item_official_account() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"url":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::home::banner::banner_item_official_account::BannerItemOfficialAccount = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_banner_banner_item_official_account",
    );
}

#[test]
fn bean_home_banner_banner_item_product() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"product_id":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::home::banner::banner_item_product::BannerItemProduct =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_banner_banner_item_product",
    );
}

#[test]
fn bean_home_tree_cat_tree_node() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"id":17,"name":"sample","is_displayed":true}"##).unwrap();
    let data: wx_rust_store::bean::home::tree::cat_tree_node::CatTreeNode =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_cat_tree_node",
    );
}

#[test]
fn bean_home_tree_level_tree_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"level_1":[{"id":17,"name":"sample","is_displayed":true,"level_2":[{"id":17,"name":"sample","is_displayed":true}]}]}"##).unwrap();
    let data: wx_rust_store::bean::home::tree::level_tree_info::LevelTreeInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_level_tree_info",
    );
}

#[test]
fn bean_home_tree_one_level_tree_node() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"id":17,"name":"sample","is_displayed":true,"level_2":[{"id":17,"name":"sample","is_displayed":true}]}"##).unwrap();
    let data: wx_rust_store::bean::home::tree::one_level_tree_node::OneLevelTreeNode =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_one_level_tree_node",
    );
}

#[test]
fn bean_home_tree_tree_audit_result() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"version":17,"audit_results":[{"level_id":17,"result_code":17}]}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::home::tree::tree_audit_result::TreeAuditResult =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_tree_audit_result",
    );
}

#[test]
fn bean_home_tree_tree_audit_result_detail() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"level_id":17,"result_code":17}"##).unwrap();
    let data: wx_rust_store::bean::home::tree::tree_audit_result_detail::TreeAuditResultDetail =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_tree_audit_result_detail",
    );
}

#[test]
fn bean_home_tree_tree_product_edit_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"level_1_id":17,"level_2_id":17,"product_ids":[3000000000]}"##)
            .unwrap();
    let data: wx_rust_store::bean::home::tree::tree_product_edit_info::TreeProductEditInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_tree_product_edit_info",
    );
}

#[test]
fn bean_home_tree_tree_product_edit_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"req":{"level_1_id":17,"level_2_id":17,"product_ids":[3000000000]}}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::home::tree::tree_product_edit_param::TreeProductEditParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_tree_product_edit_param",
    );
}

#[test]
fn bean_home_tree_tree_product_list_info() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"level_1_id":17,"level_2_id":17,"page_size":17,"page_context":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::home::tree::tree_product_list_info::TreeProductListInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_tree_product_list_info",
    );
}

#[test]
fn bean_home_tree_tree_product_list_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"req":{"level_1_id":17,"level_2_id":17,"page_size":17,"page_context":"sample"}}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::home::tree::tree_product_list_param::TreeProductListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_tree_product_list_param",
    );
}

#[test]
fn bean_home_tree_tree_product_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","resp":{"product_ids":[3000000000],"total_count":17,"page_context":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::home::tree::tree_product_list_response::TreeProductListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_tree_product_list_response",
    );
}

#[test]
fn bean_home_tree_tree_product_list_result() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"product_ids":[3000000000],"total_count":17,"page_context":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::home::tree::tree_product_list_result::TreeProductListResult =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_tree_product_list_result",
    );
}

#[test]
fn bean_home_tree_tree_show_get_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","resp":{"tree":{"level_1":[{"id":17,"name":"sample","is_displayed":true,"level_2":[{"id":17,"name":"sample","is_displayed":true}]}]},"version":17,"classification_id_deleted":["sample"]}}"##).unwrap();
    let data: wx_rust_store::bean::home::tree::tree_show_get_response::TreeShowGetResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_tree_show_get_response",
    );
}

#[test]
fn bean_home_tree_tree_show_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"tree":{"level_1":[{"id":17,"name":"sample","is_displayed":true,"level_2":[{"id":17,"name":"sample","is_displayed":true}]}]},"version":17,"classification_id_deleted":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::home::tree::tree_show_info::TreeShowInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_tree_show_info",
    );
}

#[test]
fn bean_home_tree_tree_show_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"req":{"tree":{"level_1":[{"id":17,"name":"sample","is_displayed":true,"level_2":[{"id":17,"name":"sample","is_displayed":true}]}]},"version":17,"classification_id_deleted":["sample"]}}"##).unwrap();
    let data: wx_rust_store::bean::home::tree::tree_show_param::TreeShowParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_tree_show_param",
    );
}

#[test]
fn bean_home_tree_tree_show_set_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","resp":{"version":17,"audit_results":[{"level_id":17,"result_code":17}]}}"##).unwrap();
    let data: wx_rust_store::bean::home::tree::tree_show_set_response::TreeShowSetResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_tree_tree_show_set_response",
    );
}

#[test]
fn bean_home_window_window_product_index_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"product_id":"sample","index_num":17}"##).unwrap();
    let data: wx_rust_store::bean::home::window::window_product_index_param::WindowProductIndexParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_window_window_product_index_param",
    );
}

#[test]
fn bean_home_window_window_product_list_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"page_size":17,"next_key":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::home::window::window_product_list_param::WindowProductListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_window_window_product_list_param",
    );
}

#[test]
fn bean_home_window_window_product_setting() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"product_id":"sample","is_set_hide":17,"is_set_top":17}"##)
            .unwrap();
    let data: wx_rust_store::bean::home::window::window_product_setting::WindowProductSetting =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_window_window_product_setting",
    );
}

#[test]
fn bean_home_window_window_product_setting_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","products":[{"product_id":"sample","is_set_hide":17,"is_set_top":17}],"next_key":"sample","total_num":17}"##).unwrap();
    let data: wx_rust_store::bean::home::window::window_product_setting_response::WindowProductSettingResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_home_window_window_product_setting_response",
    );
}

#[test]
fn bean_image_qualification_file_id() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"file_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::image::qualification_file_id::QualificationFileId =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_image_qualification_file_id",
    );
}

#[test]
fn bean_image_qualification_file_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"file_id":"sample"}}"##)
            .unwrap();
    let data: wx_rust_store::bean::image::qualification_file_response::QualificationFileResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_image_qualification_file_response",
    );
}

#[test]
fn bean_image_store_image_info() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"media_id":"sample","img_url":"sample","pay_media_id":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::image::store_image_info::StoreImageInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_image_store_image_info",
    );
}

#[test]
fn bean_image_store_image_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","contentType":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::image::store_image_response::StoreImageResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_image_store_image_response",
    );
}

#[test]
fn bean_image_upload_image_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","pic_file":{"media_id":"sample","img_url":"sample","pay_media_id":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::image::upload_image_response::UploadImageResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_image_upload_image_response",
    );
}

#[test]
fn bean_kf_wx_store_kf_cos_upload_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","cos_url":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::kf::wx_store_kf_cos_upload_response::WxStoreKfCosUploadResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_kf_wx_store_kf_cos_upload_response",
    );
}

#[test]
fn bean_kf_wx_store_kf_send_msg_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"request_id":"sample","open_id":"sample","msg_type":"sample","text":{"content":"sample"},"image":{"cos_url":"sample"},"video":{"cos_url":"sample"},"file":{"cos_url":"sample"},"product_share":{"product_id":"sample"},"order_share":{"order_id":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::kf::wx_store_kf_send_msg_param::WxStoreKfSendMsgParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_kf_wx_store_kf_send_msg_param",
    );
}

#[test]
fn bean_kf_wx_store_kf_send_msg_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","msg_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::kf::wx_store_kf_send_msg_response::WxStoreKfSendMsgResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_kf_wx_store_kf_send_msg_response",
    );
}

#[test]
fn bean_limit_limit_sku() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"sku_id":"sample","sale_price":17,"sale_stock":17}"##).unwrap();
    let data: wx_rust_store::bean::limit::limit_sku::LimitSku =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_limit_limit_sku",
    );
}

#[test]
fn bean_limit_limit_task_add_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","task_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::limit::limit_task_add_response::LimitTaskAddResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_limit_limit_task_add_response",
    );
}

#[test]
fn bean_limit_limit_task_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"task_id":"sample","product_id":"sample","status":17,"create_time":3000000000,"start_time":3000000000,"end_time":3000000000,"limited_discount_skus":[{"sku_id":"sample","sale_price":17,"sale_stock":17}]}"##).unwrap();
    let data: wx_rust_store::bean::limit::limit_task_info::LimitTaskInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_limit_limit_task_info",
    );
}

#[test]
fn bean_limit_limit_task_list_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"page_size":17,"next_key":"sample","status":17}"##).unwrap();
    let data: wx_rust_store::bean::limit::limit_task_list_param::LimitTaskListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_limit_limit_task_list_param",
    );
}

#[test]
fn bean_limit_limit_task_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","limited_discount_tasks":[{"task_id":"sample","product_id":"sample","status":17,"create_time":3000000000,"start_time":3000000000,"end_time":3000000000,"limited_discount_skus":[{"sku_id":"sample","sale_price":17,"sale_stock":17}]}],"next_key":"sample","total_num":17}"##).unwrap();
    let data: wx_rust_store::bean::limit::limit_task_list_response::LimitTaskListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_limit_limit_task_list_response",
    );
}

#[test]
fn bean_limit_limit_task_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","start_time":"sample","end_time":"sample","limited_discount_skus":[{"sku_id":"sample","sale_price":17,"sale_stock":17}]}"##).unwrap();
    let data: wx_rust_store::bean::limit::limit_task_param::LimitTaskParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_limit_limit_task_param",
    );
}

#[test]
fn bean_limit_limit_task_update_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"task_id":"sample","status":17,"start_time":3000000000,"end_time":3000000000,"title":"sample","limited_discount_skus":[{"product_id":"sample","sku_id":"sample","sale_price":17,"sale_stock":17}]}"##).unwrap();
    let data: wx_rust_store::bean::limit::limit_task_update_param::LimitTaskUpdateParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_limit_limit_task_update_param",
    );
}

#[test]
fn bean_limit_limit_task_update_response() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"errcode":0,"errmsg":"sample","task_id":"sample","title":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::limit::limit_task_update_response::LimitTaskUpdateResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_limit_limit_task_update_response",
    );
}

#[test]
fn bean_order_after_sale_detail() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"on_aftersale_order_cnt":17,"aftersale_order_list":[{"aftersale_order_id":"sample"}]}"##).unwrap();
    let data: wx_rust_store::bean::order::after_sale_detail::AfterSaleDetail =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_after_sale_detail",
    );
}

#[test]
fn bean_order_after_sale_order_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"aftersale_order_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::after_sale_order_info::AfterSaleOrderInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_after_sale_order_info",
    );
}

#[test]
fn bean_order_change_order_info() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"product_id":"sample","sku_id":"sample","change_price":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::order::change_order_info::ChangeOrderInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_change_order_info",
    );
}

#[test]
fn bean_order_change_sku_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"preshipment_change_sku_state":17,"old_sku_id":"sample","new_sku_id":"sample","ddl_time_stamp":17}"##).unwrap();
    let data: wx_rust_store::bean::order::change_sku_info::ChangeSkuInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_change_sku_info",
    );
}

#[test]
fn bean_order_decode_address_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::decode_address_info::DecodeAddressInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_decode_address_info",
    );
}

#[test]
fn bean_order_decode_sensitive_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample"},"virtual_number_info":{"virtual_number":"sample","extension":"sample","expiration":3000000000}}"##).unwrap();
    let data: wx_rust_store::bean::order::decode_sensitive_info_response::DecodeSensitiveInfoResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_decode_sensitive_info_response",
    );
}

#[test]
fn bean_order_delivery_product_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"waybill_id":"sample","delivery_id":"sample","product_infos":[{"product_id":"sample","sku_id":"sample","product_cnt":17}],"delivery_name":"sample","delivery_time":3000000000,"deliver_type":17,"delivery_address":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample","tel_number_ext_info":{"real_tel_number":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":3000000000},"use_tel_number":17,"hash_code":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::order::delivery_product_info::DeliveryProductInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_delivery_product_info",
    );
}

#[test]
fn bean_order_delivery_update_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"order_id":"sample","delivery_list":[{"waybill_id":"sample","delivery_id":"sample","deliver_type":17,"product_infos":[{"product_id":"sample","sku_id":"sample","product_cnt":17}]}]}"##).unwrap();
    let data: wx_rust_store::bean::order::delivery_update_param::DeliveryUpdateParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_delivery_update_param",
    );
}

#[test]
fn bean_order_dropship_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"ds_order_id":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::order::dropship_info::DropshipInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_dropship_info",
    );
}

#[test]
fn bean_order_free_gift_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"main_product_list":[{"gift_cnt":17,"task_id":17,"product_id":"sample","sku_id":17}]}"##).unwrap();
    let data: wx_rust_store::bean::order::free_gift_info::FreeGiftInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_free_gift_info",
    );
}

#[test]
fn bean_order_main_product_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"gift_cnt":17,"task_id":17,"product_id":"sample","sku_id":17}"##)
            .unwrap();
    let data: wx_rust_store::bean::order::main_product_info::MainProductInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_main_product_info",
    );
}

#[test]
fn bean_order_order_address_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample","tel_number_ext_info":{"real_tel_number":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":3000000000},"use_tel_number":17,"hash_code":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::order_address_info::OrderAddressInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_address_info",
    );
}

#[test]
fn bean_order_order_address_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"order_id":"sample","user_address":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5}}"##).unwrap();
    let data: wx_rust_store::bean::order::order_address_param::OrderAddressParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_address_param",
    );
}

#[test]
fn bean_order_order_agent_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"agent_finder_id":"sample","agent_finder_nickname":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::order::order_agent_info::OrderAgentInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_agent_info",
    );
}

#[test]
fn bean_order_order_commission_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"sku_id":"sample","nickname":"sample","type":17,"status":17,"amount":17,"finder_id":"sample","openfinderid":"sample","talent_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::order_commission_info::OrderCommissionInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_commission_info",
    );
}

#[test]
fn bean_order_order_compensation_delivery_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"order_id":"sample","delivery_list":[{"waybill_id":"sample","delivery_id":"sample","deliver_type":17,"product_infos":[{"product_id":"sample","sku_id":"sample","product_cnt":17}]}]}"##).unwrap();
    let data: wx_rust_store::bean::order::order_compensation_delivery_param::OrderCompensationDeliveryParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_compensation_delivery_param",
    );
}

#[test]
fn bean_order_order_coupon_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"user_coupon_id":"sample","coupon_type":17,"discounted_price":17,"coupon_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::order_coupon_info::OrderCouponInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_coupon_info",
    );
}

#[test]
fn bean_order_order_custom_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"custom_img_url":"sample","custom_word":"sample","custom_type":17,"custom_preview_img_url":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::order_custom_info::OrderCustomInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_custom_info",
    );
}

#[test]
fn bean_order_order_detail_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_infos":[{"product_id":"sample","sku_id":"sample","thumb_img":"sample","sku_cnt":17,"sale_price":17,"title":"sample","on_aftersale_sku_cnt":17,"finish_aftersale_sku_cnt":17,"sku_code":"sample","market_price":17,"sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"real_price":17,"out_product_id":"sample","out_sku_id":"sample","is_discounted":true,"estimate_price":17,"is_change_price":true,"change_price":17,"out_warehouse_id":"sample","sku_deliver_info":{"stock_type":17,"predict_delivery_time":"sample"},"extra_service":{"seven_day_return":17,"freight_insurance":17},"use_deduction":true,"deduction_price":17,"order_product_coupon_info_list":[{"user_coupon_id":"sample","coupon_type":17,"discounted_price":17,"coupon_id":"sample"}],"delivery_deadline":3000000000,"merchant_discounted_price":17,"finder_discounted_price":17,"is_free_gift":true,"vip_discounted_price":17,"product_unique_id":"sample","change_sku_info":{"preshipment_change_sku_state":17,"old_sku_id":"sample","new_sku_id":"sample","ddl_time_stamp":17},"free_gift_info":{"main_product_list":[{"gift_cnt":17,"task_id":17,"product_id":"sample","sku_id":17}]},"bulkbuy_discounted_price":17,"national_subsidy_discounted_price":17,"dropship_info":{"ds_order_id":3000000000},"is_flash_sale":true,"national_subsidy_merchant_discounted_price":17,"platform_activity_merchant_discounted_price":17,"cash_coupon_discounted_price":17}],"pay_info":{"payment_method":17,"pay_time":3000000000,"transaction_id":"sample"},"price_info":{"product_price":17,"order_price":17,"freight":17,"discounted_price":17,"is_discounted":true,"original_order_price":17,"estimate_product_price":17,"change_down_price":17,"change_freight":17,"is_change_freight":true,"use_deduction":true,"deduction_price":17,"merchant_receieve_price":17,"merchant_discounted_price":17,"finder_discounted_price":17,"vip_discounted_price":17,"bulkbuy_discounted_price":17,"national_subsidy_discounted_price":17,"cash_coupon_discounted_price":17,"national_subsidy_merchant_discounted_price":17},"delivery_info":{"address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample","tel_number_ext_info":{"real_tel_number":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":3000000000},"use_tel_number":17,"hash_code":"sample"},"delivery_product_info":[{"waybill_id":"sample","delivery_id":"sample","product_infos":[{"product_id":"sample","sku_id":"sample","product_cnt":17}],"delivery_name":"sample","delivery_time":3000000000,"deliver_type":17,"delivery_address":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample","tel_number_ext_info":{"real_tel_number":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":3000000000},"use_tel_number":17,"hash_code":"sample"}}],"ship_done_time":3000000000,"deliver_method":17,"address_under_review":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample","tel_number_ext_info":{"real_tel_number":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":3000000000},"use_tel_number":17,"hash_code":"sample"},"address_apply_time":3000000000,"ewaybill_order_code":"sample","quality_inspect_type":"sample","quality_inspect_info":{"inspect_status":17},"recharge_info":{"account_no":"sample","account_type":"sample","wx_openid":"sample"}},"coupon_info":{"user_coupon_id":"sample","coupon_type":17,"discounted_price":17,"coupon_id":"sample"},"ext_info":{"customer_notes":"sample","merchant_notes":"sample","confirm_receipt_time":3000000000,"finder_id":"sample","live_id":"sample","order_scene":17},"commission_infos":[{"sku_id":"sample","nickname":"sample","type":17,"status":17,"amount":17,"finder_id":"sample","openfinderid":"sample","talent_id":"sample"}],"sharer_info":{"sharer_openid":"sample","sharer_unionid":"sample","sharer_type":17,"share_scene":17,"handling_progress":17},"settle_info":{"predict_commission_fee":17,"commission_fee":17,"predict_wecoin_commission":17,"wecoin_commission":17,"settle_time":3000000000},"sku_sharer_infos":[{"sharer_openid":"sample","sharer_unionid":"sample","sharer_type":17,"share_scene":17,"sku_id":"sample","from_wecom":true}],"agent_info":{"agent_finder_id":"sample","agent_finder_nickname":"sample"},"source_infos":[{"sku_id":"sample","account_type":17,"account_id":"sample","sale_channel":17,"account_nickname":"sample","content_type":"sample","content_id":"sample","promoter_head_supplier_id":"sample"}],"refund_info":{"sku_id":"sample","account_type":17,"account_id":"sample","sale_channel":17,"account_nickname":"sample","content_type":"sample","content_id":"sample","promoter_head_supplier_id":"sample"},"greeting_card_info":{"giver_name":"sample","receiver_name":"sample","greeting_message":"sample"},"custom_info":{"custom_img_url":"sample","custom_word":"sample","custom_type":17,"custom_preview_img_url":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::order::order_detail_info::OrderDetailInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_detail_info",
    );
}

#[test]
fn bean_order_order_greeting_card_info() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"giver_name":"sample","receiver_name":"sample","greeting_message":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::order::order_greeting_card_info::OrderGreetingCardInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_greeting_card_info",
    );
}

#[test]
fn bean_order_order_id_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"order_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::order_id_param::OrderIdParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_id_param",
    );
}

#[test]
fn bean_order_order_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"order_id":"sample","status":17,"openid":"sample","unionid":"sample","order_detail":{"product_infos":[{"product_id":"sample","sku_id":"sample","thumb_img":"sample","sku_cnt":17,"sale_price":17,"title":"sample","on_aftersale_sku_cnt":17,"finish_aftersale_sku_cnt":17,"sku_code":"sample","market_price":17,"sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"real_price":17,"out_product_id":"sample","out_sku_id":"sample","is_discounted":true,"estimate_price":17,"is_change_price":true,"change_price":17,"out_warehouse_id":"sample","sku_deliver_info":{"stock_type":17,"predict_delivery_time":"sample"},"extra_service":{"seven_day_return":17,"freight_insurance":17},"use_deduction":true,"deduction_price":17,"order_product_coupon_info_list":[{"user_coupon_id":"sample","coupon_type":17,"discounted_price":17,"coupon_id":"sample"}],"delivery_deadline":3000000000,"merchant_discounted_price":17,"finder_discounted_price":17,"is_free_gift":true,"vip_discounted_price":17,"product_unique_id":"sample","change_sku_info":{"preshipment_change_sku_state":17,"old_sku_id":"sample","new_sku_id":"sample","ddl_time_stamp":17},"free_gift_info":{"main_product_list":[{"gift_cnt":17,"task_id":17,"product_id":"sample","sku_id":17}]},"bulkbuy_discounted_price":17,"national_subsidy_discounted_price":17,"dropship_info":{"ds_order_id":3000000000},"is_flash_sale":true,"national_subsidy_merchant_discounted_price":17,"platform_activity_merchant_discounted_price":17,"cash_coupon_discounted_price":17}],"pay_info":{"payment_method":17,"pay_time":3000000000,"transaction_id":"sample"},"price_info":{"product_price":17,"order_price":17,"freight":17,"discounted_price":17,"is_discounted":true,"original_order_price":17,"estimate_product_price":17,"change_down_price":17,"change_freight":17,"is_change_freight":true,"use_deduction":true,"deduction_price":17,"merchant_receieve_price":17,"merchant_discounted_price":17,"finder_discounted_price":17,"vip_discounted_price":17,"bulkbuy_discounted_price":17,"national_subsidy_discounted_price":17,"cash_coupon_discounted_price":17,"national_subsidy_merchant_discounted_price":17},"delivery_info":{"address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample","tel_number_ext_info":{"real_tel_number":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":3000000000},"use_tel_number":17,"hash_code":"sample"},"delivery_product_info":[{"waybill_id":"sample","delivery_id":"sample","product_infos":[{"product_id":"sample","sku_id":"sample","product_cnt":17}],"delivery_name":"sample","delivery_time":3000000000,"deliver_type":17,"delivery_address":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample","tel_number_ext_info":{"real_tel_number":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":3000000000},"use_tel_number":17,"hash_code":"sample"}}],"ship_done_time":3000000000,"deliver_method":17,"address_under_review":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample","tel_number_ext_info":{"real_tel_number":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":3000000000},"use_tel_number":17,"hash_code":"sample"},"address_apply_time":3000000000,"ewaybill_order_code":"sample","quality_inspect_type":"sample","quality_inspect_info":{"inspect_status":17},"recharge_info":{"account_no":"sample","account_type":"sample","wx_openid":"sample"}},"coupon_info":{"user_coupon_id":"sample","coupon_type":17,"discounted_price":17,"coupon_id":"sample"},"ext_info":{"customer_notes":"sample","merchant_notes":"sample","confirm_receipt_time":3000000000,"finder_id":"sample","live_id":"sample","order_scene":17},"commission_infos":[{"sku_id":"sample","nickname":"sample","type":17,"status":17,"amount":17,"finder_id":"sample","openfinderid":"sample","talent_id":"sample"}],"sharer_info":{"sharer_openid":"sample","sharer_unionid":"sample","sharer_type":17,"share_scene":17,"handling_progress":17},"settle_info":{"predict_commission_fee":17,"commission_fee":17,"predict_wecoin_commission":17,"wecoin_commission":17,"settle_time":3000000000},"sku_sharer_infos":[{"sharer_openid":"sample","sharer_unionid":"sample","sharer_type":17,"share_scene":17,"sku_id":"sample","from_wecom":true}],"agent_info":{"agent_finder_id":"sample","agent_finder_nickname":"sample"},"source_infos":[{"sku_id":"sample","account_type":17,"account_id":"sample","sale_channel":17,"account_nickname":"sample","content_type":"sample","content_id":"sample","promoter_head_supplier_id":"sample"}],"refund_info":{"sku_id":"sample","account_type":17,"account_id":"sample","sale_channel":17,"account_nickname":"sample","content_type":"sample","content_id":"sample","promoter_head_supplier_id":"sample"},"greeting_card_info":{"giver_name":"sample","receiver_name":"sample","greeting_message":"sample"},"custom_info":{"custom_img_url":"sample","custom_word":"sample","custom_type":17,"custom_preview_img_url":"sample"}},"aftersale_detail":{"on_aftersale_order_cnt":17,"aftersale_order_list":[{"aftersale_order_id":"sample"}]},"is_present":true,"present_order_id_str":"sample","present_note":"sample","present_giver_openid":"sample","present_giver_unionid":"sample","create_time":17,"update_time":17}"##).unwrap();
    let data: wx_rust_store::bean::order::order_info::OrderInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_info",
    );
}

#[test]
fn bean_order_order_info_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"order_id":"sample","encode_sensitive_info":true}"##).unwrap();
    let data: wx_rust_store::bean::order::order_info_param::OrderInfoParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_info_param",
    );
}

#[test]
fn bean_order_order_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","order":{"order_id":"sample","status":17,"openid":"sample","unionid":"sample","order_detail":{"product_infos":[{"product_id":"sample","sku_id":"sample","thumb_img":"sample","sku_cnt":17,"sale_price":17,"title":"sample","on_aftersale_sku_cnt":17,"finish_aftersale_sku_cnt":17,"sku_code":"sample","market_price":17,"sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"real_price":17,"out_product_id":"sample","out_sku_id":"sample","is_discounted":true,"estimate_price":17,"is_change_price":true,"change_price":17,"out_warehouse_id":"sample","sku_deliver_info":{"stock_type":17,"predict_delivery_time":"sample"},"extra_service":{"seven_day_return":17,"freight_insurance":17},"use_deduction":true,"deduction_price":17,"order_product_coupon_info_list":[{"user_coupon_id":"sample","coupon_type":17,"discounted_price":17,"coupon_id":"sample"}],"delivery_deadline":3000000000,"merchant_discounted_price":17,"finder_discounted_price":17,"is_free_gift":true,"vip_discounted_price":17,"product_unique_id":"sample","change_sku_info":{"preshipment_change_sku_state":17,"old_sku_id":"sample","new_sku_id":"sample","ddl_time_stamp":17},"free_gift_info":{"main_product_list":[{"gift_cnt":17,"task_id":17,"product_id":"sample","sku_id":17}]},"bulkbuy_discounted_price":17,"national_subsidy_discounted_price":17,"dropship_info":{"ds_order_id":3000000000},"is_flash_sale":true,"national_subsidy_merchant_discounted_price":17,"platform_activity_merchant_discounted_price":17,"cash_coupon_discounted_price":17}],"pay_info":{"payment_method":17,"pay_time":3000000000,"transaction_id":"sample"},"price_info":{"product_price":17,"order_price":17,"freight":17,"discounted_price":17,"is_discounted":true,"original_order_price":17,"estimate_product_price":17,"change_down_price":17,"change_freight":17,"is_change_freight":true,"use_deduction":true,"deduction_price":17,"merchant_receieve_price":17,"merchant_discounted_price":17,"finder_discounted_price":17,"vip_discounted_price":17,"bulkbuy_discounted_price":17,"national_subsidy_discounted_price":17,"cash_coupon_discounted_price":17,"national_subsidy_merchant_discounted_price":17},"delivery_info":{"address_info":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample","tel_number_ext_info":{"real_tel_number":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":3000000000},"use_tel_number":17,"hash_code":"sample"},"delivery_product_info":[{"waybill_id":"sample","delivery_id":"sample","product_infos":[{"product_id":"sample","sku_id":"sample","product_cnt":17}],"delivery_name":"sample","delivery_time":3000000000,"deliver_type":17,"delivery_address":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample","tel_number_ext_info":{"real_tel_number":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":3000000000},"use_tel_number":17,"hash_code":"sample"}}],"ship_done_time":3000000000,"deliver_method":17,"address_under_review":{"user_name":"sample","tel_number":"sample","postal_code":"sample","province_name":"sample","city_name":"sample","county_name":"sample","detail_info":"sample","national_code":"sample","house_number":"sample","lat":1.5,"lng":1.5,"virtual_order_tel_number":"sample","tel_number_ext_info":{"real_tel_number":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":3000000000},"use_tel_number":17,"hash_code":"sample"},"address_apply_time":3000000000,"ewaybill_order_code":"sample","quality_inspect_type":"sample","quality_inspect_info":{"inspect_status":17},"recharge_info":{"account_no":"sample","account_type":"sample","wx_openid":"sample"}},"coupon_info":{"user_coupon_id":"sample","coupon_type":17,"discounted_price":17,"coupon_id":"sample"},"ext_info":{"customer_notes":"sample","merchant_notes":"sample","confirm_receipt_time":3000000000,"finder_id":"sample","live_id":"sample","order_scene":17},"commission_infos":[{"sku_id":"sample","nickname":"sample","type":17,"status":17,"amount":17,"finder_id":"sample","openfinderid":"sample","talent_id":"sample"}],"sharer_info":{"sharer_openid":"sample","sharer_unionid":"sample","sharer_type":17,"share_scene":17,"handling_progress":17},"settle_info":{"predict_commission_fee":17,"commission_fee":17,"predict_wecoin_commission":17,"wecoin_commission":17,"settle_time":3000000000},"sku_sharer_infos":[{"sharer_openid":"sample","sharer_unionid":"sample","sharer_type":17,"share_scene":17,"sku_id":"sample","from_wecom":true}],"agent_info":{"agent_finder_id":"sample","agent_finder_nickname":"sample"},"source_infos":[{"sku_id":"sample","account_type":17,"account_id":"sample","sale_channel":17,"account_nickname":"sample","content_type":"sample","content_id":"sample","promoter_head_supplier_id":"sample"}],"refund_info":{"sku_id":"sample","account_type":17,"account_id":"sample","sale_channel":17,"account_nickname":"sample","content_type":"sample","content_id":"sample","promoter_head_supplier_id":"sample"},"greeting_card_info":{"giver_name":"sample","receiver_name":"sample","greeting_message":"sample"},"custom_info":{"custom_img_url":"sample","custom_word":"sample","custom_type":17,"custom_preview_img_url":"sample"}},"aftersale_detail":{"on_aftersale_order_cnt":17,"aftersale_order_list":[{"aftersale_order_id":"sample"}]},"is_present":true,"present_order_id_str":"sample","present_note":"sample","present_giver_openid":"sample","present_giver_unionid":"sample","create_time":17,"update_time":17}}"##).unwrap();
    let data: wx_rust_store::bean::order::order_info_response::OrderInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_info_response",
    );
}

#[test]
fn bean_order_order_list_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"page_size":17,"next_key":"sample","create_time_range":{"start_time":3000000000,"end_time":3000000000},"update_time_range":{"start_time":3000000000,"end_time":3000000000},"status":17,"openid":17}"##).unwrap();
    let data: wx_rust_store::bean::order::order_list_param::OrderListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_list_param",
    );
}

#[test]
fn bean_order_order_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","order_id_list":["sample"],"next_key":"sample","has_more":true}"##).unwrap();
    let data: wx_rust_store::bean::order::order_list_response::OrderListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_list_response",
    );
}

#[test]
fn bean_order_order_price_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_price":17,"order_price":17,"freight":17,"discounted_price":17,"is_discounted":true,"original_order_price":17,"estimate_product_price":17,"change_down_price":17,"change_freight":17,"is_change_freight":true,"use_deduction":true,"deduction_price":17,"merchant_receieve_price":17,"merchant_discounted_price":17,"finder_discounted_price":17,"vip_discounted_price":17,"bulkbuy_discounted_price":17,"national_subsidy_discounted_price":17,"cash_coupon_discounted_price":17,"national_subsidy_merchant_discounted_price":17}"##).unwrap();
    let data: wx_rust_store::bean::order::order_price_info::OrderPriceInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_price_info",
    );
}

#[test]
fn bean_order_order_price_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"order_id":"sample","change_express":true,"express_fee":17,"change_order_infos":[{"product_id":"sample","sku_id":"sample","change_price":"sample"}]}"##).unwrap();
    let data: wx_rust_store::bean::order::order_price_param::OrderPriceParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_price_param",
    );
}

#[test]
fn bean_order_order_product_extra_service() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"seven_day_return":17,"freight_insurance":17}"##).unwrap();
    let data: wx_rust_store::bean::order::order_product_extra_service::OrderProductExtraService =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_product_extra_service",
    );
}

#[test]
fn bean_order_order_product_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","sku_id":"sample","thumb_img":"sample","sku_cnt":17,"sale_price":17,"title":"sample","on_aftersale_sku_cnt":17,"finish_aftersale_sku_cnt":17,"sku_code":"sample","market_price":17,"sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"real_price":17,"out_product_id":"sample","out_sku_id":"sample","is_discounted":true,"estimate_price":17,"is_change_price":true,"change_price":17,"out_warehouse_id":"sample","sku_deliver_info":{"stock_type":17,"predict_delivery_time":"sample"},"extra_service":{"seven_day_return":17,"freight_insurance":17},"use_deduction":true,"deduction_price":17,"order_product_coupon_info_list":[{"user_coupon_id":"sample","coupon_type":17,"discounted_price":17,"coupon_id":"sample"}],"delivery_deadline":3000000000,"merchant_discounted_price":17,"finder_discounted_price":17,"is_free_gift":true,"vip_discounted_price":17,"product_unique_id":"sample","change_sku_info":{"preshipment_change_sku_state":17,"old_sku_id":"sample","new_sku_id":"sample","ddl_time_stamp":17},"free_gift_info":{"main_product_list":[{"gift_cnt":17,"task_id":17,"product_id":"sample","sku_id":17}]},"bulkbuy_discounted_price":17,"national_subsidy_discounted_price":17,"dropship_info":{"ds_order_id":3000000000},"is_flash_sale":true,"national_subsidy_merchant_discounted_price":17,"platform_activity_merchant_discounted_price":17,"cash_coupon_discounted_price":17}"##).unwrap();
    let data: wx_rust_store::bean::order::order_product_info::OrderProductInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_product_info",
    );
}

#[test]
fn bean_order_order_refund_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"refund_freight":17}"##).unwrap();
    let data: wx_rust_store::bean::order::order_refund_info::OrderRefundInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_refund_info",
    );
}

#[test]
fn bean_order_order_remark_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"order_id":"sample","merchant_notes":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::order_remark_param::OrderRemarkParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_remark_param",
    );
}

#[test]
fn bean_order_order_search_condition() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"title":"sample","sku_code":"sample","user_name":"sample","tel_number":"sample","tel_number_last4":"sample","order_id":"sample","merchant_notes":"sample","customer_notes":"sample","address_under_review":true}"##).unwrap();
    let data: wx_rust_store::bean::order::order_search_condition::OrderSearchCondition =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_search_condition",
    );
}

#[test]
fn bean_order_order_search_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"page_size":17,"next_key":"sample","search_condition":{"title":"sample","sku_code":"sample","user_name":"sample","tel_number":"sample","tel_number_last4":"sample","order_id":"sample","merchant_notes":"sample","customer_notes":"sample","address_under_review":true},"on_aftersale_order_exist":17,"status":17}"##).unwrap();
    let data: wx_rust_store::bean::order::order_search_param::OrderSearchParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_search_param",
    );
}

#[test]
fn bean_order_order_sharer_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"sharer_openid":"sample","sharer_unionid":"sample","sharer_type":17,"share_scene":17,"handling_progress":17}"##).unwrap();
    let data: wx_rust_store::bean::order::order_sharer_info::OrderSharerInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_sharer_info",
    );
}

#[test]
fn bean_order_order_sku_deliver_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"stock_type":17,"predict_delivery_time":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::order_sku_deliver_info::OrderSkuDeliverInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_sku_deliver_info",
    );
}

#[test]
fn bean_order_order_sku_share_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"sharer_openid":"sample","sharer_unionid":"sample","sharer_type":17,"share_scene":17,"sku_id":"sample","from_wecom":true}"##).unwrap();
    let data: wx_rust_store::bean::order::order_sku_share_info::OrderSkuShareInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_sku_share_info",
    );
}

#[test]
fn bean_order_order_source_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"sku_id":"sample","account_type":17,"account_id":"sample","sale_channel":17,"account_nickname":"sample","content_type":"sample","content_id":"sample","promoter_head_supplier_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::order_source_info::OrderSourceInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_order_source_info",
    );
}

#[test]
fn bean_order_pre_shipment_change_sku_reject_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"order_id":"sample","reject_reason":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::pre_shipment_change_sku_reject_param::PreShipmentChangeSkuRejectParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_pre_shipment_change_sku_reject_param",
    );
}

#[test]
fn bean_order_pre_shipment_change_sku_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","change_sku_info":{"preshipment_change_sku_state":17,"old_sku_id":"sample","new_sku_id":"sample","ddl_time_stamp":17}}"##).unwrap();
    let data: wx_rust_store::bean::order::pre_shipment_change_sku_response::PreShipmentChangeSkuResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_pre_shipment_change_sku_response",
    );
}

#[test]
fn bean_order_present_note_add_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"order_id":"sample","note":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::present_note_add_param::PresentNoteAddParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_present_note_add_param",
    );
}

#[test]
fn bean_order_present_sub_order_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","sub_order_ids":["sample"]}"##)
            .unwrap();
    let data: wx_rust_store::bean::order::present_sub_order_response::PresentSubOrderResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_present_sub_order_response",
    );
}

#[test]
fn bean_order_private_number_add_phone_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"phone":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::private_number_add_phone_param::PrivateNumberAddPhoneParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_private_number_add_phone_param",
    );
}

#[test]
fn bean_order_private_number_get_phone_response() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"errcode":0,"errmsg":"sample","phone_list":[{"phone":"sample","auth_status":17}]}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::order::private_number_get_phone_response::PrivateNumberGetPhoneResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_private_number_get_phone_response",
    );
}

#[test]
fn bean_order_private_number_phone_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"phone":"sample","auth_status":17}"##).unwrap();
    let data: wx_rust_store::bean::order::private_number_phone_info::PrivateNumberPhoneInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_private_number_phone_info",
    );
}

#[test]
fn bean_order_private_number_send_verify_code_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"phone":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::order::private_number_send_verify_code_param::PrivateNumberSendVerifyCodeParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_private_number_send_verify_code_param",
    );
}

#[test]
fn bean_order_quality_insepct_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"inspect_status":17}"##).unwrap();
    let data: wx_rust_store::bean::order::quality_insepct_info::QualityInsepctInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_quality_insepct_info",
    );
}

#[test]
fn bean_order_real_number_view_audit_response() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"errcode":0,"errmsg":"sample","audit_status":17,"real_number":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::order::real_number_view_audit_response::RealNumberViewAuditResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_real_number_view_audit_response",
    );
}

#[test]
fn bean_order_recharge_info() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"account_no":"sample","account_type":"sample","wx_openid":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::order::recharge_info::RechargeInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_recharge_info",
    );
}

#[test]
fn bean_order_tel_number_ext_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"real_tel_number":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::order::tel_number_ext_info::TelNumberExtInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_tel_number_ext_info",
    );
}

#[test]
fn bean_order_virtual_number_info() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"virtual_number":"sample","extension":"sample","expiration":3000000000}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::order::virtual_number_info::VirtualNumberInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_virtual_number_info",
    );
}

#[test]
fn bean_order_virtual_tel_number_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","virtual_tel_number":"sample","virtual_tel_expire_time":3000000000,"get_virtual_tel_cnt":17}"##).unwrap();
    let data: wx_rust_store::bean::order::virtual_tel_number_response::VirtualTelNumberResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_order_virtual_tel_number_response",
    );
}

#[test]
fn bean_product_add_product_third_party_source_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"scene_value":17,"publish_method":17,"supplier":{"probe":"value"},"supplier_shop_performance":{"probe":"value"},"product_source_info":{"probe":"value"}}"##).unwrap();
    let data: wx_rust_store::bean::product::add_product_third_party_source_param::AddProductThirdPartySourceParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_add_product_third_party_source_param",
    );
}

#[test]
fn bean_product_add_product_third_party_source_response() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"errcode":0,"errmsg":"sample","third_party_source_id":3000000000}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::product::add_product_third_party_source_response::AddProductThirdPartySourceResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_add_product_third_party_source_response",
    );
}

#[test]
fn bean_product_after_sale_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"after_sale_address_id":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::product::after_sale_info::AfterSaleInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_after_sale_info",
    );
}

#[test]
fn bean_product_description_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"desc":"sample","imgs":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::product::description_info::DescriptionInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_description_info",
    );
}

#[test]
fn bean_product_express_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"template_id":"sample","weight":17}"##).unwrap();
    let data: wx_rust_store::bean::product::express_info::ExpressInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_express_info",
    );
}

#[test]
fn bean_product_assistant_external_product_mapping_new_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"cat_id":3000000000,"external_category_name":"sample","head_imgs":["sample"],"detail_imgs":["sample"],"title":"sample","external_attributes":[{"key":"sample","value":"sample"}]}"##).unwrap();
    let data: wx_rust_store::bean::product::assistant::external_product_mapping_new_param::ExternalProductMappingNewParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_assistant_external_product_mapping_new_param",
    );
}

#[test]
fn bean_product_assistant_external_product_mapping_new_response() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"errcode":0,"errmsg":"sample","attributes":[{"key":"sample","value":"sample"}]}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::product::assistant::external_product_mapping_new_response::ExternalProductMappingNewResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_assistant_external_product_mapping_new_response",
    );
}

#[test]
fn bean_product_assistant_external_product_mapping_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"cat_id":3000000000,"external_attribute_name":"sample","external_attribute_value":"sample","external_category_name":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::product::assistant::external_product_mapping_param::ExternalProductMappingParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_assistant_external_product_mapping_param",
    );
}

#[test]
fn bean_product_assistant_external_product_mapping_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","external_attribute_name":"sample","external_attribute_value":"sample","internal_attribute_name":"sample","internal_attribute_value":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::product::assistant::external_product_mapping_response::ExternalProductMappingResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_assistant_external_product_mapping_response",
    );
}

#[test]
fn bean_product_extra_service_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"seven_day_return":17,"pay_after_use":17,"freight_insurance":17,"fake_one_pay_three":17,"damage_guarantee":17}"##).unwrap();
    let data: wx_rust_store::bean::product::extra_service_info::ExtraServiceInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_extra_service_info",
    );
}

#[test]
fn bean_product_gift_activity_add_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"gift_activity":{"activity_id":"sample","title":"sample","start_time":3000000000,"end_time":3000000000,"detail":{"show_scene":17,"receive_limit":{"is_limited":true,"limit_num":17},"main_products":[{"product_id":"sample"}],"gift_set":{"gift_items":[{"gift_id":"sample","give_num":17}],"gift_set_num":17}}}}"##).unwrap();
    let data: wx_rust_store::bean::product::gift_activity_add_param::GiftActivityAddParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_gift_activity_add_param",
    );
}

#[test]
fn bean_product_gift_activity_add_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","activity_id":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::product::gift_activity_add_response::GiftActivityAddResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_gift_activity_add_response",
    );
}

#[test]
fn bean_product_gift_activity_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"activity_id":"sample","title":"sample","start_time":3000000000,"end_time":3000000000,"detail":{"show_scene":17,"receive_limit":{"is_limited":true,"limit_num":17},"main_products":[{"product_id":"sample"}],"gift_set":{"gift_items":[{"gift_id":"sample","give_num":17}],"gift_set_num":17}}}"##).unwrap();
    let data: wx_rust_store::bean::product::gift_activity_info::GiftActivityInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_gift_activity_info",
    );
}

#[test]
fn bean_product_gift_product_add_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","product_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::product::gift_product_add_response::GiftProductAddResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_gift_product_add_response",
    );
}

#[test]
fn bean_product_gift_product_get_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","product":{"product_id":"sample","out_product_id":"sample","skus":[{"out_product_id":"sample","out_sku_id":"sample","thumb_img":"sample","sale_price":17,"market_price":17,"stock_num":17,"sku_code":"sample","sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"sku_deliver_info":{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17},"sku_id":"sample","bar_code":"sample"}],"title":"sample","sub_title":"sample","head_imgs":["sample"],"deliver_method":17,"deliver_acct_type":[17],"desc_info":{"desc":"sample","imgs":["sample"]},"cats":[{"cat_id":"sample"}],"cats_v2":[{"cat_id":"sample"}],"attrs":[{"attr_key":"sample","attr_value":"sample"}],"spu_code":"sample","brand_id":"sample","qualifications":["sample"],"express_info":{"template_id":"sample","weight":17},"aftersale_desc":"sample","limited_info":{"period_type":17,"limited_buy_num":17},"extra_service":{"seven_day_return":17,"pay_after_use":17,"freight_insurance":17,"fake_one_pay_three":17,"damage_guarantee":17},"status":17,"edit_status":17,"min_price":17,"create_time":"sample","edit_time":3000000000,"product_type":17,"after_sale_info":{"after_sale_address_id":3000000000},"src_product_id":"sample","product_qua_infos":[{"qua_id":"sample","qua_url":["sample"]}],"size_chart":{"enable":true,"specification_list":[{"name":"sample","unit":"sample","is_range":true,"value_list":[{"key":"sample","value":"sample","left":"sample","right":"sample"}]}]},"short_title":"sample","total_sold_num":17,"release_mode":17,"timing_onsale_info":{"status":17,"onsale_time":3000000000,"is_hide_price":17,"task_id":17},"listing":17},"edit_product":{"product_id":"sample","out_product_id":"sample","skus":[{"out_product_id":"sample","out_sku_id":"sample","thumb_img":"sample","sale_price":17,"market_price":17,"stock_num":17,"sku_code":"sample","sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"sku_deliver_info":{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17},"sku_id":"sample","bar_code":"sample"}],"title":"sample","sub_title":"sample","head_imgs":["sample"],"deliver_method":17,"deliver_acct_type":[17],"desc_info":{"desc":"sample","imgs":["sample"]},"cats":[{"cat_id":"sample"}],"cats_v2":[{"cat_id":"sample"}],"attrs":[{"attr_key":"sample","attr_value":"sample"}],"spu_code":"sample","brand_id":"sample","qualifications":["sample"],"express_info":{"template_id":"sample","weight":17},"aftersale_desc":"sample","limited_info":{"period_type":17,"limited_buy_num":17},"extra_service":{"seven_day_return":17,"pay_after_use":17,"freight_insurance":17,"fake_one_pay_three":17,"damage_guarantee":17},"status":17,"edit_status":17,"min_price":17,"create_time":"sample","edit_time":3000000000,"product_type":17,"after_sale_info":{"after_sale_address_id":3000000000},"src_product_id":"sample","product_qua_infos":[{"qua_id":"sample","qua_url":["sample"]}],"size_chart":{"enable":true,"specification_list":[{"name":"sample","unit":"sample","is_range":true,"value_list":[{"key":"sample","value":"sample","left":"sample","right":"sample"}]}]},"short_title":"sample","total_sold_num":17,"release_mode":17,"timing_onsale_info":{"status":17,"onsale_time":3000000000,"is_hide_price":17,"task_id":17},"listing":17}}"##).unwrap();
    let data: wx_rust_store::bean::product::gift_product_get_response::GiftProductGetResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_gift_product_get_response",
    );
}

#[test]
fn bean_product_gift_product_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","out_product_id":"sample","skus":[{"out_product_id":"sample","out_sku_id":"sample","thumb_img":"sample","sale_price":17,"market_price":17,"stock_num":17,"sku_code":"sample","sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"sku_deliver_info":{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17},"sku_id":"sample","bar_code":"sample"}],"title":"sample","sub_title":"sample","head_imgs":["sample"],"deliver_method":17,"deliver_acct_type":[17],"desc_info":{"desc":"sample","imgs":["sample"]},"cats":[{"cat_id":"sample"}],"cats_v2":[{"cat_id":"sample"}],"attrs":[{"attr_key":"sample","attr_value":"sample"}],"spu_code":"sample","brand_id":"sample","qualifications":["sample"],"express_info":{"template_id":"sample","weight":17},"aftersale_desc":"sample","limited_info":{"period_type":17,"limited_buy_num":17},"extra_service":{"seven_day_return":17,"pay_after_use":17,"freight_insurance":17,"fake_one_pay_three":17,"damage_guarantee":17},"status":17,"edit_status":17,"min_price":17,"create_time":"sample","edit_time":3000000000,"product_type":17,"after_sale_info":{"after_sale_address_id":3000000000},"src_product_id":"sample","product_qua_infos":[{"qua_id":"sample","qua_url":["sample"]}],"size_chart":{"enable":true,"specification_list":[{"name":"sample","unit":"sample","is_range":true,"value_list":[{"key":"sample","value":"sample","left":"sample","right":"sample"}]}]},"short_title":"sample","total_sold_num":17,"release_mode":17,"timing_onsale_info":{"status":17,"onsale_time":3000000000,"is_hide_price":17,"task_id":17},"listing":17}"##).unwrap();
    let data: wx_rust_store::bean::product::gift_product_info::GiftProductInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_gift_product_info",
    );
}

#[test]
fn bean_product_gift_product_list_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"page_size":17,"next_key":"sample","status":17}"##).unwrap();
    let data: wx_rust_store::bean::product::gift_product_list_param::GiftProductListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_gift_product_list_param",
    );
}

#[test]
fn bean_product_gift_product_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","total_num":17,"next_key":"sample","product_ids":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::product::gift_product_list_response::GiftProductListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_gift_product_list_response",
    );
}

#[test]
fn bean_product_limit_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"period_type":17,"limited_buy_num":17}"##).unwrap();
    let data: wx_rust_store::bean::product::limit_info::LimitInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_limit_info",
    );
}

#[test]
fn bean_product_product_audit_quota_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","audit_quota":{"block_status":17,"avail_quota":17,"total_quota":17,"unlimited_type":17,"audit_total_quota":17,"audit_total_remaining":17,"new_product_total_quota":17,"new_product_remaining":17}}"##).unwrap();
    let data: wx_rust_store::bean::product::product_audit_quota_response::ProductAuditQuotaResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_product_audit_quota_response",
    );
}

#[test]
fn bean_product_product_audit_strategy_info() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"hide_err_field_flag":17,"hit_duplicated_flag":17,"hit_low_risk_rule_flag":17}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::product::product_audit_strategy_info::ProductAuditStrategyInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_product_audit_strategy_info",
    );
}

#[test]
fn bean_product_product_audit_strategy_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","audit_strategy":{"hide_err_field_flag":17,"hit_duplicated_flag":17,"hit_low_risk_rule_flag":17}}"##).unwrap();
    let data: wx_rust_store::bean::product::product_audit_strategy_response::ProductAuditStrategyResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_product_audit_strategy_response",
    );
}

#[test]
fn bean_product_product_audit_strategy_set_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"audit_strategy":{"hide_err_field_flag":17,"hit_duplicated_flag":17,"hit_low_risk_rule_flag":17}}"##).unwrap();
    let data: wx_rust_store::bean::product::product_audit_strategy_set_param::ProductAuditStrategySetParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_product_audit_strategy_set_param",
    );
}

#[test]
fn bean_product_assistant_product_brand_recommend_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"cat_id":3000000000,"head_imgs":["sample"],"detail_imgs":["sample"],"title":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::product::assistant::product_brand_recommend_param::ProductBrandRecommendParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_assistant_product_brand_recommend_param",
    );
}

#[test]
fn bean_product_assistant_product_brand_recommend_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","brand_id":3000000000,"brand_name_chinese":"sample","brand_name_english":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::product::assistant::product_brand_recommend_response::ProductBrandRecommendResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_assistant_product_brand_recommend_response",
    );
}

#[test]
fn bean_product_product_category_classify_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"req_type":17,"title":"sample","head_imgs":["sample"],"cat_id":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::product::product_category_classify_param::ProductCategoryClassifyParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_product_category_classify_param",
    );
}

#[test]
fn bean_product_product_category_classify_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","categories":[{"cats":[{"cat_info":{"cat_id":"sample","cat_name":"sample","is_shop_no_audit":true},"has_permission":true}]}],"wrong_cat":true}"##).unwrap();
    let data: wx_rust_store::bean::product::product_category_classify_response::ProductCategoryClassifyResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_product_category_classify_response",
    );
}

#[test]
fn bean_product_product_qua_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"qua_id":"sample","qua_url":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::product::product_qua_info::ProductQuaInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_product_qua_info",
    );
}

#[test]
fn bean_product_product_sale_limit_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"is_limited":17,"title":"sample","sub_title":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::product::product_sale_limit_info::ProductSaleLimitInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_product_sale_limit_info",
    );
}

#[test]
fn bean_product_product_scheme_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"product_id":"sample","from_appid":"sample","expire":17,"ext_info":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::product::product_scheme_param::ProductSchemeParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_product_scheme_param",
    );
}

#[test]
fn bean_product_product_scheme_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","openlink":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::product::product_scheme_response::ProductSchemeResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_product_scheme_response",
    );
}

#[test]
fn bean_product_sku_deliver_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17}"##).unwrap();
    let data: wx_rust_store::bean::product::sku_deliver_info::SkuDeliverInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_sku_deliver_info",
    );
}

#[test]
fn bean_product_sku_fast_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"sku_id":"sample","sale_price":17,"stock_info":{"diff_type":17,"num":17},"sku_deliver_info":{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17},"is_delete":true,"sku_code":"sample","status":17}"##).unwrap();
    let data: wx_rust_store::bean::product::sku_fast_info::SkuFastInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_sku_fast_info",
    );
}

#[test]
fn bean_product_sku_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"out_product_id":"sample","out_sku_id":"sample","thumb_img":"sample","sale_price":17,"market_price":17,"stock_num":17,"sku_code":"sample","sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"sku_deliver_info":{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17},"sku_id":"sample","bar_code":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::product::sku_info::SkuInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_sku_info",
    );
}

#[test]
fn bean_product_sku_stock_batch_list() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"spu_stock_list":[{"product_id":"sample","sku_stock":[{"normal_stock_num":17,"limited_discount_stock_num":17,"warehouse_stocks":[{"out_warehouse_id":"sample","num":17,"lock_stock":17}],"total_stock_num":17,"finder_stock_num":17}]}]}"##).unwrap();
    let data: wx_rust_store::bean::product::sku_stock_batch_list::SkuStockBatchList =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_sku_stock_batch_list",
    );
}

#[test]
fn bean_product_sku_stock_batch_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"product_id":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::product::sku_stock_batch_param::SkuStockBatchParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_sku_stock_batch_param",
    );
}

#[test]
fn bean_product_sku_stock_batch_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"spu_stock_list":[{"product_id":"sample","sku_stock":[{"normal_stock_num":17,"limited_discount_stock_num":17,"warehouse_stocks":[{"out_warehouse_id":"sample","num":17,"lock_stock":17}],"total_stock_num":17,"finder_stock_num":17}]}]}}"##).unwrap();
    let data: wx_rust_store::bean::product::sku_stock_batch_response::SkuStockBatchResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_sku_stock_batch_response",
    );
}

#[test]
fn bean_product_sku_stock_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"normal_stock_num":17,"limited_discount_stock_num":17,"warehouse_stocks":[{"out_warehouse_id":"sample","num":17,"lock_stock":17}],"total_stock_num":17,"finder_stock_num":17}"##).unwrap();
    let data: wx_rust_store::bean::product::sku_stock_info::SkuStockInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_sku_stock_info",
    );
}

#[test]
fn bean_product_sku_stock_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"product_id":"sample","sku_id":"sample","diff_type":17,"num":17}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::product::sku_stock_param::SkuStockParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_sku_stock_param",
    );
}

#[test]
fn bean_product_sku_stock_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"normal_stock_num":17,"limited_discount_stock_num":17,"warehouse_stocks":[{"out_warehouse_id":"sample","num":17,"lock_stock":17}],"total_stock_num":17,"finder_stock_num":17}}"##).unwrap();
    let data: wx_rust_store::bean::product::sku_stock_response::SkuStockResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_sku_stock_response",
    );
}

#[test]
fn bean_product_spu_category() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"cat_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::product::spu_category::SpuCategory =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_spu_category",
    );
}

#[test]
fn bean_product_spu_fast_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","skus":[{"sku_id":"sample","sale_price":17,"stock_info":{"diff_type":17,"num":17},"sku_deliver_info":{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17},"is_delete":true,"sku_code":"sample","status":17}],"spu_code":"sample","limit_info":{"period_type":17,"limited_buy_num":17},"express_info":{"template_id":"sample","weight":17},"extra_service":{"seven_day_return":17,"pay_after_use":17,"freight_insurance":17,"fake_one_pay_three":17,"damage_guarantee":17},"deliver_method":17,"timing_onsale_info":{"status":17,"onsale_time":3000000000,"is_hide_price":17,"task_id":17}}"##).unwrap();
    let data: wx_rust_store::bean::product::spu_fast_info::SpuFastInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_spu_fast_info",
    );
}

#[test]
fn bean_product_spu_get_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","product":{"product_id":"sample","out_product_id":"sample","skus":[{"out_product_id":"sample","out_sku_id":"sample","thumb_img":"sample","sale_price":17,"market_price":17,"stock_num":17,"sku_code":"sample","sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"sku_deliver_info":{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17},"sku_id":"sample","bar_code":"sample"}],"title":"sample","sub_title":"sample","head_imgs":["sample"],"deliver_method":17,"deliver_acct_type":[17],"desc_info":{"desc":"sample","imgs":["sample"]},"cats":[{"cat_id":"sample"}],"cats_v2":[{"cat_id":"sample"}],"attrs":[{"attr_key":"sample","attr_value":"sample"}],"spu_code":"sample","brand_id":"sample","qualifications":["sample"],"express_info":{"template_id":"sample","weight":17},"aftersale_desc":"sample","limited_info":{"period_type":17,"limited_buy_num":17},"extra_service":{"seven_day_return":17,"pay_after_use":17,"freight_insurance":17,"fake_one_pay_three":17,"damage_guarantee":17},"status":17,"edit_status":17,"min_price":17,"create_time":"sample","edit_time":3000000000,"product_type":17,"after_sale_info":{"after_sale_address_id":3000000000},"src_product_id":"sample","product_qua_infos":[{"qua_id":"sample","qua_url":["sample"]}],"size_chart":{"enable":true,"specification_list":[{"name":"sample","unit":"sample","is_range":true,"value_list":[{"key":"sample","value":"sample","left":"sample","right":"sample"}]}]},"short_title":"sample","total_sold_num":17,"release_mode":17,"timing_onsale_info":{"status":17,"onsale_time":3000000000,"is_hide_price":17,"task_id":17}},"edit_product":{"product_id":"sample","out_product_id":"sample","skus":[{"out_product_id":"sample","out_sku_id":"sample","thumb_img":"sample","sale_price":17,"market_price":17,"stock_num":17,"sku_code":"sample","sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"sku_deliver_info":{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17},"sku_id":"sample","bar_code":"sample"}],"title":"sample","sub_title":"sample","head_imgs":["sample"],"deliver_method":17,"deliver_acct_type":[17],"desc_info":{"desc":"sample","imgs":["sample"]},"cats":[{"cat_id":"sample"}],"cats_v2":[{"cat_id":"sample"}],"attrs":[{"attr_key":"sample","attr_value":"sample"}],"spu_code":"sample","brand_id":"sample","qualifications":["sample"],"express_info":{"template_id":"sample","weight":17},"aftersale_desc":"sample","limited_info":{"period_type":17,"limited_buy_num":17},"extra_service":{"seven_day_return":17,"pay_after_use":17,"freight_insurance":17,"fake_one_pay_three":17,"damage_guarantee":17},"status":17,"edit_status":17,"min_price":17,"create_time":"sample","edit_time":3000000000,"product_type":17,"after_sale_info":{"after_sale_address_id":3000000000},"src_product_id":"sample","product_qua_infos":[{"qua_id":"sample","qua_url":["sample"]}],"size_chart":{"enable":true,"specification_list":[{"name":"sample","unit":"sample","is_range":true,"value_list":[{"key":"sample","value":"sample","left":"sample","right":"sample"}]}]},"short_title":"sample","total_sold_num":17,"release_mode":17,"timing_onsale_info":{"status":17,"onsale_time":3000000000,"is_hide_price":17,"task_id":17}},"sale_limit_info":{"is_limited":17,"title":"sample","sub_title":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::product::spu_get_response::SpuGetResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_spu_get_response",
    );
}

#[test]
fn bean_product_spu_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","out_product_id":"sample","skus":[{"out_product_id":"sample","out_sku_id":"sample","thumb_img":"sample","sale_price":17,"market_price":17,"stock_num":17,"sku_code":"sample","sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"sku_deliver_info":{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17},"sku_id":"sample","bar_code":"sample"}],"title":"sample","sub_title":"sample","head_imgs":["sample"],"deliver_method":17,"deliver_acct_type":[17],"desc_info":{"desc":"sample","imgs":["sample"]},"cats":[{"cat_id":"sample"}],"cats_v2":[{"cat_id":"sample"}],"attrs":[{"attr_key":"sample","attr_value":"sample"}],"spu_code":"sample","brand_id":"sample","qualifications":["sample"],"express_info":{"template_id":"sample","weight":17},"aftersale_desc":"sample","limited_info":{"period_type":17,"limited_buy_num":17},"extra_service":{"seven_day_return":17,"pay_after_use":17,"freight_insurance":17,"fake_one_pay_three":17,"damage_guarantee":17},"status":17,"edit_status":17,"min_price":17,"create_time":"sample","edit_time":3000000000,"product_type":17,"after_sale_info":{"after_sale_address_id":3000000000},"src_product_id":"sample","product_qua_infos":[{"qua_id":"sample","qua_url":["sample"]}],"size_chart":{"enable":true,"specification_list":[{"name":"sample","unit":"sample","is_range":true,"value_list":[{"key":"sample","value":"sample","left":"sample","right":"sample"}]}]},"short_title":"sample","total_sold_num":17,"release_mode":17,"timing_onsale_info":{"status":17,"onsale_time":3000000000,"is_hide_price":17,"task_id":17}}"##).unwrap();
    let data: wx_rust_store::bean::product::spu_info::SpuInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_spu_info",
    );
}

#[test]
fn bean_product_spu_list_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"page_size":17,"next_key":"sample","status":17}"##).unwrap();
    let data: wx_rust_store::bean::product::spu_list_param::SpuListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_spu_list_param",
    );
}

#[test]
fn bean_product_spu_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","total_num":17,"next_key":"sample","product_ids":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::product::spu_list_response::SpuListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_spu_list_response",
    );
}

#[test]
fn bean_product_spu_simple_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","out_product_id":"sample","skus":[{"out_product_id":"sample","out_sku_id":"sample","thumb_img":"sample","sale_price":17,"market_price":17,"stock_num":17,"sku_code":"sample","sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"sku_deliver_info":{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17},"sku_id":"sample","bar_code":"sample"}]}"##).unwrap();
    let data: wx_rust_store::bean::product::spu_simple_info::SpuSimpleInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_spu_simple_info",
    );
}

#[test]
fn bean_product_spu_size_chart() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"enable":true,"specification_list":[{"name":"sample","unit":"sample","is_range":true,"value_list":[{"key":"sample","value":"sample","left":"sample","right":"sample"}]}]}"##).unwrap();
    let data: wx_rust_store::bean::product::spu_size_chart::SpuSizeChart =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_spu_size_chart",
    );
}

#[test]
fn bean_product_spu_size_chart_item() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"name":"sample","unit":"sample","is_range":true,"value_list":[{"key":"sample","value":"sample","left":"sample","right":"sample"}]}"##).unwrap();
    let data: wx_rust_store::bean::product::spu_size_chart_item::SpuSizeChartItem =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_spu_size_chart_item",
    );
}

#[test]
fn bean_product_spu_stock_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","sku_stock":[{"normal_stock_num":17,"limited_discount_stock_num":17,"warehouse_stocks":[{"out_warehouse_id":"sample","num":17,"lock_stock":17}],"total_stock_num":17,"finder_stock_num":17}]}"##).unwrap();
    let data: wx_rust_store::bean::product::spu_stock_info::SpuStockInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_spu_stock_info",
    );
}

#[test]
fn bean_product_spu_update_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","out_product_id":"sample","skus":[{"out_product_id":"sample","out_sku_id":"sample","thumb_img":"sample","sale_price":17,"market_price":17,"stock_num":17,"sku_code":"sample","sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"sku_deliver_info":{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17},"sku_id":"sample","bar_code":"sample"}],"title":"sample","sub_title":"sample","head_imgs":["sample"],"deliver_method":17,"deliver_acct_type":[17],"desc_info":{"desc":"sample","imgs":["sample"]},"cats":[{"cat_id":"sample"}],"cats_v2":[{"cat_id":"sample"}],"attrs":[{"attr_key":"sample","attr_value":"sample"}],"spu_code":"sample","brand_id":"sample","qualifications":["sample"],"express_info":{"template_id":"sample","weight":17},"aftersale_desc":"sample","limited_info":{"period_type":17,"limited_buy_num":17},"extra_service":{"seven_day_return":17,"pay_after_use":17,"freight_insurance":17,"fake_one_pay_three":17,"damage_guarantee":17},"status":17,"edit_status":17,"min_price":17,"create_time":"sample","edit_time":3000000000,"product_type":17,"after_sale_info":{"after_sale_address_id":3000000000},"src_product_id":"sample","product_qua_infos":[{"qua_id":"sample","qua_url":["sample"]}],"size_chart":{"enable":true,"specification_list":[{"name":"sample","unit":"sample","is_range":true,"value_list":[{"key":"sample","value":"sample","left":"sample","right":"sample"}]}]},"short_title":"sample","total_sold_num":17,"release_mode":17,"timing_onsale_info":{"status":17,"onsale_time":3000000000,"is_hide_price":17,"task_id":17},"listing":17}"##).unwrap();
    let data: wx_rust_store::bean::product::spu_update_info::SpuUpdateInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_spu_update_info",
    );
}

#[test]
fn bean_product_spu_update_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"product_id":"sample","out_product_id":"sample","skus":[{"out_product_id":"sample","out_sku_id":"sample","thumb_img":"sample","sale_price":17,"market_price":17,"stock_num":17,"sku_code":"sample","sku_attrs":[{"attr_key":"sample","attr_value":"sample"}],"sku_deliver_info":{"stock_type":17,"full_payment_presale_delivery_type":17,"presale_begin_time":3000000000,"presale_end_time":3000000000,"full_payment_presale_delivery_time":17},"sku_id":"sample","bar_code":"sample"}],"title":"sample","sub_title":"sample","head_imgs":["sample"],"deliver_method":17,"deliver_acct_type":[17],"desc_info":{"desc":"sample","imgs":["sample"]},"cats":[{"cat_id":"sample"}],"cats_v2":[{"cat_id":"sample"}],"attrs":[{"attr_key":"sample","attr_value":"sample"}],"spu_code":"sample","brand_id":"sample","qualifications":["sample"],"express_info":{"template_id":"sample","weight":17},"aftersale_desc":"sample","limited_info":{"period_type":17,"limited_buy_num":17},"extra_service":{"seven_day_return":17,"pay_after_use":17,"freight_insurance":17,"fake_one_pay_three":17,"damage_guarantee":17},"status":17,"edit_status":17,"min_price":17,"create_time":"sample","edit_time":3000000000,"product_type":17,"after_sale_info":{"after_sale_address_id":3000000000},"src_product_id":"sample","product_qua_infos":[{"qua_id":"sample","qua_url":["sample"]}],"size_chart":{"enable":true,"specification_list":[{"name":"sample","unit":"sample","is_range":true,"value_list":[{"key":"sample","value":"sample","left":"sample","right":"sample"}]}]},"short_title":"sample","total_sold_num":17,"release_mode":17,"timing_onsale_info":{"status":17,"onsale_time":3000000000,"is_hide_price":17,"task_id":17}}}"##).unwrap();
    let data: wx_rust_store::bean::product::spu_update_response::SpuUpdateResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_spu_update_response",
    );
}

#[test]
fn bean_product_timing_on_sale_info() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"status":17,"onsale_time":3000000000,"is_hide_price":17,"task_id":17}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::product::timing_on_sale_info::TimingOnSaleInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_timing_on_sale_info",
    );
}

#[test]
fn bean_product_warehouse_stock_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"out_warehouse_id":"sample","num":17,"lock_stock":17}"##)
            .unwrap();
    let data: wx_rust_store::bean::product::warehouse_stock_info::WarehouseStockInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_warehouse_stock_info",
    );
}

#[test]
fn bean_product_assistant_begin_timing_sale_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"product_id":"sample","task_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::product::assistant::begin_timing_sale_param::BeginTimingSaleParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_assistant_begin_timing_sale_param",
    );
}

#[test]
fn bean_product_assistant_cancel_timing_sale_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::product::assistant::cancel_timing_sale_param::CancelTimingSaleParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_assistant_cancel_timing_sale_param",
    );
}

#[test]
fn bean_product_assistant_category_pre_check_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"cat_id":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::product::assistant::category_pre_check_param::CategoryPreCheckParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_assistant_category_pre_check_param",
    );
}

#[test]
fn bean_product_assistant_category_pre_check_response() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"errcode":0,"errmsg":"sample","all_pass":true,"fail_reasons":["sample"]}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::product::assistant::category_pre_check_response::CategoryPreCheckResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_assistant_category_pre_check_response",
    );
}

#[test]
fn bean_product_link_product_h5_url_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","product_h5url":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::product::link::product_h5_url_response::ProductH5UrlResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_link_product_h5_url_response",
    );
}

#[test]
fn bean_product_link_product_qr_code_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","product_qrcode":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::product::link::product_qr_code_response::ProductQrCodeResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_link_product_qr_code_response",
    );
}

#[test]
fn bean_product_link_product_tag_link_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","product_taglink":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::product::link::product_tag_link_response::ProductTagLinkResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_link_product_tag_link_response",
    );
}

#[test]
fn bean_product_stock_stock_flow_ext_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"unmove_from_stock_sub_type":17,"move_to_stock_sub_type":17,"upload_source":17,"order_id":"sample","out_warehouse_id":"sample","limited_discount_id":"sample","finder_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::product::stock::stock_flow_ext_info::StockFlowExtInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_stock_stock_flow_ext_info",
    );
}

#[test]
fn bean_product_stock_stock_flow_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"amount":17,"beginning_amount":17,"ending_amount":17,"stock_sub_type":17,"op_type":17,"update_time":3000000000,"ext_info":{"unmove_from_stock_sub_type":17,"move_to_stock_sub_type":17,"upload_source":17,"order_id":"sample","out_warehouse_id":"sample","limited_discount_id":"sample","finder_id":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::product::stock::stock_flow_info::StockFlowInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_stock_stock_flow_info",
    );
}

#[test]
fn bean_product_stock_stock_flow_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","sku_id":"sample","stock_type":17,"finder_id":"sample","begin_time":3000000000,"end_time":3000000000,"op_type_list":[17],"page_size":17,"next_key":"sample","stock_type_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::product::stock::stock_flow_param::StockFlowParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_product_stock_stock_flow_param",
    );
}

#[test]
fn bean_qic_inspect_code_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"backupDeliveryId":"sample","backupDeliveryName":"sample","boxDTOList":[{"boxId":3000000000,"boxName":"sample","boxNum":17}],"channelAppId":"sample","deliveryId":"sample","deliveryName":"sample","embedGoodsMaterial":"sample","goodsDesc":"sample","expressMerge":true,"goodsMainMaterial":"sample","goodsName":"sample","goodsNum":17,"goodsPartsMaterial":"sample","inspectBaseId":"sample","inspectBaseName":"sample","inspectCode":"sample","inspectOrgId":"sample","inspectOrgName":"sample","inspectOrgShortName":"sample","merchantName":"sample","orderId":"sample","urgentOrder":true,"printInfo":"sample","needLabel":true}}"##).unwrap();
    let data: wx_rust_store::bean::qic::inspect_code_response::InspectCodeResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_qic_inspect_code_response",
    );
}

#[test]
fn bean_qic_inspect_config_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","inspect_config":{"warehouse_id":"sample","delivery_address":{"contact_name":"sample","contact_phone":"sample","province":"sample","city":"sample","county":"sample","detail":"sample"},"return_address":{"contact_name":"sample","contact_phone":"sample","province":"sample","city":"sample","county":"sample","detail":"sample"},"warehouse_name":"sample","warehouse_addr":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::qic::inspect_config_response::InspectConfigResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_qic_inspect_config_response",
    );
}

#[test]
fn bean_qic_register_logistics_request() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"order_id_list":["sample"],"logistics_info":{"waybill_id":"sample","delivery_id":"sample","delivery_name":"sample","delivery_type":17}}"##).unwrap();
    let data: wx_rust_store::bean::qic::register_logistics_request::RegisterLogisticsRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_qic_register_logistics_request",
    );
}

#[test]
fn bean_qic_submit_config_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","submit_config":{"delivery_list":[{"id":"sample","name":"sample","delivery_products":[{"id":3000000000,"name":"sample","enable_insure":17,"insure_type_list":[{"id":"sample","name":"sample","upper_limit_type":17,"upper_limit_amount":3000000000}]}]}],"inspect_org_list":[{"id":"sample","name":"sample","org_category":17}],"charge_url":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::qic::submit_config_response::SubmitConfigResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_qic_submit_config_response",
    );
}

#[test]
fn bean_qic_submit_inspect_request() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"order_id":"sample","inspect_info":{"delivery_id":"sample","backup_delivery_id":"sample","express_insure":true,"express_insure_amount":3000000000,"express_merge":true,"inspect_org_id":"sample","refund_intercept":17,"inspect_org_name":"sample","warehouse_name":"sample","warehouse_addr":"sample","delivery_product_id":3000000000,"delivery_insure_id":"sample","backup_delivery_product_id":3000000000,"backup_delivery_insure_id":"sample","backup_express_insure":true,"backup_express_insure_amount":3000000000,"remark":"sample","agarwood_inspect_org_id":"sample","agarwood_inspect_org_name":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::qic::submit_inspect_request::SubmitInspectRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_qic_submit_inspect_request",
    );
}

#[test]
fn bean_sharer_finder_scene_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"promoter_id":"sample","finder_nickname":"sample","live_export_id":"sample","video_export_id":"sample","video_title":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::sharer::finder_scene_info::FinderSceneInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_sharer_finder_scene_info",
    );
}

#[test]
fn bean_sharer_sharer_bind_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","qrcode_img_base64":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::sharer::sharer_bind_response::SharerBindResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_sharer_sharer_bind_response",
    );
}

#[test]
fn bean_sharer_sharer_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"openid":"sample","unionid":"sample","nickname":"sample","bind_time":3000000000,"sharer_type":17}"##).unwrap();
    let data: wx_rust_store::bean::sharer::sharer_info::SharerInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_sharer_sharer_info",
    );
}

#[test]
fn bean_sharer_sharer_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","sharer_info_list":[{"openid":"sample","unionid":"sample","nickname":"sample","bind_time":3000000000,"sharer_type":17}]}"##).unwrap();
    let data: wx_rust_store::bean::sharer::sharer_info_response::SharerInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_sharer_sharer_info_response",
    );
}

#[test]
fn bean_sharer_sharer_list_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"page":17,"page_size":17,"sharer_type":17}"##).unwrap();
    let data: wx_rust_store::bean::sharer::sharer_list_param::SharerListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_sharer_sharer_list_param",
    );
}

#[test]
fn bean_sharer_sharer_order() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"order_id":"sample","share_scene":17,"sharer_openid":"sample","sharer_type":17,"sku_id":"sample","product_id":"sample","from_wecom":true,"finder_scene_info":{"promoter_id":"sample","finder_nickname":"sample","live_export_id":"sample","video_export_id":"sample","video_title":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::sharer::sharer_order::SharerOrder =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_sharer_sharer_order",
    );
}

#[test]
fn bean_sharer_sharer_order_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"page":17,"page_size":17,"openid":"sample","share_scene":17,"start_time":3000000000,"end_time":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::sharer::sharer_order_param::SharerOrderParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_sharer_sharer_order_param",
    );
}

#[test]
fn bean_sharer_sharer_order_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","order_list":[{"order_id":"sample","share_scene":17,"sharer_openid":"sample","sharer_type":17,"sku_id":"sample","product_id":"sample","from_wecom":true,"finder_scene_info":{"promoter_id":"sample","finder_nickname":"sample","live_export_id":"sample","video_export_id":"sample","video_title":"sample"}}]}"##).unwrap();
    let data: wx_rust_store::bean::sharer::sharer_order_response::SharerOrderResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_sharer_sharer_order_response",
    );
}

#[test]
fn bean_sharer_sharer_search_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"openid":"sample","username":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::sharer::sharer_search_param::SharerSearchParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_sharer_sharer_search_param",
    );
}

#[test]
fn bean_sharer_sharer_search_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","openid":"sample","unionid":"sample","nickname":"sample","bind_time":3000000000,"sharer_type":17}"##).unwrap();
    let data: wx_rust_store::bean::sharer::sharer_search_response::SharerSearchResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_sharer_sharer_search_response",
    );
}

#[test]
fn bean_sharer_sharer_unbind_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"openid_list":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::sharer::sharer_unbind_param::SharerUnbindParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_sharer_sharer_unbind_param",
    );
}

#[test]
fn bean_sharer_sharer_unbind_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","success_openid":["sample"],"fail_openid":["sample"],"refuse_openid":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::sharer::sharer_unbind_response::SharerUnbindResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_sharer_sharer_unbind_response",
    );
}

#[test]
fn bean_shop_shop_h5_url_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","shop_h5url":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::shop::shop_h5_url_response::ShopH5UrlResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_shop_shop_h5_url_response",
    );
}

#[test]
fn bean_shop_shop_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"nickname":"sample","headimg_url":"sample","subject_type":"sample","status":"sample","username":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::shop::shop_info::ShopInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_shop_shop_info",
    );
}

#[test]
fn bean_shop_shop_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","info":{"nickname":"sample","headimg_url":"sample","subject_type":"sample","status":"sample","username":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::shop::shop_info_response::ShopInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_shop_shop_info_response",
    );
}

#[test]
fn bean_shop_shop_qr_code_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","shop_qrcode":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::shop::shop_qr_code_response::ShopQrCodeResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_shop_shop_qr_code_response",
    );
}

#[test]
fn bean_shop_shop_tag_link_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","shop_taglink":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::shop::shop_tag_link_response::ShopTagLinkResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_shop_shop_tag_link_response",
    );
}

#[test]
fn bean_supplier_distribute_type_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","distribute_type":17,"supplier_info":{"supplier_id":"sample","supplier_name":"sample","status":17}}"##).unwrap();
    let data: wx_rust_store::bean::supplier::distribute_type_response::DistributeTypeResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_distribute_type_response",
    );
}

#[test]
fn bean_supplier_dropship_assign_request() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"order_id":"sample","supplier_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::supplier::dropship_assign_request::DropshipAssignRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_dropship_assign_request",
    );
}

#[test]
fn bean_supplier_dropship_detail_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","dropship_info":{"order_id":"sample","supplier_id":"sample","ds_order_id":"sample","status":17,"create_time":3000000000,"update_time":3000000000}}"##).unwrap();
    let data: wx_rust_store::bean::supplier::dropship_detail_response::DropshipDetailResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_dropship_detail_response",
    );
}

#[test]
fn bean_supplier_dropship_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"order_id":"sample","supplier_id":"sample","ds_order_id":"sample","status":17,"create_time":3000000000,"update_time":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::supplier::dropship_info::DropshipInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_dropship_info",
    );
}

#[test]
fn bean_supplier_dropship_list_request() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"supplier_id":"sample","status":17,"create_time_start":3000000000,"create_time_end":3000000000,"page_size":17,"next_key":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::supplier::dropship_list_request::DropshipListRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_dropship_list_request",
    );
}

#[test]
fn bean_supplier_dropship_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","dropship_list":[{"order_id":"sample","supplier_id":"sample","ds_order_id":"sample","status":17,"create_time":3000000000,"update_time":3000000000}],"next_key":"sample","has_more":true}"##).unwrap();
    let data: wx_rust_store::bean::supplier::dropship_list_response::DropshipListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_dropship_list_response",
    );
}

#[test]
fn bean_supplier_dropship_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","order_id":"sample","supplier_id":"sample","dropship_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::supplier::dropship_response::DropshipResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_dropship_response",
    );
}

#[test]
fn bean_supplier_dropship_search_request() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"supplier_id":"sample","status":17,"create_time_start":3000000000,"create_time_end":3000000000,"page_size":17,"next_key":"sample","order_id":"sample","dropship_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::supplier::dropship_search_request::DropshipSearchRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_dropship_search_request",
    );
}

#[test]
fn bean_supplier_product_distribute_request() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"supplier_id":"sample","product_id_list":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::supplier::product_distribute_request::ProductDistributeRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_product_distribute_request",
    );
}

#[test]
fn bean_supplier_product_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","product_list":[{"product_id":"sample","supplier_id":"sample"}],"next_key":"sample","has_more":true}"##).unwrap();
    let data: wx_rust_store::bean::supplier::product_list_response::ProductListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_product_list_response",
    );
}

#[test]
fn bean_supplier_supplier_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"supplier_id":"sample","supplier_name":"sample","status":17}"##)
            .unwrap();
    let data: wx_rust_store::bean::supplier::supplier_info::SupplierInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_supplier_info",
    );
}

#[test]
fn bean_supplier_supplier_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","supplier_info":{"supplier_id":"sample","supplier_name":"sample","status":17}}"##).unwrap();
    let data: wx_rust_store::bean::supplier::supplier_info_response::SupplierInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_supplier_info_response",
    );
}

#[test]
fn bean_supplier_supplier_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","supplier_list":[{"supplier_id":"sample","supplier_name":"sample","status":17}],"next_key":"sample","has_more":true}"##).unwrap();
    let data: wx_rust_store::bean::supplier::supplier_list_response::SupplierListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_supplier_supplier_list_response",
    );
}

#[test]
fn bean_talent_talent_order_detail_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"order_id":"sample","sku_id":"sample","special_id":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::talent::talent_order_detail_param::TalentOrderDetailParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_talent_talent_order_detail_param",
    );
}

#[test]
fn bean_talent_talent_order_detail_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","base_info":{"order_id":"sample","spu_id":"sample","sku_id":"sample","special_id":"sample","order_status":17,"actual_payment":"sample","order_create_time":3000000000,"order_update_time":3000000000,"buyer_info":{"open_id":"sample","union_id":"sample"},"order_pay_time":3000000000,"settle_payment":"sample"},"commission_info":{"state":17,"ratio":"sample","expect_settle_time":3000000000,"expect_settlement":"sample","actual_settle_time":3000000000,"actual_settlement":"sample"},"channel_info":{"channel_type":17,"channel_id":"sample","channel_name":"sample"},"promotion_head_supplier_info":{"id":"sample","name":"sample","ratio":"sample","fee":"sample"},"product_info":{"title":"sample","thumb_img":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::talent::talent_order_detail_response::TalentOrderDetailResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_talent_talent_order_detail_response",
    );
}

#[test]
fn bean_talent_talent_order_list_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"create_time_gt":3000000000,"create_time_lt":3000000000,"order_id":"sample","spu_id":"sample","update_time_gt":3000000000,"update_time_lt":3000000000,"page_size":17,"next_key":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::talent::talent_order_list_param::TalentOrderListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_talent_talent_order_list_param",
    );
}

#[test]
fn bean_talent_talent_order_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","order_list":[{"order_id":"sample","sku_id":"sample","special_id":"sample"}],"has_more":true,"next_key":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::talent::talent_order_list_response::TalentOrderListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_talent_talent_order_list_response",
    );
}

#[test]
fn bean_talent_talent_window_product_detail_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::talent::talent_window_product_detail_param::TalentWindowProductDetailParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_talent_talent_window_product_detail_param",
    );
}

#[test]
fn bean_talent_talent_window_product_detail_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","product":{"product_id":"sample","appid":"sample","out_product_id":"sample","title":"sample","img_url":"sample","leaf_category_id":3000000000,"status":17,"selling_price":3000000000,"stock":3000000000,"sales":3000000000,"is_hide":true,"product_promotion_link":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::talent::talent_window_product_detail_response::TalentWindowProductDetailResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_talent_talent_window_product_detail_response",
    );
}

#[test]
fn bean_talent_talent_window_product_list_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"page_size":17,"page_index":17,"last_buffer":"sample"}"##)
            .unwrap();
    let data: wx_rust_store::bean::talent::talent_window_product_list_param::TalentWindowProductListParam = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_talent_talent_window_product_list_param",
    );
}

#[test]
fn bean_talent_talent_window_product_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","products":[{"product_id":"sample","appid":"sample","product_source":17,"out_product_id":"sample"}],"last_buffer":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::talent::talent_window_product_list_response::TalentWindowProductListResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_talent_talent_window_product_list_response",
    );
}

#[test]
fn bean_token_stable_token_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"grant_type":"sample","appid":"sample","secret":"sample","force_refresh":true}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::token::stable_token_param::StableTokenParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_token_stable_token_param",
    );
}

#[test]
fn bean_vip_score_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"score":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::vip::score_info::ScoreInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_vip_score_info",
    );
}

#[test]
fn bean_vip_user_grade_info() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"grade":17,"experience_value":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::vip::user_grade_info::UserGradeInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_vip_user_grade_info",
    );
}

#[test]
fn bean_vip_vip_grade_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"openid":"sample","grade":17}"##).unwrap();
    let data: wx_rust_store::bean::vip::vip_grade_param::VipGradeParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_vip_vip_grade_param",
    );
}

#[test]
fn bean_vip_vip_info() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"openid":"sample","union_id":"sample","user_info":{"phone_number":"sample"},"user_grade_info":{"grade":17,"experience_value":"sample"}}"##).unwrap();
    let data: wx_rust_store::bean::vip::vip_info::VipInfo =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_vip_vip_info",
    );
}

#[test]
fn bean_vip_vip_info_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"openid":"sample","need_phone_number":true}"##).unwrap();
    let data: wx_rust_store::bean::vip::vip_info_param::VipInfoParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_vip_vip_info_param",
    );
}

#[test]
fn bean_vip_vip_info_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","info":{"openid":"sample","union_id":"sample","user_info":{"phone_number":"sample"},"user_grade_info":{"grade":17,"experience_value":"sample"}}}"##).unwrap();
    let data: wx_rust_store::bean::vip::vip_info_response::VipInfoResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_vip_vip_info_response",
    );
}

#[test]
fn bean_vip_vip_list_param() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"need_phone_number":true,"page_num":17,"page_size":17}"##)
            .unwrap();
    let data: wx_rust_store::bean::vip::vip_list_param::VipListParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_vip_vip_list_param",
    );
}

#[test]
fn bean_vip_vip_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","list":[{"openid":"sample","union_id":"sample","user_info":{"phone_number":"sample"},"user_grade_info":{"grade":17,"experience_value":"sample"}}],"total_num":3000000000}"##).unwrap();
    let data: wx_rust_store::bean::vip::vip_list_response::VipListResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_vip_vip_list_response",
    );
}

#[test]
fn bean_vip_vip_open_id_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"openid":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::vip::vip_open_id_param::VipOpenIdParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_vip_vip_open_id_param",
    );
}

#[test]
fn bean_vip_vip_score_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"openid":"sample","score":"sample","remark":"sample","request_id":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::vip::vip_score_param::VipScoreParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_vip_vip_score_param",
    );
}

#[test]
fn bean_vip_vip_score_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","info":{"score":"sample"}}"##)
            .unwrap();
    let data: wx_rust_store::bean::vip::vip_score_response::VipScoreResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_vip_vip_score_response",
    );
}

#[test]
fn bean_warehouse_location_priority_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","priority_sort":["sample"]}"##)
            .unwrap();
    let data: wx_rust_store::bean::warehouse::location_priority_response::LocationPriorityResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_warehouse_location_priority_response",
    );
}

#[test]
fn bean_warehouse_priority_location_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"address_id1":17,"address_id2":17,"address_id3":17,"address_id4":17,"priority_sort":["sample"]}"##).unwrap();
    let data: wx_rust_store::bean::warehouse::priority_location_param::PriorityLocationParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_warehouse_priority_location_param",
    );
}

#[test]
fn bean_warehouse_stock_get_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"product_id":"sample","sku_id":"sample","out_warehouse_id":"sample"}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::warehouse::stock_get_param::StockGetParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_warehouse_stock_get_param",
    );
}

#[test]
fn bean_warehouse_update_location_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"out_warehouse_id":"sample","cover_locations":[{"address_id1":17,"address_id2":17,"address_id3":17,"address_id4":17}]}"##).unwrap();
    let data: wx_rust_store::bean::warehouse::update_location_param::UpdateLocationParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_warehouse_update_location_param",
    );
}

#[test]
fn bean_warehouse_warehouse() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"out_warehouse_id":"sample","name":"sample","intro":"sample","cover_locations":[{"address_id1":17,"address_id2":17,"address_id3":17,"address_id4":17}]}"##).unwrap();
    let data: wx_rust_store::bean::warehouse::warehouse::Warehouse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_warehouse_warehouse",
    );
}

#[test]
fn bean_warehouse_warehouse_location() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"address_id1":17,"address_id2":17,"address_id3":17,"address_id4":17}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::warehouse::warehouse_location::WarehouseLocation =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_warehouse_warehouse_location",
    );
}

#[test]
fn bean_warehouse_warehouse_location_param() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"address_id1":17,"address_id2":17,"address_id3":17,"address_id4":17}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::warehouse::warehouse_location_param::WarehouseLocationParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_warehouse_warehouse_location_param",
    );
}

#[test]
fn bean_warehouse_warehouse_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"out_warehouse_id":"sample","name":"sample","intro":"sample","cover_locations":[{"address_id1":17,"address_id2":17,"address_id3":17,"address_id4":17}]}"##).unwrap();
    let data: wx_rust_store::bean::warehouse::warehouse_param::WarehouseParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_warehouse_warehouse_param",
    );
}

#[test]
fn bean_warehouse_warehouse_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","data":{"out_warehouse_id":"sample","name":"sample","intro":"sample","cover_locations":[{"address_id1":17,"address_id2":17,"address_id3":17,"address_id4":17}]}}"##).unwrap();
    let data: wx_rust_store::bean::warehouse::warehouse_response::WarehouseResponse =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_warehouse_warehouse_response",
    );
}

#[test]
fn bean_warehouse_warehouse_stock_param() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"product_id":"sample","sku_id":"sample","diff_type":17,"num":17,"out_warehouse_id":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::warehouse::warehouse_stock_param::WarehouseStockParam =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_warehouse_warehouse_stock_param",
    );
}

#[test]
fn bean_window_request_add_window_product_request() {
    let fixture: serde_json::Value = serde_json::from_str(
        r##"{"product_id":"sample","appid":"sample","is_hide_for_window":true}"##,
    )
    .unwrap();
    let data: wx_rust_store::bean::window::request::add_window_product_request::AddWindowProductRequest = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_window_request_add_window_product_request",
    );
}

#[test]
fn bean_window_request_get_window_product_list_request() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"appid":"sample","branch_id":17,"page_size":17,"page_index":17,"last_buffer":"sample","need_total_num":17}"##).unwrap();
    let data: wx_rust_store::bean::window::request::get_window_product_list_request::GetWindowProductListRequest = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_window_request_get_window_product_list_request",
    );
}

#[test]
fn bean_window_request_window_product_request() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"product_id":"sample","appid":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::window::request::window_product_request::WindowProductRequest =
        serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_window_request_window_product_request",
    );
}

#[test]
fn bean_window_response_get_window_product_list_response() {
    let fixture: serde_json::Value = serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","products":[{"product_id":"sample","appid":"sample"}],"last_buffer":"sample","total_num":17}"##).unwrap();
    let data: wx_rust_store::bean::window::response::get_window_product_list_response::GetWindowProductListResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_window_response_get_window_product_list_response",
    );
}

#[test]
fn bean_window_response_get_window_product_response() {
    let fixture: serde_json::Value =
        serde_json::from_str(r##"{"errcode":0,"errmsg":"sample","product":"sample"}"##).unwrap();
    let data: wx_rust_store::bean::window::response::get_window_product_response::GetWindowProductResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_subset(
        &fixture,
        &serde_json::to_value(data).unwrap(),
        "bean_window_response_get_window_product_response",
    );
}
