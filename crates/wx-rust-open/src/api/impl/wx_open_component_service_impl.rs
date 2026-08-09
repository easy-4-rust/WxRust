//! 组件服务实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenComponentServiceImpl`：
//! 持有门面服务引用（Java 强引用构造 `new WxOpenComponentServiceImpl(this)`，
//! Rust 以 `Weak<dyn WxOpenService>` 打破循环），实现
//! component_access_token 双检锁刷新、带 token 注入的 get/post、
//! verify ticket 推送启动、预授权码与预授权链接，以及 Wave 2 补齐的
//! 授权方信息/选项/列表、代码模板、open 帐号、快速创建、minishop、tcb、
//! oauth2、服务器域名等全部接口方法（Wave 2 待定清单见各方法注释）。
//!
//! 镜像约定：
//! - Java `getWxOpenService().post(url, json)`（裸 post，URL 已带
//!   access_token 或无需 token）→ [`Self::bare_post`]（直调执行器，不做
//!   token 注入）；Java 组件层 `post(url, json)` → [`Self::post`]。
//! - minishop 入参 bean 的 Java `toJsonObject()` 均为手工拼装 snake_case
//!   JSON，与 Rust bean 的 serde（camelCase）不一致，故在
//!   [`minishop_json`] 中以 `serde_json::json!` 逐键镜像 Java 线格式。
//! - Java `WxOpenMinishopService.UPLOAD_IMG_MINISHOP_FILE_URL` 与接口常量
//!   `UPLOAD_IMAGE_URL` 同值，统一为 [`urls::upload_image_url`]。
//! - 未实现（待依赖接线）清单：`getWxMpServiceByAppid`/
//!   `getWxMaServiceByAppid`/`getWxFastMaServiceByAppid`/
//!   `getWxMinishopServiceByAppid`（恒 None）与依赖其 post 的
//!   `createOpenAccount`/`bindOpenAccount`/`unbindOpenAccount`/
//!   `getOpenAccount`（Err(-99)）；Java 恒 `return null` 的
//!   `minishopGetCouponList`/`minishopCommonPost` 以 `Ok(None)` 镜像。
//!
//! Wave 4 更新（代 mp/ma 桥接接线，对应本文件
//! [`WxOpenComponentServiceImpl`]）：
//! - `getWxMpServiceByAppid`/`getWxMaServiceByAppid`/
//!   `getWxFastMaServiceByAppid` 由恒 None 改为按 appId 双检锁缓存装配
//!   [`crate::api::r#impl::WxOpenMpService`] /
//!   [`crate::api::r#impl::WxOpenMaService`]（镜像 Java 静态
//!   ConcurrentHashMap + synchronized 双检锁；Rust 以实例级
//!   `Mutex<HashMap>` 表达），返回值仍为 `Arc<dyn Any + Send + Sync>`，
//!   调用方经 `downcast_mp_service`/`downcast_ma_service` 下转；
//! - `createOpenAccount`/`bindOpenAccount`/`unbindOpenAccount`/
//!   `getOpenAccount` 解锁：镜像 Java 私有方法 `openAccountServicePost`
//!   （switch appIdType：mp → 代 mp 服务 post；mini → 代 ma 服务 post；
//!   其余抛「appIdType类型异常」）；
//! - `getWxMinishopServiceByAppid` 接线（Wave 5）：双检锁缓存装配
//!   [`WxOpenMinishopServiceImpl`]（此前恒 None）。
//! - Ma*/Minishop 子域服务（Wave 5）见 `api/wx_open_ma_*_service.rs` /
//!   `api/impl/wx_open_ma_*_service_impl.rs`，由代 ma 桥接
//!   [`WxOpenMaService`] 按 Java 语义装配七个子服务；组件层
//!   `getWxMaServiceByAppid` 取回的桥接实例自带全部子服务 getter。

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;

use async_trait::async_trait;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

use wx_rust_common::api::wx_consts::app_id_type;
use wx_rust_common::bean::oauth2::WxOAuth2AccessToken;
use wx_rust_common::bean::result::WxMinishopImageUploadResult;
use wx_rust_common::enums::WxType;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::{
    RequestExecutor, SimpleGetRequestExecutor, SimplePostRequestExecutor,
};
use wx_rust_miniapp::api::WxMaService;
use wx_rust_mp::api::WxMpService;

use crate::api::r#impl::base_wx_open_service_impl;
use crate::api::r#impl::{WxOpenMaService, WxOpenMinishopServiceImpl, WxOpenMpService};
use crate::api::{WxOpenComponentService, WxOpenService};
use crate::bean::message::WxOpenXmlMessage;
use crate::bean::{
    GetShareCloudBaseEnvResponse, GetTcbEnvListResponse, LimitDiscountGoods, LimitDiscountSku,
    MinishopBrand, MinishopBrandList, MinishopBusiLicense, MinishopCategories, MinishopCategory,
    MinishopDeliveryTemplate, MinishopDeliveryTemplateResult, MinishopIdcardInfo, MinishopNameInfo,
    MinishopOrganizationCodeInfo, MinishopReturnInfo, MinishopShopCat, MinishopShopCatList,
    MinishopSuperAdministratorInfo, ShareCloudBaseEnvRequest, ShareCloudBaseEnvResponse,
    ValuationType, WxMinishopAddGoodsSpuResult, WxMinishopCoupon, WxMinishopCouponStock,
    WxMinishopSku, WxMinishopSpu, WxOpenAuthorizerAccessToken, WxOpenAuthorizerInfoResult,
    WxOpenAuthorizerListResult, WxOpenAuthorizerOptionResult, WxOpenCreateResult, WxOpenGetResult,
    WxOpenHaveResult, WxOpenMaApplyOrderPathInfo, WxOpenMaCodeTemplate,
    WxOpenMaDomainConfirmFileResult, WxOpenMaDomainResult, WxOpenMaWebDomainResult,
    WxOpenQueryAuthResult, WxOpenRegisterBetaWeappResult, WxOpenRegisterPersonalWeappResult,
    WxOpenResult,
};
use crate::config::WxOpenConfigStorage;
use crate::constant::wx_open_constants::ACCESS_TOKEN_KEY_COMPONENT;
use crate::enums::url_core::{
    api_authorizer_token_url, api_create_preauthcode_url, api_start_push_ticket_url,
    component_login_page_url, component_mobile_login_page_url,
};

// ---------------------------------------------------------------------------
// URL 常量（对应 Java `WxOpenComponentService` 接口常量；以
// `WxOpenHostConfig.api_host` 前缀拼接，与 url_core 同一模式；limit
// discount 三个 URL 在 Java 中直接拼接 "access_token="（无 "?"），镜像）
// ---------------------------------------------------------------------------
mod urls {
    use crate::config::{API_DEFAULT_HOST_URL, WxOpenConfigStorage};

    /// 生成完整接口地址：域名前缀 + 路径（对应 Java 常量中的
    /// `https://api.weixin.qq.com` 前缀，支持自定义 apiHostUrl 替换）。
    pub(super) fn api(config: &dyn WxOpenConfigStorage, path: &str) -> String {
        let host = config
            .wx_open_host_config()
            .map(|h| h.api_host)
            .unwrap_or_else(|| API_DEFAULT_HOST_URL.to_string());
        format!("{host}{path}")
    }

    pub(super) fn query_auth(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/component/api_query_auth")
    }
    pub(super) fn get_authorizer_info(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/component/api_get_authorizer_info")
    }
    pub(super) fn get_authorizer_option(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/component/get_authorizer_option")
    }
    pub(super) fn set_authorizer_option(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/component/set_authorizer_option")
    }
    pub(super) fn get_authorizer_list(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/component/api_get_authorizer_list")
    }
    /// oauth2 access_token（对应 Java `OAUTH2_ACCESS_TOKEN_URL`，%s 由调用方
    /// format；Java 不编码 appid/code，镜像）。
    pub(super) fn oauth2_access_token(
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
    /// oauth2 refresh_token（对应 Java `OAUTH2_REFRESH_TOKEN_URL`）。
    pub(super) fn oauth2_refresh_token(
        config: &dyn WxOpenConfigStorage,
        app_id: &str,
        refresh_token: &str,
    ) -> String {
        format!(
            "{}/sns/oauth2/component/refresh_token?appid={}&grant_type=refresh_token&refresh_token={}&component_appid={}",
            host(config),
            app_id,
            refresh_token,
            component_app_id(config),
        )
    }
    /// 小程序 code 换 session（对应 Java `MINIAPP_JSCODE_2_SESSION`）。
    pub(super) fn miniapp_jscode2_session(
        config: &dyn WxOpenConfigStorage,
        app_id: &str,
        js_code: &str,
    ) -> String {
        format!(
            "{}/sns/component/jscode2session?appid={}&js_code={}&grant_type=authorization_code&component_appid={}",
            host(config),
            app_id,
            js_code,
            component_app_id(config),
        )
    }

    pub(super) fn have_open(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/open/have")
    }
    /// 创建开放平台帐号（对应 Java `WxOpenComponentService.CREATE_OPEN_URL`）。
    pub(super) fn create_open(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/open/create")
    }
    /// 绑定开放平台帐号（对应 Java `BIND_OPEN_URL`）。
    pub(super) fn bind_open(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/open/bind")
    }
    /// 解绑开放平台帐号（对应 Java `UNBIND_OPEN_URL`）。
    pub(super) fn unbind_open(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/open/unbind")
    }
    /// 获取开放平台帐号（对应 Java `GET_OPEN_URL`）。
    pub(super) fn get_open(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/open/get")
    }
    pub(super) fn get_template_draft_list(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/wxa/gettemplatedraftlist")
    }
    pub(super) fn get_template_list(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/wxa/gettemplatelist")
    }
    pub(super) fn add_to_template(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/wxa/addtotemplate")
    }
    pub(super) fn delete_template(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/wxa/deletetemplate")
    }
    pub(super) fn fast_register_weapp(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/component/fastregisterweapp?action=create")
    }
    pub(super) fn fast_register_weapp_search(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/component/fastregisterweapp?action=search")
    }
    pub(super) fn fast_register_personal_weapp(config: &dyn WxOpenConfigStorage) -> String {
        api(
            config,
            "/wxa/component/fastregisterpersonalweapp?action=create",
        )
    }
    pub(super) fn fast_register_personal_weapp_search(config: &dyn WxOpenConfigStorage) -> String {
        api(
            config,
            "/wxa/component/fastregisterpersonalweapp?action=query",
        )
    }
    pub(super) fn fast_register_beta_weapp(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/wxa/component/fastregisterbetaweapp")
    }
    pub(super) fn register_shop(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/register/register_shop")
    }
    pub(super) fn check_shop_audit_status(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/register/check_audit_status")
    }
    pub(super) fn submit_merchant_info(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/register/submit_merchantinfo")
    }
    pub(super) fn submit_basic_info(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/register/submit_basicinfo")
    }
    /// 上传图片（对应 Java `UPLOAD_IMAGE_URL` 与 `WxOpenMinishopService.
    /// UPLOAD_IMG_MINISHOP_FILE_URL`，同值）。
    pub(super) fn upload_image(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/img/upload")
    }
    pub(super) fn minishop_category_get(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/category/get")
    }
    pub(super) fn minishop_brand_get(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/brand/get")
    }
    pub(super) fn minishop_delivery_template_get(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/delivery/get_freight_template")
    }
    pub(super) fn minishop_shopcategory_get(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/store/get_shopcat")
    }
    pub(super) fn minishop_create_coupon(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/coupon/create")
    }
    pub(super) fn minishop_get_coupon_list(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/coupon/get_list")
    }
    pub(super) fn minishop_push_coupon(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/coupon/push")
    }
    pub(super) fn minishop_update_coupon(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/coupon/update")
    }
    pub(super) fn minishop_update_coupon_status(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/coupon/update_status")
    }
    pub(super) fn minishop_get_delivery_company(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/delivery/get_company_list")
    }
    pub(super) fn minishop_add_spu(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/spu/add")
    }
    pub(super) fn minishop_del_spu(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/spu/del")
    }
    pub(super) fn minishop_update_spu(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/spu/update")
    }
    pub(super) fn minishop_listing_spu(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/spu/listing")
    }
    pub(super) fn minishop_delisting_spu(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/spu/delisting")
    }
    pub(super) fn minishop_add_sku(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/sku/add")
    }
    pub(super) fn minishop_batch_add_sku(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/sku/batch_add")
    }
    pub(super) fn minishop_del_sku(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/sku/del")
    }
    pub(super) fn minishop_update_sku(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/sku/update")
    }
    pub(super) fn minishop_update_sku_price(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/sku/update_price")
    }
    pub(super) fn minishop_update_sku_stock(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/stock/update")
    }
    /// 注意：Java `API_MINISHOP_ADD_LIMIT_DISCOUNT_URL` 后直接拼接
    /// `"access_token="`（无 "?"），镜像该线格式。
    pub(super) fn minishop_add_limit_discount(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/limiteddiscount/add/")
    }
    pub(super) fn minishop_get_limit_discount(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/product/limiteddiscount/get_list/")
    }
    pub(super) fn minishop_update_limit_discount_status(
        config: &dyn WxOpenConfigStorage,
    ) -> String {
        api(config, "/product/limiteddiscount/update_status/")
    }
    pub(super) fn batch_get_env_id(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/componenttcb/batchgetenvid")
    }
    pub(super) fn describe_envs(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/componenttcb/describeenvs")
    }
    pub(super) fn modify_env(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/tcb/modifyenv")
    }
    pub(super) fn batch_share_env(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/componenttcb/batchshareenv")
    }
    pub(super) fn clear_quota(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/component/clear_quota/v2")
    }
    pub(super) fn modify_wxa_server_domain(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/component/modify_wxa_server_domain")
    }
    pub(super) fn get_domain_confirm_file(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/component/get_domain_confirmfile")
    }
    pub(super) fn modify_wxa_jump_domain(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/cgi-bin/component/modify_wxa_jump_domain")
    }
    pub(super) fn apply_set_order_path_info(config: &dyn WxOpenConfigStorage) -> String {
        api(config, "/wxa/security/applysetorderpathinfo")
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
}

// ---------------------------------------------------------------------------
// minishop 入参 JSON 拼装（逐键镜像 Java bean 的 `toJsonObject()` snake_case
// 线格式；Rust bean serde 为 camelCase，不可直接序列化）
// ---------------------------------------------------------------------------
mod minishop_json {
    use serde_json::{Value, json};

    use crate::bean::{
        LimitDiscountGoods, LimitDiscountSku, MinishopAddressInfo, MinishopBusiLicense,
        MinishopIdcardInfo, MinishopNameInfo, MinishopOrganizationCodeInfo, MinishopPicFile,
        MinishopReturnInfo, MinishopShopCat, MinishopSuperAdministratorInfo, WxMinishopCoupon,
        WxMinishopCouponDiscountCondition, WxMinishopCouponDiscountInfo, WxMinishopCouponExtInfo,
        WxMinishopCouponPromoteInfo, WxMinishopCouponReceiveInfo, WxMinishopCouponValidInfo,
        WxMinishopGoodsSkuAttr, WxMinishopSku, WxMinishopSpu,
    };

    /// 对应 Java `MinishopPicFile.toJsonObject()`。
    pub(super) fn pic_file(p: &MinishopPicFile) -> Value {
        json!({ "media_id": p.media_id, "pay_media_id": p.pay_media_id })
    }

    /// 对应 Java `MinishopBusiLicense.toJsonObject()`。
    pub(super) fn busi_license(b: &MinishopBusiLicense) -> Value {
        let mut v = json!({
            "license_type": b.license_type,
            "pic_file": pic_file(&b.pic_file),
            "registration_num": b.registration_num,
            "merchant_name": b.merchant_name,
            "legal_representative": b.legal_representative,
            "start_date": b.start_date,
            "end_date": b.end_date,
        });
        // Java `if (registeredAddrs != null)`；Rust String 非空判断镜像
        if !b.registered_addrs.is_empty() {
            v["registered_addrs"] = json!(b.registered_addrs);
        }
        v
    }

    /// 对应 Java `MinishopOrganizationCodeInfo.toJsonObject()`。
    pub(super) fn organization_code_info(o: &MinishopOrganizationCodeInfo) -> Value {
        json!({
            "pic_file": pic_file(&o.pic_file),
            "organization_code": o.organization_code,
            "start_date": o.start_date,
            "end_date": o.end_date,
        })
    }

    /// 对应 Java `MinishopIdcardInfo.toJsonObject()`。
    pub(super) fn idcard_info(i: &MinishopIdcardInfo) -> Value {
        json!({
            "portrait_pic_file": pic_file(&i.portrait_pic_file),
            "nation_pic_file": pic_file(&i.nation_pic_file),
            "id_card_name": i.id_card_name,
            "id_card_number": i.id_card_number,
            "start_date": i.start_date,
            "end_date": i.end_date,
        })
    }

    /// 对应 Java `MinishopSuperAdministratorInfo.toJsonObject()`。
    pub(super) fn super_administrator_info(s: &MinishopSuperAdministratorInfo) -> Value {
        json!({
            "type": s.r#type,
            "name": s.name,
            "id_card_number": s.id_card_number,
            "phone": s.phone,
            "mail": s.mail,
        })
    }

    /// 对应 Java `MinishopNameInfo.toJsonObject()`。
    pub(super) fn name_info(n: &MinishopNameInfo) -> Value {
        json!({
            "nickname": n.nick_name,
            "abbr": n.abbr,
            "introduction": n.introduction,
        })
    }

    /// 对应 Java `MinishopAddressInfo.toJsonObject()`。
    pub(super) fn address_info(a: &MinishopAddressInfo) -> Value {
        json!({
            "user_name": a.user_name,
            "postal_code": a.postal_code,
            "province_name": a.province,
            "city_name": a.city_name,
            "county_name": a.county_name,
            "detail_info": a.detail_info,
            "national_code": a.national_code,
            "tel_number": a.tel_number,
        })
    }

    /// 对应 Java `MinishopReturnInfo.toJsonObject()`。
    pub(super) fn return_info(r: &MinishopReturnInfo) -> Value {
        json!({
            "address_info": address_info(&r.address_info),
            "mail": r.email,
            "company_address": address_info(&r.company_address),
        })
    }

    /// 对应 Java `WxMinishopGoodsSkuAttr.toJsonObject()`。
    pub(super) fn sku_attr(a: &WxMinishopGoodsSkuAttr) -> Value {
        json!({ "attr_key": a.attr_key, "attr_value": a.attr_value })
    }

    /// 对应 Java `WxMinishopShopCat`（spu 中 cats 元素：cat_id/level）。
    pub(super) fn shop_cat(c: &MinishopShopCat) -> Value {
        json!({ "cat_id": c.shop_cat_id, "level": c.cat_level })
    }

    /// 对应 Java `WxMinishopSku.toJsonObject()`。
    pub(super) fn sku(s: &WxMinishopSku) -> Value {
        let attrs: Vec<Value> = s.sku_attrs.iter().map(sku_attr).collect();
        json!({
            "product_id": s.product_id,
            "out_product_id": s.out_product_id,
            "out_sku_id": s.out_sku_id,
            "thumb_img": s.thumb_img,
            "sale_price": s.sale_price,
            "market_price": s.market_price,
            "stock_num": s.stock_num,
            "sku_code": s.sku_code,
            "barcode": s.bar_code,
            "sku_attrs": attrs,
        })
    }

    /// 对应 Java `WxMinishopSpu.toJsonObject()`。
    pub(super) fn spu(spu: &WxMinishopSpu) -> Value {
        let cats: Vec<Value> = spu.shop_cats.iter().map(shop_cat).collect();
        let attrs: Vec<Value> = spu.attrs.iter().map(sku_attr).collect();
        let skus: Vec<Value> = spu.skus.iter().map(sku).collect();
        json!({
            "out_product_id": spu.out_product_id,
            "title": spu.title,
            "sub_title": spu.sub_title,
            "head_img": spu.head_imgs,
            "desc_info": { "imgs": spu.desc_info_imgs },
            "brand_id": spu.brand_id,
            "cats": cats,
            "attrs": attrs,
            "model": spu.model,
            "express_info": { "template_id": spu.express_template_id },
            "skus": skus,
        })
    }

    /// 对应 Java `WxMinishopCouponDiscountCondition.toJsonObject()`。
    pub(super) fn coupon_discount_condition(c: &WxMinishopCouponDiscountCondition) -> Value {
        json!({
            "product_cnt": c.product_cnt,
            "product_ids": c.product_ids,
            "product_price": c.product_price,
        })
    }

    /// 对应 Java `WxMinishopCouponDiscountInfo.toJsonObject()`。
    pub(super) fn coupon_discount_info(d: &WxMinishopCouponDiscountInfo) -> Value {
        json!({
            "discount_condition": coupon_discount_condition(&d.discount_condition),
            "discount_fee": d.discount_fee,
            "discount_num": d.discount_num,
        })
    }

    /// 对应 Java `WxMinishopCouponExtInfo.toJsonObject()`。
    pub(super) fn coupon_ext_info(e: &WxMinishopCouponExtInfo) -> Value {
        json!({
            "jump_product_id": e.jump_product_id,
            "notes": e.notes,
            "valid_time": e.valid_time,
            "invalid_time": e.invalid_time,
        })
    }

    /// 对应 Java `WxMinishopCouponPromoteInfo.toJsonObject()`。
    pub(super) fn coupon_promote_info(p: &WxMinishopCouponPromoteInfo) -> Value {
        json!({
            "customize_channel": p.customize_channel,
            "promote_type": p.promotion_type,
        })
    }

    /// 对应 Java `WxMinishopCouponReceiveInfo.toJsonObject()`。
    pub(super) fn coupon_receive_info(r: &WxMinishopCouponReceiveInfo) -> Value {
        json!({
            "start_time": r.start_time,
            "end_time": r.end_time,
            "limit_num_one_person": r.limit_num_one_person,
            "total_num": r.total_num,
        })
    }

    /// 对应 Java `WxMinishopCouponValidInfo.toJsonObject()`。
    pub(super) fn coupon_valid_info(v: &WxMinishopCouponValidInfo) -> Value {
        json!({
            "start_time": v.start_time,
            "end_time": v.end_time,
            "valid_day_num": v.valid_day_num,
            "valid_type": v.valid_type,
        })
    }

    /// 对应 Java `WxMinishopCoupon.toJsonObject()`。
    pub(super) fn coupon(c: &WxMinishopCoupon) -> Value {
        let mut v = json!({
            "type": c.r#type,
            "name": c.name,
        });
        // Java `if (couponId != null)`：以 -1 表达未设置（ADAPTED，见 bean）
        if c.coupon_id >= 0 {
            v["coupon_id"] = json!(c.coupon_id);
        }
        if c.status >= 0 {
            v["status"] = json!(c.status);
        }
        v["discount_info"] = coupon_discount_info(&c.discount_info);
        v["ext_info"] = coupon_ext_info(&c.ext_info);
        v["promote_info"] = coupon_promote_info(&c.promote_info);
        v["receive_info"] = coupon_receive_info(&c.receive_info);
        v["valid_info"] = coupon_valid_info(&c.valid_info);
        v
    }

    /// 对应 Java `LimitDiscountSku.toJsonObject()`（sale_price 以分为单位：
    /// Java `salePrice.multiply(100).longValue()`；Rust 侧 bean 以 String
    /// 存金额，解析为 f64 后乘 100，ADAPTED）。
    pub(super) fn limit_discount_sku(s: &LimitDiscountSku) -> Value {
        let sale_price_cents = s
            .sale_price
            .parse::<f64>()
            .map(|p| (p * 100.0).round() as i64)
            .unwrap_or(0);
        json!({
            "sku_id": s.sku_id,
            "sale_price": sale_price_cents,
            "sale_stock": s.sale_stock,
        })
    }

    /// 对应 Java `LimitDiscountGoods.toJsonObject()`。
    pub(super) fn limit_discount_goods(g: &LimitDiscountGoods) -> Value {
        let mut v = json!({
            "product_id": g.product_id,
            "start_time": g.start_time,
            "end_time": g.end_time,
        });
        if g.task_id != 0 {
            v["task_id"] = json!(g.task_id);
        }
        if g.status != 0 {
            v["status"] = json!(g.status);
        }
        let skus: Vec<Value> = g
            .limit_discount_sku_list
            .iter()
            .map(limit_discount_sku)
            .collect();
        v["limited_discount_sku_list"] = json!(skus);
        v
    }
}

/// 组件服务实现。
pub struct WxOpenComponentServiceImpl {
    wx_open_service: Weak<dyn WxOpenService>,
    /// 代 mp 服务缓存（对应 Java 静态 `WX_OPEN_MP_SERVICE_MAP`
    /// ConcurrentHashMap + synchronized 双检锁；Rust 以实例级
    /// `Mutex<HashMap>` 表达，值经具体类型上转为
    /// `Arc<dyn Any + Send + Sync>`，调用方 downcast 下转）。
    mp_services: Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>,
    /// 代 ma 服务缓存（对应 Java 静态 `WX_OPEN_MA_SERVICE_MAP`）。
    ma_services: Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>,
    /// 代 fast_ma 服务缓存（对应 Java 静态 `WX_OPEN_FAST_MA_SERVICE_MAP`；
    /// Java fast_ma 为独立类但语义与 ma 桥接相同，Rust 统一以
    /// [`WxOpenMaService`] 承载，ADAPTED）。
    fast_ma_services: Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>,
    /// 小商店服务缓存（对应 Java 静态 `WX_OPEN_MINISHOP_SERVICE_MAP`，
    /// 装配 [`WxOpenMinishopServiceImpl`]）。
    minishop_services: Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl WxOpenComponentServiceImpl {
    /// 构建组件服务（对应 Java `new WxOpenComponentServiceImpl(this)`）。
    ///
    /// # 参数
    /// - `wx_open_service`：门面服务弱引用（Rust 以弱引用打破循环引用）
    pub fn new(wx_open_service: Weak<dyn WxOpenService>) -> Self {
        Self {
            wx_open_service,
            mp_services: Mutex::new(HashMap::new()),
            ma_services: Mutex::new(HashMap::new()),
            fast_ma_services: Mutex::new(HashMap::new()),
            minishop_services: Mutex::new(HashMap::new()),
        }
    }

    /// 升级门面服务引用；门面已释放时返回业务错误。
    fn svc(&self) -> Result<Arc<dyn WxOpenService>, WxErrorException> {
        self.wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))
    }

    /// 裸 POST（对应 Java `getWxOpenService().post(url, postData)`：URL 已
    /// 含 access_token 或无需 token，不做 component_access_token 注入）。
    async fn bare_post(&self, uri: &str, post_data: &str) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let executor = SimplePostRequestExecutor::new(svc.http_client().clone());
        executor
            .execute(uri, post_data.to_string(), WxType::Open)
            .await
    }

    /// 微信开放平台帐号管理统一请求入口（对应 Java 私有方法
    /// `openAccountServicePost(String appId, String appIdType, String
    /// requestUrl, JsonObject param)`）。
    ///
    /// 镜像 Java switch：`mp` → 代 mp 服务（[`WxOpenMpService`]）的 post
    /// （mp 执行引擎注入 authorizer access_token）；`mini` → 代 ma 服务
    /// （[`WxOpenMaService`]）的 post；其余类型抛「appIdType类型异常」。
    ///
    /// ADAPTED：Java 直接抛 `new WxErrorException("appIdType类型异常")`，
    /// Rust 统一以 `Err(-99)` 表达；`Arc<dyn Any>` 下转失败（缓存值类型
    /// 不匹配）同样以 `Err(-99)` 表达（Java 强类型不可能出现）。
    async fn open_account_service_post(
        &self,
        app_id: &str,
        app_id_type: &str,
        request_url: &str,
        param: &serde_json::Value,
    ) -> Result<String, WxErrorException> {
        match app_id_type {
            app_id_type::MP_TYPE => {
                let any = self.get_wx_mp_service_by_appid(app_id).ok_or_else(|| {
                    WxErrorException::from_code(-99, "getWxMpServiceByAppid 返回 None")
                })?;
                let mp = any.downcast::<WxOpenMpService>().map_err(|_| {
                    WxErrorException::from_code(-99, "代 mp 服务 downcast 失败（缓存类型不匹配）")
                })?;
                let mp: Arc<dyn WxMpService> = mp;
                mp.post(request_url, &param.to_string()).await
            }
            app_id_type::MINI_TYPE => {
                let any = self.get_wx_ma_service_by_appid(app_id).ok_or_else(|| {
                    WxErrorException::from_code(-99, "getWxMaServiceByAppid 返回 None")
                })?;
                let ma = any.downcast::<WxOpenMaService>().map_err(|_| {
                    WxErrorException::from_code(-99, "代 ma 服务 downcast 失败（缓存类型不匹配）")
                })?;
                let ma: Arc<dyn WxMaService> = ma;
                ma.post(request_url, &param.to_string()).await
            }
            _ => Err(WxErrorException::from_code(-99, "appIdType类型异常")),
        }
    }

    /// 创建预授权链接（对应 Java 私有方法
    /// `createPreAuthUrl(String, String, String, boolean)`）。
    ///
    /// 流程：POST `api_create_preauthcode` 取 `pre_auth_code` → 按
    /// `COMPONENT_LOGIN_PAGE_URL`/`COMPONENT_MOBILE_LOGIN_PAGE_URL` 格式化
    /// （redirect_uri 经 JS `encodeURIComponent` 语义编码）→ auth_type/
    /// biz_appid 非空时替换 `xxx` 占位（Java 对空值替换为空串，此处以
    /// 只替换非空值表达，其余占位符保留由调用方处理——Wave 0 冻结语义）。
    async fn create_pre_auth_url(
        &self,
        redirect_uri: &str,
        auth_type: Option<&str>,
        biz_appid: Option<&str>,
        is_mobile: bool,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let component_app_id = config.component_app_id().unwrap_or_default().to_string();
        let pre_auth_code = self.get_pre_auth_code().await?;

        // Java `URIUtil.encodeURIComponent(redirectUri)`：JS encodeURIComponent 语义
        let encoded_redirect = utf8_percent_encode(redirect_uri, NON_ALPHANUMERIC).to_string();
        let mut pre_auth_url_str = if is_mobile {
            component_mobile_login_page_url(&component_app_id, &pre_auth_code, &encoded_redirect)
        } else {
            component_login_page_url(&component_app_id, &pre_auth_code, &encoded_redirect)
        };
        // Java `StringUtils.isNotEmpty(authType)` 才替换
        if let Some(auth_type) = auth_type {
            if !auth_type.is_empty() {
                pre_auth_url_str =
                    pre_auth_url_str.replace("&auth_type=xxx", &format!("&auth_type={auth_type}"));
            }
        }
        if let Some(biz_appid) = biz_appid {
            if !biz_appid.is_empty() {
                pre_auth_url_str =
                    pre_auth_url_str.replace("&biz_appid=xxx", &format!("&biz_appid={biz_appid}"));
            }
        }
        Ok(pre_auth_url_str)
    }

    /// 取 JSON 值辅助（Java `getAsInt` 语义：缺失/非数字时默认值）。
    fn json_i64(v: &serde_json::Value, key: &str) -> i64 {
        v.get(key).and_then(|x| x.as_i64()).unwrap_or(0)
    }

    fn json_str(v: &serde_json::Value, key: &str) -> String {
        v.get(key)
            .and_then(|x| x.as_str())
            .unwrap_or_default()
            .to_string()
    }

    /// 归一化 errcode 为字符串（对应 Java Gson 宽松类型转换语义）。
    ///
    /// 微信接口 `errcode` 为数字（如 `0`），Java 各 bean 的 `String errcode`
    /// 经 Gson 自动转字符串（`"0"`）；Rust serde 严格，数字无法反序列化
    /// 进 String 字段（生成的 bean 与 Java 字段类型一致，冻结），故在
    /// bean 解析前将数字 errcode 归一化为字符串——严格镜像 Java 原语义
    /// （ADAPTED，见 open_account 实现注释）。
    fn normalize_errcode(json: &str) -> Result<String, WxErrorException> {
        let mut value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if let Some(errcode) = value.get("errcode") {
            if errcode.is_number() {
                value["errcode"] = serde_json::Value::String(errcode.to_string());
            }
        }
        serde_json::to_string(&value).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxOpenComponentService for WxOpenComponentServiceImpl {
    fn wx_open_service(&self) -> Option<Arc<dyn WxOpenService>> {
        self.wx_open_service.upgrade()
    }

    /// 获取指定 appid 的开放平台公众号服务（对应 Java
    /// `getWxMpServiceByAppid(String appid)`，双检锁缓存按 appid 装配
    /// `WxOpenMpServiceImpl`）。
    ///
    /// 镜像 Java：先无锁读缓存，未命中再持锁二次检查后装配
    /// [`WxOpenMpService`]（构造时快照门面配置存储与 HTTP 客户端）。
    /// 返回值以 `Arc<dyn Any + Send + Sync>` 承载（trait 签名冻结），
    /// 调用方经 [`crate::api::r#impl::downcast_mp_service`] 下转为
    /// `Arc<dyn WxMpService>`。
    fn get_wx_mp_service_by_appid(&self, app_id: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        // Java `WX_OPEN_MP_SERVICE_MAP.get(appId)`（无锁首查）
        if let Some(svc) = self.mp_services.lock().unwrap().get(app_id) {
            return Some(svc.clone());
        }
        // Java `synchronized (WX_OPEN_MP_SERVICE_MAP)` 二次检查
        let mut map = self.mp_services.lock().unwrap();
        if let Some(svc) = map.get(app_id) {
            return Some(svc.clone());
        }
        let open_svc = self.svc().ok()?;
        // 先经具体类型上转（`Arc<WxOpenMpService>` → `Arc<dyn Any>`），
        // 保证调用方按具体类型 downcast 成功；new_arc 装配代公众号 oauth2
        // 服务（对应 Java 构造器 setOAuth2Service）
        let svc: Arc<dyn Any + Send + Sync> =
            WxOpenMpService::new_arc(open_svc, app_id.to_string());
        map.insert(app_id.to_string(), svc.clone());
        Some(svc)
    }

    /// 获取指定 appid 的开放平台小程序服务（对应 Java
    /// `getWxMaServiceByAppid(String appid)`，双检锁缓存装配
    /// `WxOpenMaServiceImpl`）。
    ///
    /// 返回值经 [`crate::api::r#impl::downcast_ma_service`] 下转为
    /// `Arc<dyn WxMaService>`。
    fn get_wx_ma_service_by_appid(&self, app_id: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        if let Some(svc) = self.ma_services.lock().unwrap().get(app_id) {
            return Some(svc.clone());
        }
        let mut map = self.ma_services.lock().unwrap();
        if let Some(svc) = map.get(app_id) {
            return Some(svc.clone());
        }
        let open_svc = self.svc().ok()?;
        let svc: Arc<dyn Any + Send + Sync> =
            Arc::new(WxOpenMaService::new(open_svc, app_id.to_string()));
        map.insert(app_id.to_string(), svc.clone());
        Some(svc)
    }

    /// 获取指定 appid 的快速创建的小程序服务（对应 Java
    /// `getWxFastMaServiceByAppid(String appid)`）。
    ///
    /// Java `@Deprecated`（2021-06-23：请使用 `WxOpenMaService`）。
    /// Java 以独立类 `WxOpenFastMaServiceImpl`（extends WxMaServiceImpl，
    /// getAccessToken 同委托 authorizer token）装配；Rust 语义等价，
    /// 统一以 [`WxOpenMaService`] 承载（独立缓存桶镜像 Java 独立 map，
    /// ADAPTED），返回值经 [`crate::api::r#impl::downcast_ma_service`]
    /// 下转为 `Arc<dyn WxMaService>`。
    fn get_wx_fast_ma_service_by_appid(&self, app_id: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        if let Some(svc) = self.fast_ma_services.lock().unwrap().get(app_id) {
            return Some(svc.clone());
        }
        let mut map = self.fast_ma_services.lock().unwrap();
        if let Some(svc) = map.get(app_id) {
            return Some(svc.clone());
        }
        let open_svc = self.svc().ok()?;
        let svc: Arc<dyn Any + Send + Sync> =
            Arc::new(WxOpenMaService::new(open_svc, app_id.to_string()));
        map.insert(app_id.to_string(), svc.clone());
        Some(svc)
    }

    /// 获取指定 appid 的小商店服务（对应 Java
    /// `getWxMinishopServiceByAppid(String appid)`，双检锁缓存装配
    /// `WxOpenMinishopServiceImpl`）。
    ///
    /// Wave 5 接线（此前恒 None）：镜像 Java 静态
    /// `WX_OPEN_MINISHOP_SERVICE_MAP` + synchronized 双检锁；Rust 以
    /// 实例级 `Mutex<HashMap>` 表达，装配 [`WxOpenMinishopServiceImpl`]
    /// （构造入参 appId + 门面弱引用，Java 传 `this` 组件服务 + appId +
    /// wxMaConfig，语义一致，ADAPTED）。返回值以
    /// `Arc<dyn Any + Send + Sync>` 承载，调用方按具体类型
    /// `WxOpenMinishopServiceImpl` downcast 下转。
    fn get_wx_minishop_service_by_appid(&self, app_id: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        if let Some(svc) = self.minishop_services.lock().unwrap().get(app_id) {
            return Some(svc.clone());
        }
        let mut map = self.minishop_services.lock().unwrap();
        if let Some(svc) = map.get(app_id) {
            return Some(svc.clone());
        }
        let open_svc = self.svc().ok()?;
        let svc: Arc<dyn Any + Send + Sync> =
            Arc::new(WxOpenMinishopServiceImpl::new(open_svc, app_id.to_string()));
        map.insert(app_id.to_string(), svc.clone());
        Some(svc)
    }

    async fn start_push_ticket(&self) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        // Java `jsonObject`：component_appid + component_secret
        let body = serde_json::json!({
            "component_appid": config.component_app_id().unwrap_or_default(),
            "component_secret": config.component_app_secret().unwrap_or_default(),
        });
        self.post(
            &api_start_push_ticket_url(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get_component_access_token(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        base_wx_open_service_impl::get_component_access_token_with_lock(svc.as_ref(), force_refresh)
            .await
    }

    async fn post(&self, uri: &str, post_data: &str) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let executor = SimplePostRequestExecutor::new(svc.http_client().clone());
        base_wx_open_service_impl::execute_with_retry(
            svc.as_ref(),
            &executor,
            uri,
            post_data.to_string(),
            ACCESS_TOKEN_KEY_COMPONENT,
        )
        .await
    }

    async fn post_with_key(
        &self,
        uri: &str,
        post_data: &str,
        access_token_key: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let executor = SimplePostRequestExecutor::new(svc.http_client().clone());
        base_wx_open_service_impl::execute_with_retry(
            svc.as_ref(),
            &executor,
            uri,
            post_data.to_string(),
            access_token_key,
        )
        .await
    }

    async fn post_with_token(
        &self,
        uri: &str,
        post_data: &str,
        access_token_key: &str,
        access_token: &str,
    ) -> Result<String, WxErrorException> {
        // Java：显式 token 拼进 uri，经门面裸 post 发送（无自动刷新）；
        // 执行器对 errcode!=0 已抛错，Java 中 `errorCode()==0` 分支
        // （返回 errorMsg）理论不可达，直接上抛（与 base 引擎同一约定）
        let uri_with_token = if uri.contains('?') {
            format!("{uri}&{access_token_key}={access_token}")
        } else {
            format!("{uri}?{access_token_key}={access_token}")
        };
        self.bare_post(&uri_with_token, post_data).await
    }

    async fn get(&self, uri: &str) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let executor = SimpleGetRequestExecutor::new(svc.http_client().clone());
        base_wx_open_service_impl::execute_with_retry(
            svc.as_ref(),
            &executor,
            uri,
            String::new(),
            ACCESS_TOKEN_KEY_COMPONENT,
        )
        .await
    }

    async fn get_with_key(
        &self,
        uri: &str,
        access_token_key: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let executor = SimpleGetRequestExecutor::new(svc.http_client().clone());
        base_wx_open_service_impl::execute_with_retry(
            svc.as_ref(),
            &executor,
            uri,
            String::new(),
            access_token_key,
        )
        .await
    }

    async fn get_pre_auth_code(&self) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        // Java `jsonObject.addProperty("component_appid", ...)`
        let body = serde_json::json!({
            "component_appid": config.component_app_id().unwrap_or_default(),
        });
        let response = self
            .post(
                &api_create_preauthcode_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("pre_auth_code")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "pre_auth_code 字段缺失"))
    }

    async fn get_pre_auth_url(&self, redirect_uri: &str) -> Result<String, WxErrorException> {
        self.create_pre_auth_url(redirect_uri, None, None, false)
            .await
    }

    async fn get_pre_auth_url_with(
        &self,
        redirect_uri: &str,
        auth_type: Option<&str>,
        biz_appid: Option<&str>,
    ) -> Result<String, WxErrorException> {
        self.create_pre_auth_url(redirect_uri, auth_type, biz_appid, false)
            .await
    }

    async fn get_mobile_pre_auth_url(
        &self,
        redirect_uri: &str,
    ) -> Result<String, WxErrorException> {
        self.create_pre_auth_url(redirect_uri, None, None, true)
            .await
    }

    async fn get_mobile_pre_auth_url_with(
        &self,
        redirect_uri: &str,
        auth_type: Option<&str>,
        biz_appid: Option<&str>,
    ) -> Result<String, WxErrorException> {
        self.create_pre_auth_url(redirect_uri, auth_type, biz_appid, true)
            .await
    }

    async fn route(&self, message: &WxOpenXmlMessage) -> Result<String, WxErrorException> {
        // Java 先判 null 抛 NPE；Rust 入参非空类型，跳过
        let info_type = message.info_type.as_deref().unwrap_or_default();
        // verify_ticket：存储链（component_verify_ticket → config）
        if info_type.eq_ignore_ascii_case("component_verify_ticket") {
            let config = self.svc()?.wx_open_config_storage();
            if let Some(ticket) = &message.component_verify_ticket {
                config.set_component_verify_ticket(ticket);
            }
            return Ok("success".to_string());
        }
        // 新增、更新授权：授权码换授权信息（回写 authorizer token）
        if info_type.eq_ignore_ascii_case("authorized")
            || info_type.eq_ignore_ascii_case("updateauthorized")
        {
            let code = message.authorization_code.as_deref().unwrap_or_default();
            let query_auth = self.get_query_auth(code).await?;
            if query_auth
                .authorization_info
                .as_ref()
                .and_then(|i| i.authorizer_appid.as_ref())
                .is_none()
            {
                // 对应 Java `throw new NullPointerException("getQueryAuth")`
                return Err(WxErrorException::from_code(-99, "getQueryAuth"));
            }
            return Ok("success".to_string());
        }
        // 快速创建小程序
        if info_type.eq_ignore_ascii_case("notify_third_fasteregister") && message.status == Some(0)
        {
            let code = message.auth_code.as_deref().unwrap_or_default();
            let query_auth = self.get_query_auth(code).await?;
            if query_auth
                .authorization_info
                .as_ref()
                .and_then(|i| i.authorizer_appid.as_ref())
                .is_none()
            {
                return Err(WxErrorException::from_code(-99, "getQueryAuth"));
            }
            return Ok("success".to_string());
        }
        Ok(String::new())
    }

    async fn get_query_auth(
        &self,
        authorization_code: &str,
    ) -> Result<WxOpenQueryAuthResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({
            "component_appid": config.component_app_id().unwrap_or_default(),
            "authorization_code": authorization_code,
        });
        let response = self
            .post(&urls::query_auth(config.as_ref()), &body.to_string())
            .await?;
        let query_auth: WxOpenQueryAuthResult =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        // Java：authorizer_access_token 非空 → updateAuthorizerAccessToken；
        // authorizer_refresh_token 非空 → updateAuthorizerRefreshToken
        if let Some(info) = &query_auth.authorization_info {
            if let (Some(appid), Some(token)) =
                (&info.authorizer_appid, &info.authorizer_access_token)
            {
                config.update_authorizer_access_token_with_expiry(
                    appid,
                    token,
                    info.expires_in.unwrap_or(0),
                );
            }
            if let (Some(appid), Some(refresh)) =
                (&info.authorizer_appid, &info.authorizer_refresh_token)
            {
                config.update_authorizer_refresh_token(appid, refresh);
            }
        }
        Ok(query_auth)
    }

    async fn get_authorizer_info(
        &self,
        authorizer_appid: &str,
    ) -> Result<WxOpenAuthorizerInfoResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({
            "component_appid": config.component_app_id().unwrap_or_default(),
            "authorizer_appid": authorizer_appid,
        });
        let response = self
            .post(
                &urls::get_authorizer_info(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_authorizer_list(
        &self,
        begin: i32,
        len: i32,
    ) -> Result<WxOpenAuthorizerListResult, WxErrorException> {
        // Java `begin = Math.max(begin, 0); len = len == 0 ? 10 : len;`
        let begin = begin.max(0);
        let len = if len == 0 { 10 } else { len };
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({
            "component_appid": config.component_app_id().unwrap_or_default(),
            "offset": begin,
            "count": len,
        });
        let response = self
            .post(
                &urls::get_authorizer_list(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let ret: WxOpenAuthorizerListResult =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        // Java：逐条回写 authorizer_appid/refresh_token 到配置存储
        if let Some(list) = &ret.list {
            for data in list {
                if let (Some(appid), Some(refresh)) =
                    (data.get("authorizer_appid"), data.get("refresh_token"))
                {
                    config.update_authorizer_refresh_token(appid, refresh);
                }
            }
        }
        Ok(ret)
    }

    async fn get_authorizer_option(
        &self,
        authorizer_appid: &str,
        option_name: &str,
    ) -> Result<WxOpenAuthorizerOptionResult, WxErrorException> {
        // Java：授权方 access_token 为 key，经 post(uri, json, "access_token",
        // authorizerAccessToken) 发送
        let authorizer_access_token = self
            .get_authorizer_access_token(authorizer_appid, false)
            .await?;
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({
            "component_appid": config.component_app_id().unwrap_or_default(),
            "authorizer_appid": authorizer_appid,
            "option_name": option_name,
        });
        let response = self
            .post_with_token(
                &urls::get_authorizer_option(config.as_ref()),
                &body.to_string(),
                "access_token",
                &authorizer_access_token,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn set_authorizer_option(
        &self,
        authorizer_appid: &str,
        option_name: &str,
        option_value: &str,
    ) -> Result<(), WxErrorException> {
        let authorizer_access_token = self
            .get_authorizer_access_token(authorizer_appid, false)
            .await?;
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({
            "component_appid": config.component_app_id().unwrap_or_default(),
            "authorizer_appid": authorizer_appid,
            "option_name": option_name,
            "option_value": option_value,
        });
        self.post_with_token(
            &urls::set_authorizer_option(config.as_ref()),
            &body.to_string(),
            "access_token",
            &authorizer_access_token,
        )
        .await?;
        Ok(())
    }

    async fn get_authorizer_access_token(
        &self,
        app_id: &str,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        if !force_refresh && !config.is_authorizer_access_token_expired(app_id) {
            return config
                .authorizer_access_token(app_id)
                .ok_or_else(|| WxErrorException::from_code(-99, "authorizer access token 为空"));
        }
        // 对应 Java `config.getWxMpConfigStorage(appId).getAccessTokenLock()`
        // （按 appId 分桶锁）；Java tryLock(100ms) 轮询 + 每轮重查缓存
        let lock = config.lock_by_key(app_id);
        let _guard = loop {
            if !force_refresh && !config.is_authorizer_access_token_expired(app_id) {
                return config.authorizer_access_token(app_id).ok_or_else(|| {
                    WxErrorException::from_code(-99, "authorizer access token 为空")
                });
            }
            match lock.try_lock() {
                Ok(guard) => break guard,
                Err(_) => tokio::time::sleep(Duration::from_millis(100)).await,
            }
        };

        let body = serde_json::json!({
            "component_appid": config.component_app_id().unwrap_or_default(),
            "authorizer_appid": app_id,
            "authorizer_refresh_token": config.authorizer_refresh_token(app_id).unwrap_or_default(),
        });
        let response = self
            .post(
                &api_authorizer_token_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let token: WxOpenAuthorizerAccessToken =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        config.update_authorizer_access_token(app_id, &token);
        config.update_authorizer_refresh_token(app_id, token.authorizer_refresh_token());
        config
            .authorizer_access_token(app_id)
            .ok_or_else(|| WxErrorException::from_code(-99, "authorizer access token 为空"))
    }

    async fn oauth2_get_access_token(
        &self,
        app_id: &str,
        code: &str,
    ) -> Result<WxOAuth2AccessToken, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = urls::oauth2_access_token(config.as_ref(), app_id, code);
        let response = self.get(&url).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn oauth2_refresh_access_token(
        &self,
        app_id: &str,
        refresh_token: &str,
    ) -> Result<WxOAuth2AccessToken, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = urls::oauth2_refresh_token(config.as_ref(), app_id, refresh_token);
        let response = self.get(&url).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn miniapp_jscode2_session(
        &self,
        app_id: &str,
        js_code: &str,
    ) -> Result<serde_json::Value, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = urls::miniapp_jscode2_session(config.as_ref(), app_id, js_code);
        let response = self.get(&url).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_template_draft_list(
        &self,
    ) -> Result<Option<Vec<WxOpenMaCodeTemplate>>, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        // Java `get(GET_TEMPLATE_DRAFT_LIST_URL, "access_token")`
        let response = self
            .get_with_key(
                &urls::get_template_draft_list(config.as_ref()),
                "access_token",
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if let Some(list) = json.get("draft_list") {
            serde_json::from_value(list.clone())
                .map(Some)
                .map_err(|e| WxErrorException::Serde(e.to_string()))
        } else {
            // Java 无 draft_list 返回 null
            Ok(None)
        }
    }

    async fn get_template_list(
        &self,
    ) -> Result<Option<Vec<WxOpenMaCodeTemplate>>, WxErrorException> {
        self.get_template_list_with_type(None).await
    }

    async fn get_template_list_with_type(
        &self,
        template_type: Option<i32>,
    ) -> Result<Option<Vec<WxOpenMaCodeTemplate>>, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        // Java `GET_TEMPLATE_LIST_URL + (templateType == null ? "" :
        // "?template_type=" + templateType)`
        let url = match template_type {
            Some(t) => format!(
                "{}?template_type={t}",
                urls::get_template_list(config.as_ref())
            ),
            None => urls::get_template_list(config.as_ref()),
        };
        let response = self.get_with_key(&url, "access_token").await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if let Some(list) = json.get("template_list") {
            serde_json::from_value(list.clone())
                .map(Some)
                .map_err(|e| WxErrorException::Serde(e.to_string()))
        } else {
            Ok(None)
        }
    }

    async fn add_to_template(&self, draft_id: i64) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({ "draft_id": draft_id });
        self.post_with_key(
            &urls::add_to_template(config.as_ref()),
            &body.to_string(),
            "access_token",
        )
        .await?;
        Ok(())
    }

    async fn add_to_template_with_type(
        &self,
        draft_id: i64,
        template_type: i32,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({ "draft_id": draft_id, "template_type": template_type });
        self.post_with_key(
            &urls::add_to_template(config.as_ref()),
            &body.to_string(),
            "access_token",
        )
        .await?;
        Ok(())
    }

    async fn delete_template(&self, template_id: i64) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({ "template_id": template_id });
        self.post_with_key(
            &urls::delete_template(config.as_ref()),
            &body.to_string(),
            "access_token",
        )
        .await?;
        Ok(())
    }

    async fn create_open_account(
        &self,
        app_id: &str,
        app_id_type: &str,
    ) -> Result<WxOpenCreateResult, WxErrorException> {
        // Java `param.addProperty("appid", appId)` 后经
        // `openAccountServicePost(appId, appIdType, CREATE_OPEN_URL, param)`
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let param = serde_json::json!({ "appid": app_id });
        let json = self
            .open_account_service_post(
                app_id,
                app_id_type,
                &urls::create_open(config.as_ref()),
                &param,
            )
            .await?;
        // Java `WxOpenCreateResult.fromJson(json)`；errcode 先经
        // normalize_errcode 归一化为字符串（Gson 宽松转换语义，ADAPTED）
        let json = Self::normalize_errcode(&json)?;
        WxOpenCreateResult::from_json(&json).map_err(WxErrorException::Serde)
    }

    async fn bind_open_account(
        &self,
        app_id: &str,
        app_id_type: &str,
        open_appid: &str,
    ) -> Result<bool, WxErrorException> {
        // Java `param.addProperty("appid", ...) + addProperty("open_appid", ...)`
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let param = serde_json::json!({ "appid": app_id, "open_appid": open_appid });
        let json = self
            .open_account_service_post(
                app_id,
                app_id_type,
                &urls::bind_open(config.as_ref()),
                &param,
            )
            .await?;
        // Java `WxOpenResult.fromJson(json).isSuccess()`（errcode 忽略大小写
        // 等于 "0"；errcode 先归一化为字符串，Gson 宽松转换语义，ADAPTED；
        // 缺失时 "" → false）
        let json = Self::normalize_errcode(&json)?;
        Ok(WxOpenResult::from_json(&json)
            .map_err(WxErrorException::Serde)?
            .errcode
            .eq_ignore_ascii_case("0"))
    }

    async fn unbind_open_account(
        &self,
        app_id: &str,
        app_id_type: &str,
        open_appid: &str,
    ) -> Result<bool, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let param = serde_json::json!({ "appid": app_id, "open_appid": open_appid });
        let json = self
            .open_account_service_post(
                app_id,
                app_id_type,
                &urls::unbind_open(config.as_ref()),
                &param,
            )
            .await?;
        let json = Self::normalize_errcode(&json)?;
        Ok(WxOpenResult::from_json(&json)
            .map_err(WxErrorException::Serde)?
            .errcode
            .eq_ignore_ascii_case("0"))
    }

    async fn get_open_account(
        &self,
        app_id: &str,
        app_id_type: &str,
    ) -> Result<WxOpenGetResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let param = serde_json::json!({ "appid": app_id });
        let json = self
            .open_account_service_post(
                app_id,
                app_id_type,
                &urls::get_open(config.as_ref()),
                &param,
            )
            .await?;
        // Java `WxOpenGetResult.fromJson(json)`；errcode 归一化同
        // create_open_account（ADAPTED）
        let json = Self::normalize_errcode(&json)?;
        WxOpenGetResult::from_json(&json).map_err(WxErrorException::Serde)
    }

    async fn have_open(&self) -> Result<WxOpenHaveResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        // Java `get(HAVE_OPEN_URL, "access_token")`
        let response = self
            .get_with_key(&urls::have_open(config.as_ref()), "access_token")
            .await?;
        WxOpenHaveResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn fast_register_weapp(
        &self,
        name: &str,
        code: &str,
        code_type: &str,
        legal_persona_wechat: &str,
        legal_persona_name: &str,
        component_phone: &str,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({
            "name": name,
            "code": code,
            "code_type": code_type,
            "legal_persona_wechat": legal_persona_wechat,
            "legal_persona_name": legal_persona_name,
            "component_phone": component_phone,
        });
        // Java `post(..., "component_access_token")`（默认键，逐字镜像）
        let response = self
            .post_with_key(
                &urls::fast_register_weapp(config.as_ref()),
                &body.to_string(),
                "component_access_token",
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn fast_register_weapp_search(
        &self,
        name: &str,
        legal_persona_wechat: &str,
        legal_persona_name: &str,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({
            "name": name,
            "legal_persona_wechat": legal_persona_wechat,
            "legal_persona_name": legal_persona_name,
        });
        let response = self
            .post_with_key(
                &urls::fast_register_weapp_search(config.as_ref()),
                &body.to_string(),
                "component_access_token",
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn fast_register_personal_weapp(
        &self,
        idname: &str,
        wxuser: &str,
        component_phone: &str,
    ) -> Result<WxOpenRegisterPersonalWeappResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({
            "idname": idname,
            "wxuser": wxuser,
            "component_phone": component_phone,
        });
        let response = self
            .post_with_key(
                &urls::fast_register_personal_weapp(config.as_ref()),
                &body.to_string(),
                "component_access_token",
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn fast_register_personal_weapp_search(
        &self,
        taskid: &str,
    ) -> Result<WxOpenRegisterPersonalWeappResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({ "taskid": taskid });
        let response = self
            .post_with_key(
                &urls::fast_register_personal_weapp_search(config.as_ref()),
                &body.to_string(),
                "component_access_token",
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn fast_register_beta_weapp(
        &self,
        name: &str,
        openid: &str,
    ) -> Result<WxOpenRegisterBetaWeappResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({ "name": name, "openid": openid });
        // Java `post(..., "access_token")`
        let response = self
            .post_with_key(
                &urls::fast_register_beta_weapp(config.as_ref()),
                &body.to_string(),
                "access_token",
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn register_shop(
        &self,
        wx_name: &str,
        id_card_name: &str,
        id_card_number: &str,
        channel_id: Option<&str>,
        api_openstore_type: Option<i32>,
        auth_page_url: Option<&str>,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let mut body = serde_json::json!({
            "wx_name": wx_name,
            "id_card_name": id_card_name,
            "id_card_number": id_card_number,
            "api_openstore_type": api_openstore_type,
        });
        if let Some(channel_id) = channel_id {
            if !channel_id.is_empty() {
                body["channel_id"] = serde_json::json!(channel_id);
            }
        }
        if let Some(auth_page_url) = auth_page_url {
            if !auth_page_url.is_empty() {
                body["auth_page_url"] = serde_json::json!(auth_page_url);
            }
        }
        let response = self
            .post_with_key(
                &urls::register_shop(config.as_ref()),
                &body.to_string(),
                "component_access_token",
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn check_audit_status(&self, wx_name: &str) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        // Java：URL 先拼 component_access_token，再经组件层 post（会再次
        // 注入 component_access_token，双 token 线格式镜像）
        let url = format!(
            "{}?access_token={}",
            urls::check_shop_audit_status(config.as_ref()),
            self.get_component_access_token(false).await?
        );
        let body = serde_json::json!({ "wx_name": wx_name });
        self.post(&url, &body.to_string()).await
    }

    async fn check_audit_status_with_appid(
        &self,
        app_id: &str,
        wx_name: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::check_shop_audit_status(config.as_ref()),
            self.get_authorizer_access_token(app_id, false).await?
        );
        let body = serde_json::json!({ "wx_name": wx_name });
        self.post(&url, &body.to_string()).await
    }

    async fn submit_merchant_info(
        &self,
        app_id: &str,
        subject_type: &str,
        busi_license: &MinishopBusiLicense,
        organization_code_info: Option<&MinishopOrganizationCodeInfo>,
        idcard_info: Option<&MinishopIdcardInfo>,
        super_administrator_info: Option<&MinishopSuperAdministratorInfo>,
        merchant_shortname: Option<&str>,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let mut body = serde_json::json!({
            "app_id": app_id,
            "subject_type": subject_type,
            "busi_license": minishop_json::busi_license(busi_license),
        });
        if let Some(o) = organization_code_info {
            body["organization_code_info"] = minishop_json::organization_code_info(o);
        }
        if let Some(i) = idcard_info {
            body["id_card_info"] = minishop_json::idcard_info(i);
        }
        if let Some(s) = super_administrator_info {
            body["super_administrator_info"] = minishop_json::super_administrator_info(s);
        }
        if let Some(name) = merchant_shortname {
            if !name.is_empty() {
                body["merchant_shortname"] = serde_json::json!(name);
            }
        }
        // Java `getWxOpenService().post(url, json)`（裸 post，URL 已带
        // 授权方 access_token）
        let url = format!(
            "{}?access_token={}",
            urls::submit_merchant_info(config.as_ref()),
            self.get_authorizer_access_token(app_id, false).await?
        );
        let response = self.bare_post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn submit_basic_info(
        &self,
        app_id: &str,
        name_info: &MinishopNameInfo,
        return_info: &MinishopReturnInfo,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({
            "appid": app_id,
            "name_info": minishop_json::name_info(name_info),
            "return_info": minishop_json::return_info(return_info),
        });
        let url = format!(
            "{}?access_token={}",
            urls::submit_basic_info(config.as_ref()),
            self.get_authorizer_access_token(app_id, false).await?
        );
        let response = self.bare_post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn upload_minishop_image_pic_file(
        &self,
        app_id: &str,
        height: i32,
        width: i32,
        file_path: &str,
    ) -> Result<WxMinishopImageUploadResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        // Java `WxOpenMinishopService.UPLOAD_IMG_MINISHOP_FILE_URL + "?access_token="
        // + getAuthorizerAccessToken(appId, false) + "&height=" + height +
        // "&width=" + width`；multipart 上传经门面 uploadMinishopMediaFile
        let url = format!(
            "{}?access_token={}&height={}&width={}",
            urls::upload_image(config.as_ref()),
            self.get_authorizer_access_token(app_id, false).await?,
            height,
            width,
        );
        svc.upload_minishop_media_file(&url, file_path).await
    }

    async fn get_minishop_categories(
        &self,
        app_id: &str,
        f_cat_id: i32,
    ) -> Result<MinishopCategories, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({ "f_cat_id": f_cat_id });
        let url = format!(
            "{}?access_token={}",
            urls::minishop_category_get(config.as_ref()),
            self.get_authorizer_access_token(app_id, false).await?
        );
        let response = self.bare_post(&url, &body.to_string()).await?;
        let resp: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let mut categories = MinishopCategories::default();
        categories.errcode = Self::json_i64(&resp, "errcode") as i32;
        if categories.errcode == 0 {
            // Java `if (catListJson != null || !catListJson.isEmpty())` 后遍历
            if let Some(list) = resp.get("cat_list").and_then(|v| v.as_array()) {
                for item in list {
                    let mut c = MinishopCategory::default();
                    c.cat_id = Self::json_i64(item, "cat_id") as i32;
                    c.f_cat_id = Self::json_i64(item, "f_cat_id") as i32;
                    c.name = Self::json_str(item, "name");
                    categories.cat_list.push(c);
                }
            }
        } else {
            categories.errmsg = Self::json_str(&resp, "errmsg");
        }
        Ok(categories)
    }

    async fn get_minishop_brands(
        &self,
        app_id: &str,
    ) -> Result<MinishopBrandList, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_brand_get(config.as_ref()),
            self.get_authorizer_access_token(app_id, false).await?
        );
        let response = self.bare_post(&url, "{}").await?;
        let resp: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let mut brand_list = MinishopBrandList::default();
        brand_list.errcode = Self::json_i64(&resp, "errcode") as i32;
        if brand_list.errcode == 0 {
            if let Some(list) = resp.get("brands").and_then(|v| v.as_array()) {
                for item in list {
                    let mut brand = MinishopBrand::default();
                    brand.first_cat_id = Self::json_i64(item, "first_cat_id") as i32;
                    brand.second_cat_id = Self::json_i64(item, "second_cat_id") as i32;
                    brand.third_cat_id = Self::json_i64(item, "third_cat_id") as i32;
                    if let Some(info) = item.get("brand_info") {
                        brand.brand_info.brand_id = Self::json_i64(info, "brand_id") as i32;
                        brand.brand_info.brand_name = Self::json_str(info, "brand_name");
                    }
                    brand_list.brands.push(brand);
                }
            }
        } else {
            brand_list.errmsg = Self::json_str(&resp, "errmsg");
        }
        Ok(brand_list)
    }

    async fn get_minishop_delivery_template(
        &self,
        app_id: &str,
    ) -> Result<MinishopDeliveryTemplateResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_delivery_template_get(config.as_ref()),
            self.get_authorizer_access_token(app_id, false).await?
        );
        let response = self.bare_post(&url, "{}").await?;
        let resp: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let mut result = MinishopDeliveryTemplateResult::default();
        result.err_code = Self::json_i64(&resp, "errcode") as i32;
        if result.err_code == 0 {
            if let Some(list) = resp.get("template_list").and_then(|v| v.as_array()) {
                for item in list {
                    let mut template = MinishopDeliveryTemplate::default();
                    template.template_id = Self::json_i64(item, "template_id") as i32;
                    template.name = Self::json_str(item, "name");
                    // Java `valuation_type == 1 ? WEIGHT : PACKAGE`
                    template.valuation_type = if Self::json_i64(item, "valuation_type") == 1 {
                        ValuationType::Weight
                    } else {
                        ValuationType::Package
                    };
                    result.template_list.push(template);
                }
            }
        } else {
            result.err_msg = Self::json_str(&resp, "errmsg");
        }
        Ok(result)
    }

    async fn get_minishop_cat_list(
        &self,
        app_id: &str,
    ) -> Result<MinishopShopCatList, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_shopcategory_get(config.as_ref()),
            self.get_authorizer_access_token(app_id, false).await?
        );
        let response = self.bare_post(&url, "{}").await?;
        let resp: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let mut shop_cat_list = MinishopShopCatList::default();
        shop_cat_list.errcode = Self::json_i64(&resp, "errcode") as i32;
        if shop_cat_list.errcode == 0 {
            if let Some(list) = resp.get("shopcat_list").and_then(|v| v.as_array()) {
                for item in list {
                    let mut cat = MinishopShopCat::default();
                    cat.shop_cat_id = Self::json_i64(item, "shopcat_id") as i32;
                    cat.shop_cat_name = Self::json_str(item, "shopcat_name");
                    cat.f_shop_cat_id = Self::json_i64(item, "f_shopcat_id") as i32;
                    cat.cat_level = Self::json_i64(item, "cat_level") as i32;
                    shop_cat_list.shop_cat_list.push(cat);
                }
            }
        } else {
            shop_cat_list.errmsg = Self::json_str(&resp, "errmsg");
        }
        Ok(shop_cat_list)
    }

    async fn get_minishop_delivery_company(
        &self,
        app_id: &str,
    ) -> Result<WxMinishopAddGoodsSpuResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_get_delivery_company(config.as_ref()),
            self.get_authorizer_access_token(app_id, false).await?
        );
        let response = self.bare_post(&url, "{}").await?;
        let resp: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let mut result = WxMinishopAddGoodsSpuResult::default();
        result.errcode = Self::json_i64(&resp, "errcode") as i32;
        if result.errcode == 0 {
            // Java `result.setData(companies)`：data 为 company_list 数组
            result.data = resp.get("company_list").cloned().unwrap_or_default();
        } else {
            result.errmsg = Self::json_str(&resp, "errmsg");
        }
        Ok(result)
    }

    async fn minishop_create_coupon(
        &self,
        app_id: &str,
        coupon_info: &WxMinishopCoupon,
    ) -> Result<i32, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        // Java `getAuthorizerAccessToken(appId, true)`（强制刷新）
        let url = format!(
            "{}?access_token={}",
            urls::minishop_create_coupon(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        let response = self
            .bare_post(&url, &minishop_json::coupon(coupon_info).to_string())
            .await?;
        let resp: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        // Java：errcode==0 → data.coupon_id，否则 -1
        let coupon_id = if Self::json_i64(&resp, "errcode") == 0 {
            resp.get("data")
                .map(|d| Self::json_i64(d, "coupon_id") as i32)
                .unwrap_or(-1)
        } else {
            -1
        };
        Ok(coupon_id)
    }

    async fn minishop_get_coupon_list(
        &self,
        app_id: &str,
        _start_create_time: &str,
        _end_create_time: &str,
        _status: i32,
        _page: i32,
        _page_size: i32,
    ) -> Result<Option<WxMinishopCouponStock>, WxErrorException> {
        // Java 实现先取授权方 token（forceRefresh=true）后 `return null`
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let _url = format!(
            "{}?access_token={}",
            urls::minishop_get_coupon_list(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        Ok(None)
    }

    async fn minishop_push_coupon_to_user(
        &self,
        app_id: &str,
        open_id: &str,
        coupon_id: i32,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_push_coupon(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        let body = serde_json::json!({ "openid": open_id, "coupon_id": coupon_id });
        let response = self.bare_post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn minishop_update_coupon(
        &self,
        app_id: &str,
        coupon_info: &WxMinishopCoupon,
    ) -> Result<i32, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_update_coupon(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        let response = self
            .bare_post(&url, &minishop_json::coupon(coupon_info).to_string())
            .await?;
        let resp: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let coupon_id = if Self::json_i64(&resp, "errcode") == 0 {
            resp.get("data")
                .map(|d| Self::json_i64(d, "coupon_id") as i32)
                .unwrap_or(-1)
        } else {
            -1
        };
        Ok(coupon_id)
    }

    async fn minishop_update_coupon_status(
        &self,
        app_id: &str,
        coupon_id: i32,
        status: i32,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_update_coupon_status(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        let body = serde_json::json!({ "coupon_id": coupon_id, "status": status });
        let response = self.bare_post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn minishop_goods_add_spu(
        &self,
        app_id: &str,
        spu: &WxMinishopSpu,
    ) -> Result<WxMinishopAddGoodsSpuResult, WxErrorException> {
        self.minishop_spu_common(app_id, urls::minishop_add_spu, spu, "create_time")
            .await
    }

    async fn minishop_goods_del_spu(
        &self,
        app_id: &str,
        product_id: i64,
        out_product_id: i64,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_del_spu(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        // Java `outProductId.toString()`
        let body = serde_json::json!({ "product_id": product_id, "out_product_id": out_product_id.to_string() });
        let response = self.bare_post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn minishop_goods_update_spu(
        &self,
        app_id: &str,
        spu: &WxMinishopSpu,
    ) -> Result<WxMinishopAddGoodsSpuResult, WxErrorException> {
        self.minishop_spu_common(app_id, urls::minishop_update_spu, spu, "update_time")
            .await
    }

    async fn minishop_goods_listing_spu(
        &self,
        app_id: &str,
        product_id: i64,
        out_product_id: i64,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_listing_spu(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        let body = serde_json::json!({ "product_id": product_id, "out_product_id": out_product_id.to_string() });
        let response = self.bare_post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn minishop_goods_delisting_spu(
        &self,
        app_id: &str,
        product_id: i64,
        out_product_id: i64,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_delisting_spu(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        let body = serde_json::json!({ "product_id": product_id, "out_product_id": out_product_id.to_string() });
        let response = self.bare_post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn minishop_goods_add_sku(
        &self,
        app_id: &str,
        sku: &WxMinishopSku,
    ) -> Result<WxMinishopAddGoodsSpuResult, WxErrorException> {
        // Java `minishiopGoodsAddSku`：add 接口响应 data{sku_id, create_time}
        self.minishop_sku_add_common(app_id, urls::minishop_add_sku, sku)
            .await
    }

    async fn minishop_goods_batch_add_sku(
        &self,
        app_id: &str,
        sku_list: &[WxMinishopSku],
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_batch_add_sku(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        let skus: Vec<serde_json::Value> = sku_list.iter().map(minishop_json::sku).collect();
        let body = serde_json::json!({ "skus": skus });
        let response = self.bare_post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn minishop_goods_del_sku(
        &self,
        app_id: &str,
        product_id: i64,
        out_product_id: i64,
        out_sku_id: &str,
        sku_id: i64,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_del_sku(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        let body = serde_json::json!({
            "product_id": product_id,
            "out_product_id": out_product_id,
            "sku_id": sku_id,
            "out_sku_id": out_sku_id,
        });
        let response = self.bare_post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn minishop_goods_update_sku(
        &self,
        app_id: &str,
        sku: &WxMinishopSku,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_update_sku(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        let response = self
            .bare_post(&url, &minishop_json::sku(sku).to_string())
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn minishop_goods_update_sku_price(
        &self,
        app_id: &str,
        product_id: i64,
        out_product_id: i64,
        out_sku_id: &str,
        sku_id: i64,
        _sale_price: i64,
        _market_price: i64,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_update_sku_price(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        // 镜像 Java 上游 bug：WxJava 将 sale_price/market_price 均写成
        // outSkuId（严格镜像原语义，见 impl 模块文档）
        let body = serde_json::json!({
            "product_id": product_id,
            "out_product_id": out_product_id,
            "sku_id": sku_id,
            "out_sku_id": out_sku_id,
            "sale_price": out_sku_id,
            "market_price": out_sku_id,
        });
        let response = self.bare_post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn minishop_goods_update_sku_stock(
        &self,
        app_id: &str,
        product_id: i64,
        out_product_id: i64,
        out_sku_id: &str,
        sku_id: i64,
        r#type: i32,
        stock_num: i32,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            urls::minishop_update_sku_stock(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        let body = serde_json::json!({
            "product_id": product_id,
            "out_product_id": out_product_id,
            "sku_id": sku_id,
            "out_sku_id": out_sku_id,
            "type": r#type,
            "stock_num": stock_num,
        });
        let response = self.bare_post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn minishop_common_post(
        &self,
        _app_id: &str,
        _url: &str,
        _request_param: &str,
    ) -> Result<Option<String>, WxErrorException> {
        // Java 实现 `return null`
        Ok(None)
    }

    async fn add_limit_discount_goods(
        &self,
        app_id: &str,
        limit_discount_goods: &LimitDiscountGoods,
    ) -> Result<i32, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        // Java `API_MINISHOP_ADD_LIMIT_DISCOUNT_URL + "access_token=" + ...`
        // （无 "?"，镜像线格式）
        let url = format!(
            "{}access_token={}",
            urls::minishop_add_limit_discount(config.as_ref()),
            self.get_authorizer_access_token(app_id, false).await?
        );
        let response = self
            .bare_post(
                &url,
                &minishop_json::limit_discount_goods(limit_discount_goods).to_string(),
            )
            .await?;
        let resp: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        // Java：taskId 默认 0，errcode==0 时取 task_id
        let task_id = if Self::json_i64(&resp, "errcode") == 0 {
            Self::json_i64(&resp, "task_id") as i32
        } else {
            0
        };
        Ok(task_id)
    }

    async fn get_limit_discount_list(
        &self,
        app_id: &str,
        status: Option<i32>,
    ) -> Result<Vec<LimitDiscountGoods>, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}access_token={}",
            urls::minishop_get_limit_discount(config.as_ref()),
            self.get_authorizer_access_token(app_id, false).await?
        );
        let mut body = serde_json::json!({});
        if let Some(status) = status {
            body["status"] = serde_json::json!(status);
        }
        let response = self.bare_post(&url, &body.to_string()).await?;
        let resp: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let mut list = Vec::new();
        if Self::json_i64(&resp, "errcode") == 0 {
            if let Some(goods_array) = resp.get("limited_discount_list").and_then(|v| v.as_array())
            {
                for (i, goods) in goods_array.iter().enumerate() {
                    let mut goods_bean = LimitDiscountGoods::default();
                    goods_bean.task_id = Self::json_i64(goods, "task_id");
                    goods_bean.status = Self::json_i64(goods, "status") as i32;
                    // Java `new Date(getAsLong() * 1000)`；Rust bean 以 String
                    // 承载（epoch 毫秒字符串，ADAPTED）
                    goods_bean.start_time = Self::json_i64(goods, "start_time")
                        .saturating_mul(1000)
                        .to_string();
                    goods_bean.end_time = Self::json_i64(goods, "end_time")
                        .saturating_mul(1000)
                        .to_string();
                    if let Some(sku_array) = goods
                        .get("limited_discount_sku_list")
                        .and_then(|v| v.as_array())
                    {
                        for (j, _) in sku_array.iter().enumerate() {
                            // 镜像 Java 上游 bug：`skuObj = skuArray.get(i)` 取
                            // 外层索引 i 而非 j（严格镜像原语义）
                            let sku_json = sku_array.get(i).unwrap_or(&serde_json::Value::Null);
                            let mut sku = LimitDiscountSku::default();
                            sku.sku_id = Self::json_i64(sku_json, "sku_id");
                            // Java `BigDecimal.valueOf(getAsDouble() / 100)`；
                            // Rust bean 以 String 承载金额（ADAPTED）
                            let price = sku_json
                                .get("sale_price")
                                .and_then(|v| v.as_f64())
                                .map(|p| (p / 100.0).to_string())
                                .unwrap_or_default();
                            sku.sale_price = price;
                            sku.sale_stock = Self::json_i64(sku_json, "sale_stock") as i32;
                            let _ = j;
                            goods_bean.limit_discount_sku_list.push(sku);
                        }
                    }
                    list.push(goods_bean);
                }
            }
        }
        Ok(list)
    }

    async fn update_limit_discount_status(
        &self,
        app_id: &str,
        task_id: i64,
        status: i32,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}access_token={}",
            urls::minishop_update_limit_discount_status(config.as_ref()),
            self.get_authorizer_access_token(app_id, false).await?
        );
        let body = serde_json::json!({ "task_id": task_id, "status": status });
        let response = self.bare_post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_share_cloud_base_env(
        &self,
        appids: &[String],
    ) -> Result<GetShareCloudBaseEnvResponse, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({ "appids": appids });
        let response = self
            .post(&urls::batch_get_env_id(config.as_ref()), &body.to_string())
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_tcb_env_list(&self) -> Result<GetTcbEnvListResponse, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let response = self
            .post(&urls::describe_envs(config.as_ref()), "{}")
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn change_tcb_env(&self, env: &str) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body = serde_json::json!({ "env": env });
        let response = self
            .post(&urls::modify_env(config.as_ref()), &body.to_string())
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn share_cloud_base_env(
        &self,
        request: &ShareCloudBaseEnvRequest,
    ) -> Result<ShareCloudBaseEnvResponse, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = self
            .post(&urls::batch_share_env(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn clear_quota_v2(&self, appid: &str) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        // Java `getWxOpenService().post(...)`（裸 post，无 token 注入）
        let body = serde_json::json!({
            "appid": appid,
            "component_appid": config.component_app_id().unwrap_or_default(),
            "appsecret": config.component_app_secret().unwrap_or_default(),
        });
        let response = self
            .bare_post(&urls::clear_quota(config.as_ref()), &body.to_string())
            .await?;
        WxOpenResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn apply_set_order_path_info(
        &self,
        info: &WxOpenMaApplyOrderPathInfo,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        // Java `new Gson().toJson(info)`；Rust serde 跳过 None 字段（ADAPTED）
        let body =
            serde_json::to_string(info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = self
            .post(&urls::apply_set_order_path_info(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn modify_wxa_server_domain(
        &self,
        action: &str,
        request_domains: &[String],
        ws_request_domains: &[String],
        upload_domains: &[String],
        download_domains: &[String],
        udp_domains: &[String],
        tcp_domains: &[String],
    ) -> Result<WxOpenMaDomainResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let mut body = serde_json::json!({ "action": action });
        // Java `!"get".equals(action)` 才带域名数组
        if action != "get" {
            body["requestdomain"] = serde_json::json!(request_domains);
            body["wsrequestdomain"] = serde_json::json!(ws_request_domains);
            body["uploaddomain"] = serde_json::json!(upload_domains);
            body["downloaddomain"] = serde_json::json!(download_domains);
            body["udpdomain"] = serde_json::json!(udp_domains);
            body["tcpdomain"] = serde_json::json!(tcp_domains);
        }
        let response = self
            .post(
                &urls::modify_wxa_server_domain(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_domain_confirm_file(
        &self,
    ) -> Result<WxOpenMaDomainConfirmFileResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let response = self
            .post(&urls::get_domain_confirm_file(config.as_ref()), "{}")
            .await?;
        WxOpenMaDomainConfirmFileResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn modify_wxa_jump_domain(
        &self,
        action: &str,
        domain_list: &[String],
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let mut body = serde_json::json!({ "action": action });
        if action != "get" {
            body["webviewdomain"] = serde_json::json!(domain_list);
        }
        self.post(
            &urls::modify_wxa_jump_domain(config.as_ref()),
            &body.to_string(),
        )
        .await
    }

    async fn modify_wxa_jump_domain_info(
        &self,
        action: &str,
        domain_list: &[String],
    ) -> Result<WxOpenMaWebDomainResult, WxErrorException> {
        let response = self.modify_wxa_jump_domain(action, domain_list).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

/// spu 增改公共实现（对应 Java `minishopGoodsAddSpu`/`minishopGoodsUpdateSpu`
/// 的响应解析差异仅在时间字段键名）。
impl WxOpenComponentServiceImpl {
    async fn minishop_spu_common<F>(
        &self,
        app_id: &str,
        url_fn: F,
        spu: &WxMinishopSpu,
        time_key: &str,
    ) -> Result<WxMinishopAddGoodsSpuResult, WxErrorException>
    where
        F: FnOnce(&dyn WxOpenConfigStorage) -> String,
    {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            url_fn(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        let response = self
            .bare_post(&url, &minishop_json::spu(spu).to_string())
            .await?;
        let resp: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let mut result = WxMinishopAddGoodsSpuResult::default();
        result.errcode = Self::json_i64(&resp, "errcode") as i32;
        if result.errcode == 0 {
            let data = resp.get("data").cloned().unwrap_or_default();
            // Java 以 product_id/out_product_id/时间字段重建 data 对象
            let mut data_obj = serde_json::Map::new();
            if let Some(pid) = data.get("product_id") {
                data_obj.insert("product_id".to_string(), pid.clone());
            }
            if let Some(oid) = data.get("out_product_id") {
                data_obj.insert("out_product_id".to_string(), oid.clone());
            }
            if let Some(t) = data.get(time_key) {
                data_obj.insert(time_key.to_string(), t.clone());
            }
            result.data = serde_json::Value::Object(data_obj);
        } else {
            result.errmsg = Self::json_str(&resp, "errmsg");
        }
        Ok(result)
    }

    /// sku 新增公共实现（对应 Java `minishiopGoodsAddSku` 响应解析：
    /// data{sku_id, create_time}）。
    async fn minishop_sku_add_common<F>(
        &self,
        app_id: &str,
        url_fn: F,
        sku: &WxMinishopSku,
    ) -> Result<WxMinishopAddGoodsSpuResult, WxErrorException>
    where
        F: FnOnce(&dyn WxOpenConfigStorage) -> String,
    {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let url = format!(
            "{}?access_token={}",
            url_fn(config.as_ref()),
            self.get_authorizer_access_token(app_id, true).await?
        );
        let response = self
            .bare_post(&url, &minishop_json::sku(sku).to_string())
            .await?;
        let resp: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let mut result = WxMinishopAddGoodsSpuResult::default();
        result.errcode = Self::json_i64(&resp, "errcode") as i32;
        if result.errcode == 0 {
            let data = resp.get("data").cloned().unwrap_or_default();
            let mut data_obj = serde_json::Map::new();
            if let Some(sid) = data.get("sku_id") {
                data_obj.insert("sku_id".to_string(), sid.clone());
            }
            if let Some(t) = data.get("create_time") {
                data_obj.insert("create_time".to_string(), t.clone());
            }
            result.data = serde_json::Value::Object(data_obj);
        } else {
            result.errmsg = Self::json_str(&resp, "errmsg");
        }
        Ok(result)
    }
}
