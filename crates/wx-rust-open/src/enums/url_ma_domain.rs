//! Ma*/Minishop 子域服务接口地址。
//!
//! 对应 Java `me.chanjar.weixin.open.api` 下 9 个子域服务接口
//! （`WxOpenMaAuthService`/`WxOpenMaBasicService`/`WxOpenMaEmbeddedService`/
//! `WxOpenMaIcpService`/`WxOpenMaPrivacyService`/`WxOpenMaShoppingOrdersService`/
//! `WxOpenMinishopGoodsService`/`WxOpenMinishopService`/
//! `WxOpenMaAuthAndIcpService`）中声明的 URL 常量。
//!
//! 函数风格与 [`crate::enums::url_core`] 一致：`WxOpenHostConfig.api_host`
//! 前缀 + 路径，支持自定义 apiHostUrl（Java `apiHostUrl` 替换语义一致）。
//! `componentrebindadmin` 与 oauth2 授权链接为 `mp.weixin.qq.com` /
//! `open.weixin.qq.com` 域名的固定格式化串（Java 常量即写死完整地址，
//! 无 apiHost 替换，原样镜像）。

use crate::config::{API_DEFAULT_HOST_URL, WxOpenConfigStorage};

/// 生成完整接口地址：域名前缀 + 路径。
fn url(config: &dyn WxOpenConfigStorage, path: &str) -> String {
    let host = config
        .wx_open_host_config()
        .map(|h| h.api_host)
        .unwrap_or_else(|| API_DEFAULT_HOST_URL.to_string());
    format!("{host}{path}")
}

// ---------------------------------------------------------------------------
// WxOpenMaAuthService（小程序认证/年审）
// ---------------------------------------------------------------------------

/// 小程序认证（提审）（对应 Java `WxOpenMaAuthService.OPEN_MA_AUTH_SUBMIT`）。
pub fn ma_auth_submit_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/sec/wxaauth")
}

/// 小程序认证任务进度查询（对应 Java `OPEN_MA_AUTH_QUERY`）。
pub fn ma_auth_query_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/sec/queryauth")
}

/// 小程序认证上传补充材料（对应 Java `OPEN_MA_AUTH_UPLOAD`）。
pub fn ma_auth_upload_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/sec/uploadauthmaterial")
}

/// 小程序认证重新提审（对应 Java `OPEN_MA_AUTH_RESUBMIT`）。
pub fn ma_auth_resubmit_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/sec/reauth")
}

/// 查询个人认证身份选项列表（对应 Java `OPEN_MA_AUTH_IDENTITY`）。
pub fn ma_auth_identity_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/sec/authidentitytree")
}

// ---------------------------------------------------------------------------
// WxOpenMaBasicService（小程序基础信息）
// ---------------------------------------------------------------------------

/// 获取帐号基本信息（对应 Java `WxOpenMaBasicService.OPEN_GET_ACCOUNT_BASIC_INFO`）。
pub fn ma_get_account_basic_info_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/account/getaccountbasicinfo")
}

/// 小程序名称设置及改名（对应 Java `OPEN_SET_NICKNAME`）。
pub fn ma_set_nickname_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/setnickname")
}

/// 小程序改名审核状态查询（对应 Java `OPEN_API_WXA_QUERYNICKNAME`）。
pub fn ma_query_nickname_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/api_wxa_querynickname")
}

/// 微信认证名称检测（对应 Java `OPEN_CHECK_WX_VERIFY_NICKNAME`）。
pub fn ma_check_wx_verify_nickname_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/wxverify/checkwxverifynickname")
}

/// 修改头像（对应 Java `OPEN_MODIFY_HEADIMAGE`）。
pub fn ma_modify_head_image_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/account/modifyheadimage")
}

/// 修改功能介绍（对应 Java `OPEN_MODIFY_SIGNATURE`）。
pub fn ma_modify_signature_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/account/modifysignature")
}

/// 换绑小程序管理员（对应 Java `OPEN_COMPONENT_REBIND_ADMIN`）。
pub fn ma_component_rebind_admin_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/account/componentrebindadmin")
}

/// 换绑管理员 URL（对应 Java `URL_COMPONENT_REBIND_ADMIN`，
/// `mp.weixin.qq.com` 固定域名 + `%s` 格式化：appid/component_appid/
/// redirect_uri 须由调用方预编码）。
pub fn component_rebind_admin_url(
    app_id: &str,
    component_app_id: &str,
    encoded_redirect_uri: &str,
) -> String {
    format!(
        "https://mp.weixin.qq.com/wxopen/componentrebindadmin?appid={}&component_appid={}&redirect_uri={}",
        app_id, component_app_id, encoded_redirect_uri
    )
}

/// 获取账号可以设置的所有类目（对应 Java `OPEN_GET_ALL_CATEGORIES`）。
pub fn ma_get_all_categories_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/wxopen/getallcategories")
}

/// 获取不同类型主体可设置的类目（对应 Java `OPEN_GET_ALL_CATEGORIES_BY_TYPE`）。
pub fn ma_get_all_categories_by_type_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/wxopen/getcategoriesbytype")
}

/// 添加类目（对应 Java `OPEN_ADD_CATEGORY`）。
pub fn ma_add_category_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/wxopen/addcategory")
}

/// 删除类目（对应 Java `OPEN_DELETE_CATEGORY`）。
pub fn ma_delete_category_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/wxopen/deletecategory")
}

/// 获取账号已经设置的所有类目（对应 Java `OPEN_GET_CATEGORY`）。
pub fn ma_get_category_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/wxopen/getcategory")
}

/// 修改类目（对应 Java `OPEN_MODIFY_CATEGORY`）。
pub fn ma_modify_category_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/wxopen/modifycategory")
}

/// 获取类目名称信息（对应 Java `OPEN_GET_ALL_CATEGORY_NAME`）。
pub fn ma_get_all_category_name_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/wxopen/getallcategorynamelist")
}

/// 获取订单页 path 信息（对应 Java `OPEN_GET_ORDER_PATH_INFO`）。
pub fn ma_get_order_path_info_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/security/getorderpathinfo")
}

// ---------------------------------------------------------------------------
// WxOpenMaEmbeddedService（半屏小程序管理）
// ---------------------------------------------------------------------------

/// 添加半屏小程序（对应 Java `WxOpenMaEmbeddedService.API_ADD_EMBEDDED`）。
pub fn ma_embedded_add_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxaapi/wxaembedded/add_embedded")
}

/// 删除半屏小程序（对应 Java `API_DELETE_EMBEDDED`）。
pub fn ma_embedded_del_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxaapi/wxaembedded/del_embedded")
}

/// 获取半屏小程序调用列表（对应 Java `API_GET_EMBEDDED_LIST`）。
pub fn ma_embedded_get_list_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxaapi/wxaembedded/get_list")
}

/// 取消授权小程序（对应 Java `API_DELETE_AUTHORIZED_EMBEDDED`）。
pub fn ma_embedded_del_authorize_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxaapi/wxaembedded/del_authorize")
}

/// 获取半屏小程序授权列表（对应 Java `API_GET_OWN_LIST`）。
pub fn ma_embedded_get_own_list_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxaapi/wxaembedded/get_own_list")
}

/// 设置授权方式（对应 Java `API_SET_AUTHORIZED_EMBEDDED`）。
pub fn ma_embedded_set_authorize_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxaapi/wxaembedded/set_authorize")
}

// ---------------------------------------------------------------------------
// WxOpenMaIcpService（小程序备案）
// ---------------------------------------------------------------------------

/// 查询人脸核身任务状态（对应 Java `WxOpenMaIcpService.QUERY_ICP_VERIFY_TASK`）。
pub fn ma_icp_query_verify_task_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/query_icp_verifytask")
}

/// 发起小程序管理员人脸核身（对应 Java `CREATE_ICP_VERIFY_TASK`）。
pub fn ma_icp_create_verify_task_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/create_icp_verifytask")
}

/// 上传小程序备案媒体材料（对应 Java `UPLOAD_ICP_MEDIA`）。
pub fn ma_icp_upload_media_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/upload_icp_media")
}

/// 撤回小程序备案申请（对应 Java `CANCEL_APPLY_ICP_FILING`）。
pub fn ma_icp_cancel_apply_filing_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/cancel_apply_icp_filing")
}

/// 申请小程序备案（对应 Java `APPLY_ICP_FILING`）。
pub fn ma_icp_apply_filing_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/apply_icp_filing")
}

/// 注销小程序备案（对应 Java `CANCEL_ICP_FILING`）。
pub fn ma_icp_cancel_filing_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/cancel_icp_filing")
}

/// 获取小程序备案状态及驳回原因（对应 Java `GET_ICP_ENTRANCE_INFO`）。
pub fn ma_icp_get_entrance_info_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/get_icp_entrance_info")
}

/// 获取小程序已备案详情（对应 Java `GET_ONLINE_ICP_ORDER`）。
pub fn ma_icp_get_online_order_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/get_online_icp_order")
}

/// 获取小程序服务内容类型（对应 Java `QUERY_ICP_SERVICE_CONTENT_TYPES`）。
pub fn ma_icp_query_service_content_types_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/query_icp_service_content_types")
}

/// 获取证件类型（对应 Java `QUERY_ICP_CERTIFICATE_TYPES`）。
pub fn ma_icp_query_certificate_types_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/query_icp_certificate_types")
}

/// 获取区域信息（对应 Java `QUERY_ICP_DISTRICT_CODE`）。
pub fn ma_icp_query_district_code_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/query_icp_district_code")
}

/// 获取前置审批项类型（对应 Java `QUERY_ICP_NRLX_TYPES`）。
pub fn ma_icp_query_nrlx_types_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/query_icp_nrlx_types")
}

/// 获取单位性质（对应 Java `QUERY_ICP_SUBJECT_TYPES`）。
pub fn ma_icp_query_subject_types_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/query_icp_subject_types")
}

/// 获取小程序备案媒体材料（对应 Java `GET_ICP_MEDIA`）。
pub fn ma_icp_get_media_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/icp/get_icp_media")
}

/// 申请小程序认证及备案（对应 Java `SUBMIT_AUTH_AND_ICP`）。
pub fn ma_icp_submit_auth_and_icp_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/sec/submit_auth_and_icp")
}

/// 查询小程序认证及备案进度（对应 Java `QUERY_AUTH_AND_ICP`）。
pub fn ma_icp_query_auth_and_icp_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/sec/query_auth_and_icp")
}

// ---------------------------------------------------------------------------
// WxOpenMaPrivacyService（小程序用户隐私保护指引）
// ---------------------------------------------------------------------------

/// 设置小程序用户隐私保护指引（对应 Java
/// `WxOpenMaPrivacyService.OPEN_SET_PRIVACY_SETTING`）。
pub fn ma_privacy_set_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/component/setprivacysetting")
}

/// 查询小程序用户隐私保护指引（对应 Java `OPEN_GET_PRIVACY_SETTING`）。
pub fn ma_privacy_get_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/component/getprivacysetting")
}

/// 上传小程序用户隐私保护指引文件（对应 Java `OPEN_UPLOAD_PRIVACY_FILE`）。
pub fn ma_privacy_upload_file_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/cgi-bin/component/uploadprivacyextfile")
}

/// 获取隐私接口列表（对应 Java `GET_PRIVATE_INTERFACE`）。
pub fn ma_privacy_get_interface_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/security/get_privacy_interface")
}

/// 申请隐私接口（对应 Java `APPLY_PRIVATE_INTERFACE`）。
pub fn ma_privacy_apply_interface_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/security/apply_privacy_interface")
}

// ---------------------------------------------------------------------------
// WxOpenMaShoppingOrdersService（购物订单）
// ---------------------------------------------------------------------------

/// 上传购物详情（对应 Java `WxOpenMaShoppingOrdersService.UPLOAD_SHOPPING_INFO`）。
pub fn ma_orders_upload_shopping_info_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/user-order/orders")
}

/// 上传物流信息（对应 Java `UPLOAD_SHIPPING_INFO`）。
pub fn ma_orders_upload_shipping_info_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/user-order/orders/shippings")
}

/// 上传合单购物详情（对应 Java `UPLOAD_COMBINED_SHOPPING_INFO`）。
pub fn ma_orders_upload_combined_shopping_info_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/user-order/combine-orders")
}

/// 上传合单物流信息（对应 Java `UPLOAD_COMBINED_SHIPPING_INFO`）。
pub fn ma_orders_upload_combined_shipping_info_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/user-order/combine-orders/shippings")
}

/// 开通购物订单产品权限（对应 Java `OPEN_SHOPPING_ORDER_PRODUCT_PERMISSION`）。
pub fn ma_orders_open_permission_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/user-order/orders-permission/open")
}

/// 提交购物订单接入审核（对应 Java `CONFIRM_PRODUCT_PERMISSION`）。
pub fn ma_orders_confirm_permission_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/user-order/orders-permission/confirm")
}

/// 验证购物订单上传结果（对应 Java `SHOPPING_INFO_VERIFY_UPLOAD_RESULT`）。
pub fn ma_orders_verify_upload_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/user-order/shoppinginfo/verify")
}

// ---------------------------------------------------------------------------
// WxOpenMinishopGoodsService（微信小商城商品）
// ---------------------------------------------------------------------------

/// 获取类目详情（对应 Java `WxOpenMinishopGoodsService.getMinishopGoodsCatUrl`）。
pub fn minishop_goods_cat_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/category/get")
}

/// 添加商品 SPU（对应 Java `addMinishopGoodsSPUUrl`）。
pub fn minishop_goods_add_spu_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/spu/add")
}

/// 删除商品 SPU（对应 Java `delMinishopGoodsSPUUrl`）。
pub fn minishop_goods_del_spu_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/spu/del")
}

/// 获取商品 SPU（对应 Java `getMinishopGoodsSPUUrl`）。
pub fn minishop_goods_get_spu_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/spu/get")
}

/// 获取商品 SPU 列表（对应 Java `getListMinishopGoodsSPUURL`）。
pub fn minishop_goods_get_list_spu_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/spu/get_list")
}

/// 搜索商品 SPU（对应 Java `searchMinishopGoodsSPUURL`）。
pub fn minishop_goods_search_spu_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/spu/search")
}

/// 更新商品 SPU（对应 Java `updateMinishopGoodsSPUUrl`）。
pub fn minishop_goods_update_spu_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/spu/update")
}

/// 上架商品 SPU（对应 Java `listingMinishopGoodsSPUUrl`）。
pub fn minishop_goods_listing_spu_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/spu/listing")
}

/// 下架商品 SPU（对应 Java `delistingMinishopGoodsSPUUrl`）。
pub fn minishop_goods_delisting_spu_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/spu/delisting")
}

/// 添加 SKU（对应 Java `addMinishopGoodsSKUUrl`）。
pub fn minishop_goods_add_sku_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/sku/add")
}

/// 批量添加 SKU（对应 Java `batchAddMinishopGoodsSKUUrl`）。
pub fn minishop_goods_batch_add_sku_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/sku/batch_add")
}

/// 删除 SKU（对应 Java `delMinishopGoodsSKUUrl`）。
pub fn minishop_goods_del_sku_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/sku/del")
}

/// 获取 SKU 信息（对应 Java `getMinishopGoodsSKUUrl`）。
pub fn minishop_goods_get_sku_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/sku/get")
}

/// 批量获取 SKU 信息（对应 Java `getListMinishopGoodsSKUUrl`）。
pub fn minishop_goods_get_list_sku_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/sku/get_list")
}

/// 更新 SKU（对应 Java `updateMinishopGoodsSKUUrl`）。
pub fn minishop_goods_update_sku_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/sku/update")
}

/// 更新 SKU 价格（对应 Java `updatePriceMinishopGoodsSKUUrl`）。
pub fn minishop_goods_update_sku_price_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/sku/update_price")
}

/// 更新库存（对应 Java `updateStockMinishopGoodsSKUUrl`）。
pub fn minishop_goods_update_stock_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/stock/update")
}

/// 获取库存（对应 Java `getStockMinishopGoodsSKUUrl`）。
pub fn minishop_goods_get_stock_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/stock/get")
}

// ---------------------------------------------------------------------------
// WxOpenMinishopService（微信小商店开店）
// ---------------------------------------------------------------------------

/// 提交小商店商户信息（对应 Java `WxOpenMinishopService.submitMerchantInfoUrl`）。
pub fn minishop_submit_merchant_info_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/register/submit_merchantinfo")
}

/// 提交小商店基础信息（对应 Java `submitBasicInfoUrl`）。
pub fn minishop_submit_basic_info_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/register/submit_basicinfo")
}

/// 上传小商店图片（对应 Java `UPLOAD_IMG_MINISHOP_FILE_URL`）。
pub fn minishop_upload_img_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/img/upload")
}

/// 获取小商店类目（对应 Java `getCategoryUrl`）。
pub fn minishop_get_category_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/category/get")
}

/// 获取小商店品牌（对应 Java `getBrandsUrl`）。
pub fn minishop_get_brands_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/brand/get")
}

/// 获取运费模版（对应 Java `getDeliveryUrl`）。
pub fn minishop_get_delivery_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/delivery/get_freight_template")
}

/// 获取店铺的商品分类（对应 Java `getShopCatUrl`）。
pub fn minishop_get_shop_cat_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/product/store/get_shopcat")
}

// ---------------------------------------------------------------------------
// WxOpenMaAuthAndIcpService（小程序认证及备案）
// ---------------------------------------------------------------------------

/// 查询小程序认证及备案进度（对应 Java
/// `WxOpenMaAuthAndIcpService.QUERY_AUTH_AND_ICP`）。
pub fn ma_auth_and_icp_query_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/sec/query_auth_and_icp")
}

/// 提交小程序认证及备案信息（对应 Java `SUBMIT_AUTH_AND_ICP`）。
pub fn ma_auth_and_icp_submit_url(config: &dyn WxOpenConfigStorage) -> String {
    url(config, "/wxa/sec/submit_auth_and_icp")
}

// ---------------------------------------------------------------------------
// OAuth2（WxOpenOAuth2ServiceImpl，对应 Java
// `me.chanjar.weixin.mp.enums.WxMpApiUrl.OAuth2` / `Other.QRCONNECT_URL`；
// 普通 appid/secret 换取，非 component 链路）
// ---------------------------------------------------------------------------

/// 用 code 换取 oauth2 access token（对应 Java
/// `WxMpApiUrl.OAuth2.OAUTH2_ACCESS_TOKEN_URL`）。
pub fn oauth2_access_token_url(
    config: &dyn WxOpenConfigStorage,
    app_id: &str,
    app_secret: &str,
    code: &str,
) -> String {
    format!(
        "{}/sns/oauth2/access_token?appid={}&secret={}&code={}&grant_type=authorization_code",
        host(config),
        app_id,
        app_secret,
        code,
    )
}

/// 刷新 oauth2 access token（对应 Java
/// `WxMpApiUrl.OAuth2.OAUTH2_REFRESH_TOKEN_URL`）。
pub fn oauth2_refresh_token_url(
    config: &dyn WxOpenConfigStorage,
    app_id: &str,
    refresh_token: &str,
) -> String {
    format!(
        "{}/sns/oauth2/refresh_token?appid={}&grant_type=refresh_token&refresh_token={}",
        host(config),
        app_id,
        refresh_token,
    )
}

/// 用 oauth2 获取用户信息（对应 Java
/// `WxMpApiUrl.OAuth2.OAUTH2_USERINFO_URL`）。
pub fn oauth2_userinfo_url(
    config: &dyn WxOpenConfigStorage,
    access_token: &str,
    open_id: &str,
    lang: &str,
) -> String {
    format!(
        "{}/sns/userinfo?access_token={}&openid={}&lang={}",
        host(config),
        access_token,
        open_id,
        lang,
    )
}

/// 验证 oauth2 access token 是否有效（对应 Java
/// `WxMpApiUrl.OAuth2.OAUTH2_VALIDATE_TOKEN_URL`）。
pub fn oauth2_validate_token_url(
    config: &dyn WxOpenConfigStorage,
    access_token: &str,
    open_id: &str,
) -> String {
    format!(
        "{}/sns/auth?access_token={}&openid={}",
        host(config),
        access_token,
        open_id,
    )
}

/// 网站应用授权登录 URL（对应 Java `WxMpApiUrl.Other.QRCONNECT_URL`，
/// `open.weixin.qq.com` 固定域名；redirect_uri 须由调用方预编码，
/// state 已 trim）。
pub fn qrconnect_url(app_id: &str, encoded_redirect_uri: &str, scope: &str, state: &str) -> String {
    format!(
        "https://open.weixin.qq.com/connect/qrconnect?appid={}&redirect_uri={}&response_type=code&scope={}&state={}#wechat_redirect",
        app_id, encoded_redirect_uri, scope, state
    )
}

// ---------------------------------------------------------------------------
// OAuth2（WxOpenMpOAuth2ServiceImpl，对应 Java
// `WxOpenComponentService.OAUTH2_ACCESS_TOKEN_URL` /
// `CONNECT_OAUTH2_AUTHORIZE_URL`）
// ---------------------------------------------------------------------------

/// 第三方平台代公众号网页授权 code 换 token（对应 Java
/// `WxOpenComponentService.OAUTH2_ACCESS_TOKEN_URL`，`%s` 由调用方
/// format；Java 不编码 appid/code，镜像）。
pub fn oauth2_component_access_token_url(
    config: &dyn WxOpenConfigStorage,
    app_id: &str,
    code: &str,
) -> String {
    format!(
        "{}/sns/oauth2/component/access_token?appid={}&code={}&grant_type=authorization_code&component_appid={}",
        host(config),
        app_id,
        code,
        component_app_id(config),
    )
}

/// 网页授权链接（对应 Java `WxOpenComponentService.CONNECT_OAUTH2_AUTHORIZE_URL`，
/// `open.weixin.qq.com` 固定域名；redirect_uri 须由调用方预编码）。
pub fn connect_oauth2_authorize_url(
    app_id: &str,
    encoded_redirect_uri: &str,
    scope: &str,
    state: &str,
    component_app_id: &str,
) -> String {
    format!(
        "https://open.weixin.qq.com/connect/oauth2/authorize?appid={}&redirect_uri={}&response_type=code&scope={}&state={}&component_appid={}#wechat_redirect",
        app_id, encoded_redirect_uri, scope, state, component_app_id
    )
}

fn host(config: &dyn WxOpenConfigStorage) -> String {
    config
        .wx_open_host_config()
        .map(|h| h.api_host)
        .unwrap_or_else(|| API_DEFAULT_HOST_URL.to_string())
}

fn component_app_id(config: &dyn WxOpenConfigStorage) -> String {
    config.component_app_id().unwrap_or_default()
}
