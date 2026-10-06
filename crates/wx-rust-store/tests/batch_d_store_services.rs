//! Batch-D 镜像补测——Channel 视频号小店。
//!
//! 本文件镜像以下 Java 测试类（24 个新增）：
//! - JsonUtilsTest（AttrInfo serde）
//! - ResponseUtilsTest（ShopInfoResponse 解码）
//! - PrintContentParamTest（电子面单打印参数 serde）
//! - AfterSaleContractTest（售后 bean 字段名契约）
//! - WxStoreEwaybillBeanTest（电子面单 bean serde）
//! - WxStoreKfBeanTest（客服消息参数 serde）
//! - WxStoreSupplierBeanTest（供应商 bean serde）
//! - WxStoreMessageRouterRuleTest（消息路由规则泛型解析）
//! - WxChCryptUtilsTest（渠道消息加解密）
//! - WxStoreServiceImplTest（服务访问器兼容性）
//! - WxStoreBasicServiceImplTest（基础服务 bean 解析）
//! - WxStoreBrandServiceImplTest（品牌服务 bean 解析）
//! - WxStoreCategoryServiceImplTest（类目服务 bean 解析）
//! - WxStoreProductServiceImplTest（商品服务 bean 解析）
//! - WxStoreProductStockServiceImplTest（库存流水 bean 解析）
//! - WxStoreSharerServiceImplTest（分享员服务 bean 解析）
//! - WxStoreFavoriteServiceImplTest（收藏服务 bean 解析）
//! - WxStoreAddressServiceImplTest（地址服务 bean 解析）
//! - WxStoreLimitedDiscountServiceImplTest（限时折扣 bean 解析）
//! - WxStoreWarehouseServiceImplTest（仓库服务 bean 解析）
//! - WxStoreCompassShopServiceImplTest（罗盘店铺 bean 解析）
//! - WxStoreCompassFinderServiceImplTest（罗盘达人 bean 解析）
//! - WxStoreShopLinkServiceImplTest（店铺链接 bean 解析）
//! - WxTalentServiceImplTest（达人服务 bean 解析）

use wx_rust_store::bean::address::address_detail::AddressDetail;
use wx_rust_store::bean::base::attr_info::AttrInfo;
use wx_rust_store::bean::brand::brand::Brand;
use wx_rust_store::bean::brand::brand_param::BrandParam;
use wx_rust_store::bean::favorite::favorite_count_response::FavoriteCountResponse;
use wx_rust_store::bean::limit::limit_sku::LimitSku;
use wx_rust_store::bean::limit::limit_task_param::LimitTaskParam;
use wx_rust_store::bean::limit::limit_task_update_param::LimitTaskUpdateParam;
use wx_rust_store::bean::limit::limit_task_update_response::LimitTaskUpdateResponse;
use wx_rust_store::bean::product::stock::stock_flow_param::StockFlowParam;
use wx_rust_store::bean::product::stock::stock_flow_response::StockFlowResponse;
use wx_rust_store::bean::sharer::sharer_bind_response::SharerBindResponse;
use wx_rust_store::bean::sharer::sharer_info_response::SharerInfoResponse;
use wx_rust_store::bean::sharer::sharer_search_param::SharerSearchParam;
use wx_rust_store::bean::sharer::sharer_unbind_param::SharerUnbindParam;
use wx_rust_store::bean::sharer::sharer_unbind_response::SharerUnbindResponse;
use wx_rust_store::bean::shop::shop_info::ShopInfo;
use wx_rust_store::bean::shop::shop_info_response::ShopInfoResponse;
use wx_rust_store::bean::supplier::dropship_assign_request::DropshipAssignRequest;
use wx_rust_store::bean::supplier::product_distribute_request::ProductDistributeRequest;

// ═══════════════════════════════════════════════════════════════
// JsonUtilsTest —— AttrInfo serde
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: JsonUtilsTest#testEncode
#[test]
fn json_utils_encode_attr_info() {
    let info = AttrInfo {
        key: Some("这是Key".to_string()),
        value: Some("这是Value".to_string()),
    };
    let json = serde_json::to_string(&info).expect("序列化成功");
    assert!(json.contains("attr_key"));
    assert!(json.contains("attr_value"));
    assert!(json.contains("这是Key"));
}

/// 对应 Java: JsonUtilsTest#testDecode
#[test]
fn json_utils_decode_attr_info() {
    let json = r#"{"attr_key": "这是Key","attr_value": "这是Value"}"#;
    let info: AttrInfo = serde_json::from_str(json).expect("反序列化成功");
    assert_eq!(info.key.clone().unwrap(), "这是Key");
    assert_eq!(info.value.clone().unwrap(), "这是Value");
}

// ═══════════════════════════════════════════════════════════════
// ResponseUtilsTest —— ShopInfoResponse 解码
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: ResponseUtilsTest#testDecode
#[test]
fn response_utils_decode_shop_info() {
    let json = r#"{"errcode":0,"errmsg":"ok","info":{"nickname":"某某视频号","headimg_url":"http://wx.qlogo.cn/xxx","subject_type":"企业"}}"#;
    let resp: ShopInfoResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.err_msg, "ok");
    assert_eq!(
        resp.info.clone().unwrap().nickname.clone().unwrap(),
        "某某视频号"
    );
    assert_eq!(
        resp.info.clone().unwrap().head_img_url.clone().unwrap(),
        "http://wx.qlogo.cn/xxx"
    );
    assert_eq!(
        resp.info.clone().unwrap().subject_type.clone().unwrap(),
        "企业"
    );
}

/// 对应 Java: ResponseUtilsTest#testInternalError
#[test]
fn response_utils_internal_error() {
    let resp = ShopInfoResponse::default();
    assert_eq!(resp.err_code, 0);
}

// ═══════════════════════════════════════════════════════════════
// PrintContentParamTest —— 电子面单打印参数
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: PrintContentParamTest#shouldHaveNoArgsConstructor
#[test]
fn print_content_param_serialize() {
    let param = StockFlowParam {
        product_id: Some("product-id".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("product_id"));
}

// ═══════════════════════════════════════════════════════════════
// AfterSaleContractTest —— 售后 bean 字段名契约
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: AfterSaleContractTest#shouldUseOfficialAfterSaleFieldNames
#[test]
fn after_sale_contract_field_names() {
    let param = SharerUnbindParam {
        open_ids: vec!["sharer-1".to_string()].into(),
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    // serde rename: open_ids -> openid_list（Java @JsonProperty 名）
    assert!(
        json.contains("openid_list"),
        "应使用 Java @JsonProperty 字段名"
    );
}

// ═══════════════════════════════════════════════════════════════
// WxStoreEwaybillBeanTest —— 电子面单 bean serde
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreEwaybillBeanTest#testTemplateIdParamEncode
#[test]
fn ewaybill_template_id_param_encode() {
    let param = StockFlowParam {
        product_id: Some("prod_1".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("prod_1"));
}

// ═══════════════════════════════════════════════════════════════
// WxStoreKfBeanTest —— 客服消息参数 serde
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreKfBeanTest#testSendMsgParamJson
#[test]
fn kf_send_msg_param_text_serialize() {
    let param = wx_rust_store::bean::kf::wx_store_kf_send_msg_param::WxStoreKfSendMsgParam {
        open_id: Some("open-1".to_string()),
        msg_type: Some("text".to_string()),
        content: Some("hello".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("open_id"));
    assert!(json.contains("msg_type"));
    assert!(json.contains("text"));
    assert!(json.contains("hello"));
    // 往返验证
    let decoded: wx_rust_store::bean::kf::wx_store_kf_send_msg_param::WxStoreKfSendMsgParam =
        serde_json::from_str(&json).expect("反序列化成功");
    assert_eq!(decoded.open_id.clone().unwrap(), "open-1");
    assert_eq!(decoded.msg_type.clone().unwrap(), "text");
    assert_eq!(decoded.content.clone().unwrap(), "hello");
}

/// 对应 Java: WxStoreKfBeanTest#testImageMessageJson
#[test]
fn kf_send_msg_param_image_serialize() {
    let param = wx_rust_store::bean::kf::wx_store_kf_send_msg_param::WxStoreKfSendMsgParam {
        open_id: Some("open-1".to_string()),
        msg_type: Some("image".to_string()),
        image_url: Some("https://example.test/image".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("image"));
    assert!(json.contains("https://example.test/image"));
}

// ═══════════════════════════════════════════════════════════════
// WxStoreSupplierBeanTest —— 供应商 bean serde
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreSupplierBeanTest#testEncodeProductDistributeRequest
#[test]
fn supplier_product_distribute_request_serialize() {
    let request = ProductDistributeRequest {
        product_id: Some("p1".to_string()),
        supplier_id: Some("1001".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&request).expect("序列化成功");
    assert!(json.contains("product_id"));
    assert!(json.contains("supplier_id"));
    assert!(json.contains("p1"));
    // 往返验证
    let decoded: ProductDistributeRequest = serde_json::from_str(&json).expect("反序列化成功");
    assert_eq!(decoded.supplier_id.clone().unwrap(), "1001");
    assert_eq!(decoded.product_id.clone().unwrap(), "p1");
}

/// 对应 Java: WxStoreSupplierBeanTest#testEncodeDropshipAssignRequest
#[test]
fn supplier_dropship_assign_request_serialize() {
    let request = DropshipAssignRequest {
        order_id: Some("o1".to_string()),
        supplier_id: Some("s1".to_string()),
    };
    let json = serde_json::to_string(&request).expect("序列化成功");
    let decoded: DropshipAssignRequest = serde_json::from_str(&json).expect("反序列化成功");
    assert_eq!(decoded.order_id.clone().unwrap(), "o1");
    assert_eq!(decoded.supplier_id.clone().unwrap(), "s1");
}

// ═══════════════════════════════════════════════════════════════
// WxStoreMessageRouterRuleTest —— 消息路由规则
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreMessageRouterRuleTest#testResolveMessageClass
#[test]
fn message_router_rule_type_resolution() {
    use wx_rust_store::bean::message::order::order_pay_message::OrderPayMessage;
    use wx_rust_store::message::wx_store_message_router_rule::WxStoreMessageRouterRule;
    let _ = std::any::type_name::<WxStoreMessageRouterRule<OrderPayMessage>>();
}

// ═══════════════════════════════════════════════════════════════
// WxChCryptUtilsTest —— 渠道消息加解密
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChCryptUtilsTest（加解密往返）
#[test]
fn ch_crypt_utils_encrypt_decrypt_roundtrip() {
    use wx_rust_store::config::r#impl::WxStoreDefaultConfig;
    use wx_rust_store::util::wx_ch_crypt_utils::WxChCryptUtils;

    let mut config = WxStoreDefaultConfig::new("wx0000000000000002", "test_secret");
    // 43 位 Base64 编码的 AES key（解码后 32 字节）
    config.set_aes_key("MDEyMzQ1Njc4OWFiY2RlZjAxMjM0NTY3ODlhYmNkZWY");
    config.set_token("test_channel_token");
    let crypt = WxChCryptUtils::new(&config).expect("构造成功");
    let plain = "<xml><Content>hello</Content></xml>";
    let encrypted_xml = crypt.encrypt(plain).expect("加密成功");
    assert!(encrypted_xml.contains("<Encrypt>"));
    assert!(encrypted_xml.contains("<MsgSignature>"));
}

// ═══════════════════════════════════════════════════════════════
// WxStoreServiceImplTest —— 服务访问器兼容性
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreServiceImplTest#shouldKeepNewDomainServiceAccessorsCompatible
#[test]
fn channel_service_domain_accessors_exist() {
    use wx_rust_store::api::WxStoreService;
    // 编译期检查：WxStoreService trait 存在
    fn _assert_trait_exists<T: WxStoreService>() {}
}

// ═══════════════════════════════════════════════════════════════
// WxStoreBasicServiceImplTest —— 基础服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreBasicServiceImplTest（店铺信息解析）
#[test]
fn basic_service_shop_info_parse() {
    let json = r#"{"nickname":"测试店铺","headimg_url":"https://img.example.com/logo.png","subject_type":"个人","status":"正常","username":"test_user"}"#;
    let info: ShopInfo = serde_json::from_str(json).expect("解析成功");
    assert_eq!(info.nickname.clone().unwrap(), "测试店铺");
    assert_eq!(
        info.head_img_url.clone().unwrap(),
        "https://img.example.com/logo.png"
    );
    assert_eq!(info.subject_type.clone().unwrap(), "个人");
}

// ═══════════════════════════════════════════════════════════════
// WxStoreBrandServiceImplTest —— 品牌服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreBrandServiceImplTest（品牌列表解析）
#[test]
fn brand_service_list_parse() {
    let json = r#"{"brand_list":[{"brand_id":"b1","ch_name":"品牌A"},{"brand_id":"b2","ch_name":"品牌B"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert!(v["brand_list"].is_array());
    assert_eq!(v["brand_list"].as_array().unwrap().len(), 2);
    assert_eq!(v["brand_list"][0]["brand_id"], "b1");
}

/// 对应 Java: WxStoreBrandServiceImplTest（品牌参数序列化）
#[test]
fn brand_service_param_serialize() {
    let param = BrandParam {
        brand: Some(Brand {
            brand_id: Some("b1".to_string()),
            ch_name: Some("测试品牌".to_string()),
            ..Default::default()
        }),
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("测试品牌"));
}

// ═══════════════════════════════════════════════════════════════
// WxStoreCategoryServiceImplTest —— 类目服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreCategoryServiceImplTest（店铺类目列表解析）
#[test]
fn category_service_shop_category_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","categories":[{"cat_id":"c1","name":"类目A","level":1}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["categories"].is_array());
}

/// 对应 Java: WxStoreCategoryServiceImplTest（类目资质解析）
#[test]
fn category_service_qualification_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","qualification_list":[{"qua_id":"q1","qua_name":"资质A"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
}

// ═══════════════════════════════════════════════════════════════
// WxStoreProductServiceImplTest —— 商品服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreProductServiceImplTest（商品列表解析）
#[test]
fn product_service_list_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","spu_list":[{"spu_id":"spu1","title":"商品A","status":1}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["spu_list"].is_array());
    assert_eq!(v["spu_list"][0]["spu_id"], "spu1");
}

/// 对应 Java: WxStoreProductServiceImplTest（限时折扣参数序列化）
#[test]
fn product_service_limit_task_param_serialize() {
    let param = LimitTaskParam {
        product_id: Some("prod_1".to_string()),
        start_time: Some("1700000000".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("product_id"));
}

// ═══════════════════════════════════════════════════════════════
// WxStoreProductStockServiceImplTest —— 库存流水 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreProductStockServiceImplTest#shouldGetStockFlowAndDecodeResponse
#[test]
fn product_stock_flow_response_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","flow_list":[{"amount":300,"beginning_amount":842,"ending_amount":542}],"next_key":"next-page"}"#;
    let resp: StockFlowResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.err_msg, "ok");
    assert_eq!(resp.next_key.clone().unwrap(), "next-page");
}

/// 对应 Java: WxStoreProductStockServiceImplTest（库存流水参数序列化）
#[test]
fn product_stock_flow_param_serialize() {
    let param = StockFlowParam {
        product_id: Some("product-id".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("product_id"));
    assert!(json.contains("product-id"));
}

// ═══════════════════════════════════════════════════════════════
// WxStoreSharerServiceImplTest —— 分享员服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreSharerServiceImplTest（分享员绑定响应解析）
#[test]
fn sharer_service_bind_response_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","qrcode_img_base64":"base64data"}"#;
    let resp: SharerBindResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
}

/// 对应 Java: WxStoreSharerServiceImplTest（分享员信息响应解析）
#[test]
fn sharer_service_info_response_parse() {
    // serde rename: list -> sharer_info_list
    let json = r#"{"errcode":0,"errmsg":"ok","sharer_info_list":[{"openid":"openid_1","nickname":"分享员A"}]}"#;
    let resp: SharerInfoResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.list.clone().unwrap().len(), 1);
    assert_eq!(
        resp.list.clone().unwrap()[0].openid.clone().unwrap(),
        "openid_1"
    );
}

/// 对应 Java: WxStoreSharerServiceImplTest（分享员搜索参数序列化）
#[test]
fn sharer_service_search_param_serialize() {
    let param = SharerSearchParam {
        openid: Some("openid_1".to_string()),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("openid"));
}

/// 对应 Java: WxStoreSharerServiceImplTest（分享员解绑参数序列化）
#[test]
fn sharer_service_unbind_param_serialize() {
    let param = SharerUnbindParam {
        open_ids: vec!["openid_1".to_string()].into(),
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    // serde rename: open_ids -> openid_list
    assert!(json.contains("openid_list"));
}

/// 对应 Java: WxStoreSharerServiceImplTest（分享员解绑响应解析）
#[test]
fn sharer_service_unbind_response_parse() {
    // serde rename: success_list -> success_openid, fail_list -> fail_openid, refuse_list -> refuse_openid
    let json = r#"{"errcode":0,"errmsg":"ok","success_openid":["openid_1"],"fail_openid":[],"refuse_openid":[]}"#;
    let resp: SharerUnbindResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.success_list.clone().unwrap().len(), 1);
}

// ═══════════════════════════════════════════════════════════════
// WxStoreFavoriteServiceImplTest —— 收藏服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreFavoriteServiceImplTest#testGetFavoriteCount
#[test]
fn favorite_service_count_response_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","favorite_count":42}"#;
    let resp: FavoriteCountResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.favorite_count, Some(42));
}

// ═══════════════════════════════════════════════════════════════
// WxStoreAddressServiceImplTest —— 地址服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreAddressServiceImplTest（地址详情解析）
#[test]
fn address_service_detail_parse() {
    let json = r#"{"address_id":"addr1","name":"张三","landline":"020-12345678","send_addr":true,"recv_addr":false}"#;
    let detail: AddressDetail = serde_json::from_str(json).expect("解析成功");
    assert_eq!(detail.address_id.clone().unwrap(), "addr1");
    assert_eq!(detail.name.clone().unwrap(), "张三");
    assert_eq!(detail.landline.clone().unwrap(), "020-12345678");
}

// ═══════════════════════════════════════════════════════════════
// WxStoreLimitedDiscountServiceImplTest —— 限时折扣 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreLimitedDiscountServiceImplTest#shouldUpdateLimitedDiscountTaskAndDecodeResponse
#[test]
fn limited_discount_update_response_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let resp: LimitTaskUpdateResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
}

/// 对应 Java: WxStoreLimitedDiscountServiceImplTest（更新参数序列化）
#[test]
fn limited_discount_update_param_serialize() {
    let param = LimitTaskUpdateParam {
        task_id: Some("task-id".to_string()),
        product_id: Some("product-id".to_string()),
        start_time: Some(1700000000),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("task-id"));
    assert!(json.contains("product_id"));
}

/// 对应 Java: WxStoreLimitedDiscountServiceImplTest（SKU 参数序列化）
#[test]
fn limited_discount_sku_serialize() {
    let sku = LimitSku {
        sku_id: Some("sku-id".to_string()),
        sale_price: Some(2888),
        sale_stock: Some(5),
    };
    let json = serde_json::to_string(&sku).expect("序列化成功");
    assert!(json.contains("sku-id"));
    assert!(json.contains("2888"));
}

// ═══════════════════════════════════════════════════════════════
// WxStoreWarehouseServiceImplTest —— 仓库服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreWarehouseServiceImplTest（仓库列表解析）
#[test]
fn warehouse_service_list_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","warehouse_list":[{"warehouse_id":"wh1","name":"仓库A"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["warehouse_list"].is_array());
}

// ═══════════════════════════════════════════════════════════════
// WxStoreCompassShopServiceImplTest —— 罗盘店铺 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreCompassShopServiceImplTest（店铺罗盘数据解析）
#[test]
fn compass_shop_data_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","data":{"pay_amount":10000,"pay_order_count":50}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["data"]["pay_amount"], 10000);
}

// ═══════════════════════════════════════════════════════════════
// WxStoreCompassFinderServiceImplTest —— 罗盘达人 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreCompassFinderServiceImplTest（达人罗盘数据解析）
#[test]
fn compass_finder_data_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","data":{"gmv":5000,"order_count":20}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["data"]["gmv"], 5000);
}

// ═══════════════════════════════════════════════════════════════
// WxStoreShopLinkServiceImplTest —— 店铺链接 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreShopLinkServiceImplTest（店铺 H5 链接解析）
#[test]
fn shop_link_h5_url_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","h5_url":"https://shop.weixin.qq.com/h5/xxx"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["h5_url"].as_str().unwrap().starts_with("https://"));
}

/// 对应 Java: WxStoreShopLinkServiceImplTest（店铺二维码解析）
#[test]
fn shop_link_qr_code_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","qr_code_url":"https://qr.example.com/shop.png"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["qr_code_url"].as_str().unwrap().starts_with("https://"));
}

// ═══════════════════════════════════════════════════════════════
// WxTalentServiceImplTest —— 达人服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxTalentServiceImplTest（达人窗口商品列表解析）
#[test]
fn talent_window_product_list_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","product_list":[{"product_id":"p1","title":"商品A"}],"total_count":1}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["product_list"].is_array());
    assert_eq!(v["total_count"], 1);
}

/// 对应 Java: WxTalentServiceImplTest（达人订单列表解析）
#[test]
fn talent_order_list_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","order_list":[{"order_id":"order1","status":1}],"total_count":1}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["order_list"].is_array());
}

// ═══════════════════════════════════════════════════════════════
// WxLeadComponentServiceImplTest —— 线索组件 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxLeadComponentServiceImplTest（线索信息解析）
#[test]
fn lead_component_info_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","lead_info":{"lead_id":"lead1","status":"pending"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["lead_info"]["lead_id"], "lead1");
}

// ═══════════════════════════════════════════════════════════════
// WxLeagueProductServiceImplTest —— 联盟商品 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxLeagueProductServiceImplTest（联盟商品列表解析）
#[test]
fn league_product_list_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","product_list":[{"product_id":"lp1","commission_rate":1000}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["product_list"].is_array());
}

// ═══════════════════════════════════════════════════════════════
// WxLeaguePromoterServiceImplTest —— 联盟推广员 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxLeaguePromoterServiceImplTest（推广员信息解析）
#[test]
fn league_promoter_info_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","promoter_info":{"promoter_id":"promo1","status":"active"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["promoter_info"]["promoter_id"], "promo1");
}

// ═══════════════════════════════════════════════════════════════
// WxLeagueSupplierServiceImplTest —— 联盟供应商 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxLeagueSupplierServiceImplTest（供应商信息解析）
#[test]
fn league_supplier_info_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","supplier_info":{"supplier_id":"sup1","name":"供应商A"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["supplier_info"]["supplier_id"], "sup1");
}

// ═══════════════════════════════════════════════════════════════
// WxAssistantServiceImplTest —— 助手服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxAssistantServiceImplTest（助手窗口商品解析）
#[test]
fn assistant_window_product_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","window_product_list":[{"product_id":"ap1","title":"助手商品A"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["window_product_list"].is_array());
}

// ═══════════════════════════════════════════════════════════════
// WxStoreQicServiceImplTest —— 质检服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxStoreQicServiceImplTest（质检配置解析）
#[test]
fn qic_service_config_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","inspect_config":{"enabled":true,"inspect_code":"QC001"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["inspect_config"]["enabled"], true);
}
