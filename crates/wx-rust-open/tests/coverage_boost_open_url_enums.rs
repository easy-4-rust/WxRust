//! 覆盖率提升：`enums/url_ma_domain.rs` URL 常量函数全量覆盖。
//!
//! 对应 Java `me.chanjar.weixin.open.api` 下 9 个 Ma*/Minishop 子域服务
//! 接口中声明的 URL 常量（`WxOpenMaAuthService`/`WxOpenMaBasicService`/
//! `WxOpenMaEmbeddedService`/`WxOpenMaIcpService`/`WxOpenMaPrivacyService`/
//! `WxOpenMaShoppingOrdersService`/`WxOpenMinishopGoodsService`/
//! `WxOpenMinishopService`/`WxOpenMaAuthAndIcpService`）。
//!
//! 测试三层：
//! - SOURCE_PARITY：81 个 config 前缀拼接 URL 常量逐一对齐 Java 常量值
//!   （自定义 apiHostUrl 时以前缀替换，与 Java `apiHostUrl` 替换语义一致）；
//! - RUST_OBLIGATION：未配置 host 时回退 `API_DEFAULT_HOST_URL`
//!   （`https://api.weixin.qq.com`）的 `unwrap_or_else` 分支；
//! - VALUE_ADD：固定域名格式化串（`componentrebindadmin`/`qrconnect`/
//!   `connect_oauth2_authorize`）与 oauth2 查询串拼装、component_appid
//!   缺省回退空串分支。
//!
//! 纯函数测试，离线执行，无需网络与 MockServer。

use std::sync::Arc;

use wx_rust_open::config::WxOpenConfigStorage;
use wx_rust_open::config::r#impl::WxOpenDefaultConfig;
use wx_rust_open::enums::url_ma_domain;

/// 测试用自定义 apiHostUrl 前缀。
const HOST: &str = "https://mock.api.weixin.test";

/// 构建自定义 apiHostUrl 的配置（对应 Java `setApiHostUrl`）。
fn config_with_host() -> Arc<WxOpenDefaultConfig> {
    let mut config = WxOpenDefaultConfig::new();
    config.set_component_app_id("component_appid_01");
    config.set_api_host_url(HOST);
    Arc::new(config)
}

/// 未配置 apiHostUrl 的默认配置（覆盖 `url`/`host` 的回退分支）。
fn config_default_host() -> Arc<WxOpenDefaultConfig> {
    Arc::new(WxOpenDefaultConfig::new())
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxOpenMaAuthService（认证/年审）（共 5 个 URL 常量）
// 对应 Java: 子域服务接口中的 URL 常量 + apiHostUrl 替换语义
// ═══════════════════════════════════════════════════════════════════

/// WxOpenMaAuthService（认证/年审）：全部 URL 常量 = apiHostUrl 前缀 + 路径（逐常量镜像 Java 值）。
#[test]
fn url_section_1() {
    let cfg = config_with_host();
    assert_eq!(
        url_ma_domain::ma_auth_submit_url(cfg.as_ref()),
        format!("{HOST}/wxa/sec/wxaauth"),
    );
    assert_eq!(
        url_ma_domain::ma_auth_query_url(cfg.as_ref()),
        format!("{HOST}/wxa/sec/queryauth"),
    );
    assert_eq!(
        url_ma_domain::ma_auth_upload_url(cfg.as_ref()),
        format!("{HOST}/wxa/sec/uploadauthmaterial"),
    );
    assert_eq!(
        url_ma_domain::ma_auth_resubmit_url(cfg.as_ref()),
        format!("{HOST}/wxa/sec/reauth"),
    );
    assert_eq!(
        url_ma_domain::ma_auth_identity_url(cfg.as_ref()),
        format!("{HOST}/wxa/sec/authidentitytree"),
    );
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxOpenMaBasicService（基础信息/类目）（共 15 个 URL 常量）
// 对应 Java: 子域服务接口中的 URL 常量 + apiHostUrl 替换语义
// ═══════════════════════════════════════════════════════════════════

/// WxOpenMaBasicService（基础信息/类目）：全部 URL 常量 = apiHostUrl 前缀 + 路径（逐常量镜像 Java 值）。
#[test]
fn url_section_2() {
    let cfg = config_with_host();
    assert_eq!(
        url_ma_domain::ma_get_account_basic_info_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/account/getaccountbasicinfo"),
    );
    assert_eq!(
        url_ma_domain::ma_set_nickname_url(cfg.as_ref()),
        format!("{HOST}/wxa/setnickname"),
    );
    assert_eq!(
        url_ma_domain::ma_query_nickname_url(cfg.as_ref()),
        format!("{HOST}/wxa/api_wxa_querynickname"),
    );
    assert_eq!(
        url_ma_domain::ma_check_wx_verify_nickname_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/wxverify/checkwxverifynickname"),
    );
    assert_eq!(
        url_ma_domain::ma_modify_head_image_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/account/modifyheadimage"),
    );
    assert_eq!(
        url_ma_domain::ma_modify_signature_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/account/modifysignature"),
    );
    assert_eq!(
        url_ma_domain::ma_component_rebind_admin_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/account/componentrebindadmin"),
    );
    assert_eq!(
        url_ma_domain::ma_get_all_categories_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/wxopen/getallcategories"),
    );
    assert_eq!(
        url_ma_domain::ma_get_all_categories_by_type_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/wxopen/getcategoriesbytype"),
    );
    assert_eq!(
        url_ma_domain::ma_add_category_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/wxopen/addcategory"),
    );
    assert_eq!(
        url_ma_domain::ma_delete_category_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/wxopen/deletecategory"),
    );
    assert_eq!(
        url_ma_domain::ma_get_category_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/wxopen/getcategory"),
    );
    assert_eq!(
        url_ma_domain::ma_modify_category_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/wxopen/modifycategory"),
    );
    assert_eq!(
        url_ma_domain::ma_get_all_category_name_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/wxopen/getallcategorynamelist"),
    );
    assert_eq!(
        url_ma_domain::ma_get_order_path_info_url(cfg.as_ref()),
        format!("{HOST}/wxa/security/getorderpathinfo"),
    );
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxOpenMaEmbeddedService（半屏小程序）（共 6 个 URL 常量）
// 对应 Java: 子域服务接口中的 URL 常量 + apiHostUrl 替换语义
// ═══════════════════════════════════════════════════════════════════

/// WxOpenMaEmbeddedService（半屏小程序）：全部 URL 常量 = apiHostUrl 前缀 + 路径（逐常量镜像 Java 值）。
#[test]
fn url_section_3() {
    let cfg = config_with_host();
    assert_eq!(
        url_ma_domain::ma_embedded_add_url(cfg.as_ref()),
        format!("{HOST}/wxaapi/wxaembedded/add_embedded"),
    );
    assert_eq!(
        url_ma_domain::ma_embedded_del_url(cfg.as_ref()),
        format!("{HOST}/wxaapi/wxaembedded/del_embedded"),
    );
    assert_eq!(
        url_ma_domain::ma_embedded_get_list_url(cfg.as_ref()),
        format!("{HOST}/wxaapi/wxaembedded/get_list"),
    );
    assert_eq!(
        url_ma_domain::ma_embedded_del_authorize_url(cfg.as_ref()),
        format!("{HOST}/wxaapi/wxaembedded/del_authorize"),
    );
    assert_eq!(
        url_ma_domain::ma_embedded_get_own_list_url(cfg.as_ref()),
        format!("{HOST}/wxaapi/wxaembedded/get_own_list"),
    );
    assert_eq!(
        url_ma_domain::ma_embedded_set_authorize_url(cfg.as_ref()),
        format!("{HOST}/wxaapi/wxaembedded/set_authorize"),
    );
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxOpenMaIcpService（备案）（共 16 个 URL 常量）
// 对应 Java: 子域服务接口中的 URL 常量 + apiHostUrl 替换语义
// ═══════════════════════════════════════════════════════════════════

/// WxOpenMaIcpService（备案）：全部 URL 常量 = apiHostUrl 前缀 + 路径（逐常量镜像 Java 值）。
#[test]
fn url_section_4() {
    let cfg = config_with_host();
    assert_eq!(
        url_ma_domain::ma_icp_query_verify_task_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/query_icp_verifytask"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_create_verify_task_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/create_icp_verifytask"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_upload_media_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/upload_icp_media"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_cancel_apply_filing_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/cancel_apply_icp_filing"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_apply_filing_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/apply_icp_filing"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_cancel_filing_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/cancel_icp_filing"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_get_entrance_info_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/get_icp_entrance_info"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_get_online_order_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/get_online_icp_order"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_query_service_content_types_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/query_icp_service_content_types"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_query_certificate_types_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/query_icp_certificate_types"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_query_district_code_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/query_icp_district_code"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_query_nrlx_types_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/query_icp_nrlx_types"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_query_subject_types_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/query_icp_subject_types"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_get_media_url(cfg.as_ref()),
        format!("{HOST}/wxa/icp/get_icp_media"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_submit_auth_and_icp_url(cfg.as_ref()),
        format!("{HOST}/wxa/sec/submit_auth_and_icp"),
    );
    assert_eq!(
        url_ma_domain::ma_icp_query_auth_and_icp_url(cfg.as_ref()),
        format!("{HOST}/wxa/sec/query_auth_and_icp"),
    );
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxOpenMaPrivacyService（隐私指引）（共 5 个 URL 常量）
// 对应 Java: 子域服务接口中的 URL 常量 + apiHostUrl 替换语义
// ═══════════════════════════════════════════════════════════════════

/// WxOpenMaPrivacyService（隐私指引）：全部 URL 常量 = apiHostUrl 前缀 + 路径（逐常量镜像 Java 值）。
#[test]
fn url_section_5() {
    let cfg = config_with_host();
    assert_eq!(
        url_ma_domain::ma_privacy_set_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/component/setprivacysetting"),
    );
    assert_eq!(
        url_ma_domain::ma_privacy_get_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/component/getprivacysetting"),
    );
    assert_eq!(
        url_ma_domain::ma_privacy_upload_file_url(cfg.as_ref()),
        format!("{HOST}/cgi-bin/component/uploadprivacyextfile"),
    );
    assert_eq!(
        url_ma_domain::ma_privacy_get_interface_url(cfg.as_ref()),
        format!("{HOST}/wxa/security/get_privacy_interface"),
    );
    assert_eq!(
        url_ma_domain::ma_privacy_apply_interface_url(cfg.as_ref()),
        format!("{HOST}/wxa/security/apply_privacy_interface"),
    );
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxOpenMaShoppingOrdersService（购物订单）（共 7 个 URL 常量）
// 对应 Java: 子域服务接口中的 URL 常量 + apiHostUrl 替换语义
// ═══════════════════════════════════════════════════════════════════

/// WxOpenMaShoppingOrdersService（购物订单）：全部 URL 常量 = apiHostUrl 前缀 + 路径（逐常量镜像 Java 值）。
#[test]
fn url_section_6() {
    let cfg = config_with_host();
    assert_eq!(
        url_ma_domain::ma_orders_upload_shopping_info_url(cfg.as_ref()),
        format!("{HOST}/user-order/orders"),
    );
    assert_eq!(
        url_ma_domain::ma_orders_upload_shipping_info_url(cfg.as_ref()),
        format!("{HOST}/user-order/orders/shippings"),
    );
    assert_eq!(
        url_ma_domain::ma_orders_upload_combined_shopping_info_url(cfg.as_ref()),
        format!("{HOST}/user-order/combine-orders"),
    );
    assert_eq!(
        url_ma_domain::ma_orders_upload_combined_shipping_info_url(cfg.as_ref()),
        format!("{HOST}/user-order/combine-orders/shippings"),
    );
    assert_eq!(
        url_ma_domain::ma_orders_open_permission_url(cfg.as_ref()),
        format!("{HOST}/user-order/orders-permission/open"),
    );
    assert_eq!(
        url_ma_domain::ma_orders_confirm_permission_url(cfg.as_ref()),
        format!("{HOST}/user-order/orders-permission/confirm"),
    );
    assert_eq!(
        url_ma_domain::ma_orders_verify_upload_url(cfg.as_ref()),
        format!("{HOST}/user-order/shoppinginfo/verify"),
    );
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxOpenMinishopGoodsService（小商城商品）（共 18 个 URL 常量）
// 对应 Java: 子域服务接口中的 URL 常量 + apiHostUrl 替换语义
// ═══════════════════════════════════════════════════════════════════

/// WxOpenMinishopGoodsService（小商城商品）：全部 URL 常量 = apiHostUrl 前缀 + 路径（逐常量镜像 Java 值）。
#[test]
fn url_section_7() {
    let cfg = config_with_host();
    assert_eq!(
        url_ma_domain::minishop_goods_cat_url(cfg.as_ref()),
        format!("{HOST}/product/category/get"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_add_spu_url(cfg.as_ref()),
        format!("{HOST}/product/spu/add"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_del_spu_url(cfg.as_ref()),
        format!("{HOST}/product/spu/del"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_get_spu_url(cfg.as_ref()),
        format!("{HOST}/product/spu/get"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_get_list_spu_url(cfg.as_ref()),
        format!("{HOST}/product/spu/get_list"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_search_spu_url(cfg.as_ref()),
        format!("{HOST}/product/spu/search"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_update_spu_url(cfg.as_ref()),
        format!("{HOST}/product/spu/update"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_listing_spu_url(cfg.as_ref()),
        format!("{HOST}/product/spu/listing"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_delisting_spu_url(cfg.as_ref()),
        format!("{HOST}/product/spu/delisting"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_add_sku_url(cfg.as_ref()),
        format!("{HOST}/product/sku/add"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_batch_add_sku_url(cfg.as_ref()),
        format!("{HOST}/product/sku/batch_add"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_del_sku_url(cfg.as_ref()),
        format!("{HOST}/product/sku/del"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_get_sku_url(cfg.as_ref()),
        format!("{HOST}/product/sku/get"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_get_list_sku_url(cfg.as_ref()),
        format!("{HOST}/product/sku/get_list"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_update_sku_url(cfg.as_ref()),
        format!("{HOST}/product/sku/update"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_update_sku_price_url(cfg.as_ref()),
        format!("{HOST}/product/sku/update_price"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_update_stock_url(cfg.as_ref()),
        format!("{HOST}/product/stock/update"),
    );
    assert_eq!(
        url_ma_domain::minishop_goods_get_stock_url(cfg.as_ref()),
        format!("{HOST}/product/stock/get"),
    );
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxOpenMinishopService（小商店开店）（共 7 个 URL 常量）
// 对应 Java: 子域服务接口中的 URL 常量 + apiHostUrl 替换语义
// ═══════════════════════════════════════════════════════════════════

/// WxOpenMinishopService（小商店开店）：全部 URL 常量 = apiHostUrl 前缀 + 路径（逐常量镜像 Java 值）。
#[test]
fn url_section_8() {
    let cfg = config_with_host();
    assert_eq!(
        url_ma_domain::minishop_submit_merchant_info_url(cfg.as_ref()),
        format!("{HOST}/product/register/submit_merchantinfo"),
    );
    assert_eq!(
        url_ma_domain::minishop_submit_basic_info_url(cfg.as_ref()),
        format!("{HOST}/product/register/submit_basicinfo"),
    );
    assert_eq!(
        url_ma_domain::minishop_upload_img_url(cfg.as_ref()),
        format!("{HOST}/product/img/upload"),
    );
    assert_eq!(
        url_ma_domain::minishop_get_category_url(cfg.as_ref()),
        format!("{HOST}/product/category/get"),
    );
    assert_eq!(
        url_ma_domain::minishop_get_brands_url(cfg.as_ref()),
        format!("{HOST}/product/brand/get"),
    );
    assert_eq!(
        url_ma_domain::minishop_get_delivery_url(cfg.as_ref()),
        format!("{HOST}/product/delivery/get_freight_template"),
    );
    assert_eq!(
        url_ma_domain::minishop_get_shop_cat_url(cfg.as_ref()),
        format!("{HOST}/product/store/get_shopcat"),
    );
}

// ═══════════════════════════════════════════════════════════════════
// SOURCE_PARITY：WxOpenMaAuthAndIcpService（认证及备案）（共 2 个 URL 常量）
// 对应 Java: 子域服务接口中的 URL 常量 + apiHostUrl 替换语义
// ═══════════════════════════════════════════════════════════════════

/// WxOpenMaAuthAndIcpService（认证及备案）：全部 URL 常量 = apiHostUrl 前缀 + 路径（逐常量镜像 Java 值）。
#[test]
fn url_section_9() {
    let cfg = config_with_host();
    assert_eq!(
        url_ma_domain::ma_auth_and_icp_query_url(cfg.as_ref()),
        format!("{HOST}/wxa/sec/query_auth_and_icp"),
    );
    assert_eq!(
        url_ma_domain::ma_auth_and_icp_submit_url(cfg.as_ref()),
        format!("{HOST}/wxa/sec/submit_auth_and_icp"),
    );
}

// ═══════════════════════════════════════════════════════════════════
// RUST_OBLIGATION：默认域名回退分支
// ═══════════════════════════════════════════════════════════════════

/// 未配置 host 时回退 `API_DEFAULT_HOST_URL`
/// （对应 Java 未设置 apiHostUrl 时使用默认域名）。
#[test]
fn url_default_host_fallback() {
    let cfg = config_default_host();
    assert_eq!(
        url_ma_domain::ma_auth_submit_url(cfg.as_ref()),
        "https://api.weixin.qq.com/wxa/sec/wxaauth"
    );
    assert_eq!(
        url_ma_domain::oauth2_access_token_url(cfg.as_ref(), "appid_1", "secret_1", "code_1"),
        "https://api.weixin.qq.com/sns/oauth2/access_token?appid=appid_1&secret=secret_1&code=code_1&grant_type=authorization_code"
    );
}

// ═══════════════════════════════════════════════════════════════════
// VALUE_ADD：固定域名格式化串 + oauth2 查询串拼装
// ═══════════════════════════════════════════════════════════════════

/// `componentrebindadmin` 固定 mp 域名格式化串
/// （对应 Java `WxOpenMaBasicService.URL_COMPONENT_REBIND_ADMIN`）。
#[test]
fn component_rebind_admin_url_fixed_format() {
    let url = url_ma_domain::component_rebind_admin_url(
        "appid_1",
        "component_appid_1",
        "https%3A%2F%2Fexample.com%2Fcb",
    );
    assert_eq!(
        url,
        "https://mp.weixin.qq.com/wxopen/componentrebindadmin?appid=appid_1&component_appid=component_appid_1&redirect_uri=https%3A%2F%2Fexample.com%2Fcb"
    );
}

/// `qrconnect` 固定 open 域名格式化串
/// （对应 Java `WxMpApiUrl.Other.QRCONNECT_URL`）。
#[test]
fn qrconnect_url_fixed_format() {
    let url = url_ma_domain::qrconnect_url(
        "appid_1",
        "https%3A%2F%2Fexample.com%2Fqr",
        "snsapi_login",
        "st_1",
    );
    assert_eq!(
        url,
        "https://open.weixin.qq.com/connect/qrconnect?appid=appid_1&redirect_uri=https%3A%2F%2Fexample.com%2Fqr&response_type=code&scope=snsapi_login&state=st_1#wechat_redirect"
    );
}

/// `connect_oauth2_authorize` 固定 open 域名格式化串
/// （对应 Java `WxOpenComponentService.CONNECT_OAUTH2_AUTHORIZE_URL`）。
#[test]
fn connect_oauth2_authorize_url_fixed_format() {
    let url = url_ma_domain::connect_oauth2_authorize_url(
        "appid_1",
        "https%3A%2F%2Fexample.com%2Fau",
        "snsapi_userinfo",
        "st_2",
        "component_appid_1",
    );
    assert_eq!(
        url,
        "https://open.weixin.qq.com/connect/oauth2/authorize?appid=appid_1&redirect_uri=https%3A%2F%2Fexample.com%2Fau&response_type=code&scope=snsapi_userinfo&state=st_2&component_appid=component_appid_1#wechat_redirect"
    );
}

/// oauth2 查询串拼装（refresh/userinfo/validate/component）
/// （对应 Java `WxMpApiUrl.OAuth2` / `WxOpenComponentService.OAUTH2_ACCESS_TOKEN_URL`）。
#[test]
fn oauth2_query_string_assembly() {
    let cfg = config_with_host();
    assert_eq!(
        url_ma_domain::oauth2_refresh_token_url(cfg.as_ref(), "appid_1", "refresh_1"),
        format!(
            "{HOST}/sns/oauth2/refresh_token?appid=appid_1&grant_type=refresh_token&refresh_token=refresh_1"
        )
    );
    assert_eq!(
        url_ma_domain::oauth2_userinfo_url(cfg.as_ref(), "tok_1", "openid_1", "zh_CN"),
        format!("{HOST}/sns/userinfo?access_token=tok_1&openid=openid_1&lang=zh_CN")
    );
    assert_eq!(
        url_ma_domain::oauth2_validate_token_url(cfg.as_ref(), "tok_1", "openid_1"),
        format!("{HOST}/sns/auth?access_token=tok_1&openid=openid_1")
    );
    // component 链路：component_appid 取配置
    assert_eq!(
        url_ma_domain::oauth2_component_access_token_url(cfg.as_ref(), "appid_1", "code_1"),
        format!(
            "{HOST}/sns/oauth2/component/access_token?appid=appid_1&code=code_1&grant_type=authorization_code&component_appid=component_appid_01"
        )
    );
    // component_appid 未配置 → 空串（`unwrap_or_default` 分支）
    let plain = config_default_host();
    assert_eq!(
        url_ma_domain::oauth2_component_access_token_url(plain.as_ref(), "appid_1", "code_1"),
        "https://api.weixin.qq.com/sns/oauth2/component/access_token?appid=appid_1&code=code_1&grant_type=authorization_code&component_appid="
    );
}
