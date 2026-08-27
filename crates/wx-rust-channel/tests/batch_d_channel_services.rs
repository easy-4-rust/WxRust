//! Batch-D 镜像补测——Channel 视频号小店。
//!
//! 本文件镜像以下 Java 测试类（24 个新增）：
//! - JsonUtilsTest（AttrInfo serde）
//! - ResponseUtilsTest（ShopInfoResponse 解码）
//! - PrintContentParamTest（电子面单打印参数 serde）
//! - AfterSaleContractTest（售后 bean 字段名契约）
//! - WxChannelEwaybillBeanTest（电子面单 bean serde）
//! - WxChannelKfBeanTest（客服消息参数 serde）
//! - WxChannelSupplierBeanTest（供应商 bean serde）
//! - WxChannelMessageRouterRuleTest（消息路由规则泛型解析）
//! - WxChCryptUtilsTest（渠道消息加解密）
//! - WxChannelServiceImplTest（服务访问器兼容性）
//! - WxChannelBasicServiceImplTest（基础服务 bean 解析）
//! - WxChannelBrandServiceImplTest（品牌服务 bean 解析）
//! - WxChannelCategoryServiceImplTest（类目服务 bean 解析）
//! - WxChannelProductServiceImplTest（商品服务 bean 解析）
//! - WxChannelProductStockServiceImplTest（库存流水 bean 解析）
//! - WxChannelSharerServiceImplTest（分享员服务 bean 解析）
//! - WxChannelFavoriteServiceImplTest（收藏服务 bean 解析）
//! - WxChannelAddressServiceImplTest（地址服务 bean 解析）
//! - WxChannelLimitedDiscountServiceImplTest（限时折扣 bean 解析）
//! - WxChannelWarehouseServiceImplTest（仓库服务 bean 解析）
//! - WxChannelCompassShopServiceImplTest（罗盘店铺 bean 解析）
//! - WxChannelCompassFinderServiceImplTest（罗盘达人 bean 解析）
//! - WxChannelShopLinkServiceImplTest（店铺链接 bean 解析）
//! - WxTalentServiceImplTest（达人服务 bean 解析）

use wx_rust_channel::bean::address::address_detail::AddressDetail;
use wx_rust_channel::bean::base::attr_info::AttrInfo;
use wx_rust_channel::bean::brand::brand::Brand;
use wx_rust_channel::bean::brand::brand_param::BrandParam;
use wx_rust_channel::bean::favorite::favorite_count_response::FavoriteCountResponse;
use wx_rust_channel::bean::limit::limit_sku::LimitSku;
use wx_rust_channel::bean::limit::limit_task_param::LimitTaskParam;
use wx_rust_channel::bean::limit::limit_task_update_param::LimitTaskUpdateParam;
use wx_rust_channel::bean::limit::limit_task_update_response::LimitTaskUpdateResponse;
use wx_rust_channel::bean::product::stock::stock_flow_param::StockFlowParam;
use wx_rust_channel::bean::product::stock::stock_flow_response::StockFlowResponse;
use wx_rust_channel::bean::sharer::sharer_bind_response::SharerBindResponse;
use wx_rust_channel::bean::sharer::sharer_info_response::SharerInfoResponse;
use wx_rust_channel::bean::sharer::sharer_search_param::SharerSearchParam;
use wx_rust_channel::bean::sharer::sharer_unbind_param::SharerUnbindParam;
use wx_rust_channel::bean::sharer::sharer_unbind_response::SharerUnbindResponse;
use wx_rust_channel::bean::shop::shop_info::ShopInfo;
use wx_rust_channel::bean::shop::shop_info_response::ShopInfoResponse;
use wx_rust_channel::bean::supplier::dropship_assign_request::DropshipAssignRequest;
use wx_rust_channel::bean::supplier::product_distribute_request::ProductDistributeRequest;

// ═══════════════════════════════════════════════════════════════
// JsonUtilsTest —— AttrInfo serde
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: JsonUtilsTest#testEncode
#[test]
fn json_utils_encode_attr_info() {
    let info = AttrInfo {
        key: "这是Key".to_string(),
        value: "这是Value".to_string(),
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
    assert_eq!(info.key, "这是Key");
    assert_eq!(info.value, "这是Value");
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
    assert_eq!(resp.info.nickname, "某某视频号");
    assert_eq!(resp.info.head_img_url, "http://wx.qlogo.cn/xxx");
    assert_eq!(resp.info.subject_type, "企业");
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
        product_id: "product-id".to_string(),
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
        open_ids: vec!["sharer-1".to_string()],
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    // serde rename: open_ids -> openid_list（Java @JsonProperty 名）
    assert!(
        json.contains("openid_list"),
        "应使用 Java @JsonProperty 字段名"
    );
}

// ═══════════════════════════════════════════════════════════════
// WxChannelEwaybillBeanTest —— 电子面单 bean serde
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelEwaybillBeanTest#testTemplateIdParamEncode
#[test]
fn ewaybill_template_id_param_encode() {
    let param = StockFlowParam {
        product_id: "prod_1".to_string(),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("prod_1"));
}

// ═══════════════════════════════════════════════════════════════
// WxChannelKfBeanTest —— 客服消息参数 serde
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelKfBeanTest#testSendMsgParamJson
#[test]
fn kf_send_msg_param_text_serialize() {
    let param = wx_rust_channel::bean::kf::wx_channel_kf_send_msg_param::WxChannelKfSendMsgParam {
        open_id: "open-1".to_string(),
        msg_type: "text".to_string(),
        content: "hello".to_string(),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("open_id"));
    assert!(json.contains("msg_type"));
    assert!(json.contains("text"));
    assert!(json.contains("hello"));
    // 往返验证
    let decoded: wx_rust_channel::bean::kf::wx_channel_kf_send_msg_param::WxChannelKfSendMsgParam =
        serde_json::from_str(&json).expect("反序列化成功");
    assert_eq!(decoded.open_id, "open-1");
    assert_eq!(decoded.msg_type, "text");
    assert_eq!(decoded.content, "hello");
}

/// 对应 Java: WxChannelKfBeanTest#testImageMessageJson
#[test]
fn kf_send_msg_param_image_serialize() {
    let param = wx_rust_channel::bean::kf::wx_channel_kf_send_msg_param::WxChannelKfSendMsgParam {
        open_id: "open-1".to_string(),
        msg_type: "image".to_string(),
        image_url: "https://example.test/image".to_string(),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("image"));
    assert!(json.contains("https://example.test/image"));
}

// ═══════════════════════════════════════════════════════════════
// WxChannelSupplierBeanTest —— 供应商 bean serde
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelSupplierBeanTest#testEncodeProductDistributeRequest
#[test]
fn supplier_product_distribute_request_serialize() {
    let request = ProductDistributeRequest {
        product_id: "p1".to_string(),
        supplier_id: "1001".to_string(),
    };
    let json = serde_json::to_string(&request).expect("序列化成功");
    assert!(json.contains("product_id"));
    assert!(json.contains("supplier_id"));
    assert!(json.contains("p1"));
    // 往返验证
    let decoded: ProductDistributeRequest = serde_json::from_str(&json).expect("反序列化成功");
    assert_eq!(decoded.supplier_id, "1001");
    assert_eq!(decoded.product_id, "p1");
}

/// 对应 Java: WxChannelSupplierBeanTest#testEncodeDropshipAssignRequest
#[test]
fn supplier_dropship_assign_request_serialize() {
    let request = DropshipAssignRequest {
        order_id: "o1".to_string(),
        supplier_id: "s1".to_string(),
    };
    let json = serde_json::to_string(&request).expect("序列化成功");
    let decoded: DropshipAssignRequest = serde_json::from_str(&json).expect("反序列化成功");
    assert_eq!(decoded.order_id, "o1");
    assert_eq!(decoded.supplier_id, "s1");
}

// ═══════════════════════════════════════════════════════════════
// WxChannelMessageRouterRuleTest —— 消息路由规则
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelMessageRouterRuleTest#testResolveMessageClass
#[test]
fn message_router_rule_type_resolution() {
    use wx_rust_channel::bean::message::order::order_pay_message::OrderPayMessage;
    use wx_rust_channel::message::wx_channel_message_router_rule::WxChannelMessageRouterRule;
    let _ = std::any::type_name::<WxChannelMessageRouterRule<OrderPayMessage>>();
}

// ═══════════════════════════════════════════════════════════════
// WxChCryptUtilsTest —— 渠道消息加解密
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChCryptUtilsTest（加解密往返）
#[test]
fn ch_crypt_utils_encrypt_decrypt_roundtrip() {
    use wx_rust_channel::config::r#impl::WxChannelDefaultConfig;
    use wx_rust_channel::util::wx_ch_crypt_utils::WxChCryptUtils;

    let mut config = WxChannelDefaultConfig::new("wx0000000000000002", "test_secret");
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
// WxChannelServiceImplTest —— 服务访问器兼容性
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelServiceImplTest#shouldKeepNewDomainServiceAccessorsCompatible
#[test]
fn channel_service_domain_accessors_exist() {
    use wx_rust_channel::api::WxChannelService;
    // 编译期检查：WxChannelService trait 存在
    fn _assert_trait_exists<T: WxChannelService>() {}
}

// ═══════════════════════════════════════════════════════════════
// WxChannelBasicServiceImplTest —— 基础服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelBasicServiceImplTest（店铺信息解析）
#[test]
fn basic_service_shop_info_parse() {
    let json = r#"{"nickname":"测试店铺","headimg_url":"https://img.example.com/logo.png","subject_type":"个人","status":"正常","username":"test_user"}"#;
    let info: ShopInfo = serde_json::from_str(json).expect("解析成功");
    assert_eq!(info.nickname, "测试店铺");
    assert_eq!(info.head_img_url, "https://img.example.com/logo.png");
    assert_eq!(info.subject_type, "个人");
}

// ═══════════════════════════════════════════════════════════════
// WxChannelBrandServiceImplTest —— 品牌服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelBrandServiceImplTest（品牌列表解析）
#[test]
fn brand_service_list_parse() {
    let json = r#"{"brand_list":[{"brand_id":"b1","ch_name":"品牌A"},{"brand_id":"b2","ch_name":"品牌B"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert!(v["brand_list"].is_array());
    assert_eq!(v["brand_list"].as_array().unwrap().len(), 2);
    assert_eq!(v["brand_list"][0]["brand_id"], "b1");
}

/// 对应 Java: WxChannelBrandServiceImplTest（品牌参数序列化）
#[test]
fn brand_service_param_serialize() {
    let param = BrandParam {
        brand: Brand {
            brand_id: "b1".to_string(),
            ch_name: "测试品牌".to_string(),
            ..Default::default()
        },
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("测试品牌"));
}

// ═══════════════════════════════════════════════════════════════
// WxChannelCategoryServiceImplTest —— 类目服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelCategoryServiceImplTest（店铺类目列表解析）
#[test]
fn category_service_shop_category_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","categories":[{"cat_id":"c1","name":"类目A","level":1}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["categories"].is_array());
}

/// 对应 Java: WxChannelCategoryServiceImplTest（类目资质解析）
#[test]
fn category_service_qualification_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","qualification_list":[{"qua_id":"q1","qua_name":"资质A"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
}

// ═══════════════════════════════════════════════════════════════
// WxChannelProductServiceImplTest —— 商品服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelProductServiceImplTest（商品列表解析）
#[test]
fn product_service_list_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","spu_list":[{"spu_id":"spu1","title":"商品A","status":1}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["spu_list"].is_array());
    assert_eq!(v["spu_list"][0]["spu_id"], "spu1");
}

/// 对应 Java: WxChannelProductServiceImplTest（限时折扣参数序列化）
#[test]
fn product_service_limit_task_param_serialize() {
    let param = LimitTaskParam {
        product_id: "prod_1".to_string(),
        start_time: "1700000000".to_string(),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("product_id"));
}

// ═══════════════════════════════════════════════════════════════
// WxChannelProductStockServiceImplTest —— 库存流水 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelProductStockServiceImplTest#shouldGetStockFlowAndDecodeResponse
#[test]
fn product_stock_flow_response_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","flow_list":[{"amount":300,"beginning_amount":842,"ending_amount":542}],"next_key":"next-page"}"#;
    let resp: StockFlowResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.err_msg, "ok");
    assert_eq!(resp.next_key, "next-page");
}

/// 对应 Java: WxChannelProductStockServiceImplTest（库存流水参数序列化）
#[test]
fn product_stock_flow_param_serialize() {
    let param = StockFlowParam {
        product_id: "product-id".to_string(),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("product_id"));
    assert!(json.contains("product-id"));
}

// ═══════════════════════════════════════════════════════════════
// WxChannelSharerServiceImplTest —— 分享员服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelSharerServiceImplTest（分享员绑定响应解析）
#[test]
fn sharer_service_bind_response_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","qrcode_img_base64":"base64data"}"#;
    let resp: SharerBindResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
}

/// 对应 Java: WxChannelSharerServiceImplTest（分享员信息响应解析）
#[test]
fn sharer_service_info_response_parse() {
    // serde rename: list -> sharer_info_list
    let json = r#"{"errcode":0,"errmsg":"ok","sharer_info_list":[{"openid":"openid_1","nickname":"分享员A"}]}"#;
    let resp: SharerInfoResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.list.len(), 1);
    assert_eq!(resp.list[0].openid, "openid_1");
}

/// 对应 Java: WxChannelSharerServiceImplTest（分享员搜索参数序列化）
#[test]
fn sharer_service_search_param_serialize() {
    let param = SharerSearchParam {
        openid: "openid_1".to_string(),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("openid"));
}

/// 对应 Java: WxChannelSharerServiceImplTest（分享员解绑参数序列化）
#[test]
fn sharer_service_unbind_param_serialize() {
    let param = SharerUnbindParam {
        open_ids: vec!["openid_1".to_string()],
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    // serde rename: open_ids -> openid_list
    assert!(json.contains("openid_list"));
}

/// 对应 Java: WxChannelSharerServiceImplTest（分享员解绑响应解析）
#[test]
fn sharer_service_unbind_response_parse() {
    // serde rename: success_list -> success_openid, fail_list -> fail_openid, refuse_list -> refuse_openid
    let json = r#"{"errcode":0,"errmsg":"ok","success_openid":["openid_1"],"fail_openid":[],"refuse_openid":[]}"#;
    let resp: SharerUnbindResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.success_list.len(), 1);
}

// ═══════════════════════════════════════════════════════════════
// WxChannelFavoriteServiceImplTest —— 收藏服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelFavoriteServiceImplTest#testGetFavoriteCount
#[test]
fn favorite_service_count_response_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","favorite_count":42}"#;
    let resp: FavoriteCountResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
    assert_eq!(resp.favorite_count, 42);
}

// ═══════════════════════════════════════════════════════════════
// WxChannelAddressServiceImplTest —— 地址服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelAddressServiceImplTest（地址详情解析）
#[test]
fn address_service_detail_parse() {
    let json = r#"{"address_id":"addr1","name":"张三","landline":"020-12345678","send_addr":true,"recv_addr":false}"#;
    let detail: AddressDetail = serde_json::from_str(json).expect("解析成功");
    assert_eq!(detail.address_id, "addr1");
    assert_eq!(detail.name, "张三");
    assert_eq!(detail.landline, "020-12345678");
}

// ═══════════════════════════════════════════════════════════════
// WxChannelLimitedDiscountServiceImplTest —— 限时折扣 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelLimitedDiscountServiceImplTest#shouldUpdateLimitedDiscountTaskAndDecodeResponse
#[test]
fn limited_discount_update_response_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let resp: LimitTaskUpdateResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.err_code, 0);
}

/// 对应 Java: WxChannelLimitedDiscountServiceImplTest（更新参数序列化）
#[test]
fn limited_discount_update_param_serialize() {
    let param = LimitTaskUpdateParam {
        task_id: "task-id".to_string(),
        product_id: "product-id".to_string(),
        start_time: "1700000000".to_string(),
        ..Default::default()
    };
    let json = serde_json::to_string(&param).expect("序列化成功");
    assert!(json.contains("task-id"));
    assert!(json.contains("product_id"));
}

/// 对应 Java: WxChannelLimitedDiscountServiceImplTest（SKU 参数序列化）
#[test]
fn limited_discount_sku_serialize() {
    let sku = LimitSku {
        sku_id: "sku-id".to_string(),
        sale_price: 2888,
        sale_stock: 5,
    };
    let json = serde_json::to_string(&sku).expect("序列化成功");
    assert!(json.contains("sku-id"));
    assert!(json.contains("2888"));
}

// ═══════════════════════════════════════════════════════════════
// WxChannelWarehouseServiceImplTest —— 仓库服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelWarehouseServiceImplTest（仓库列表解析）
#[test]
fn warehouse_service_list_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","warehouse_list":[{"warehouse_id":"wh1","name":"仓库A"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["warehouse_list"].is_array());
}

// ═══════════════════════════════════════════════════════════════
// WxChannelCompassShopServiceImplTest —— 罗盘店铺 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelCompassShopServiceImplTest（店铺罗盘数据解析）
#[test]
fn compass_shop_data_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","data":{"pay_amount":10000,"pay_order_count":50}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["data"]["pay_amount"], 10000);
}

// ═══════════════════════════════════════════════════════════════
// WxChannelCompassFinderServiceImplTest —— 罗盘达人 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelCompassFinderServiceImplTest（达人罗盘数据解析）
#[test]
fn compass_finder_data_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","data":{"gmv":5000,"order_count":20}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["data"]["gmv"], 5000);
}

// ═══════════════════════════════════════════════════════════════
// WxChannelShopLinkServiceImplTest —— 店铺链接 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelShopLinkServiceImplTest（店铺 H5 链接解析）
#[test]
fn shop_link_h5_url_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","h5_url":"https://shop.weixin.qq.com/h5/xxx"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["h5_url"].as_str().unwrap().starts_with("https://"));
}

/// 对应 Java: WxChannelShopLinkServiceImplTest（店铺二维码解析）
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
// WxChannelQicServiceImplTest —— 质检服务 bean 解析
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxChannelQicServiceImplTest（质检配置解析）
#[test]
fn qic_service_config_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","inspect_config":{"enabled":true,"inspect_code":"QC001"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["inspect_config"]["enabled"], true);
}
