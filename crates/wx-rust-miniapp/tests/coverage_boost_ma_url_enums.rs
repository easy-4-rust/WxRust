//! 小程序覆盖率提升：G1/G3/G4 URL 构建函数全量断言（离线）。
//!
//! 对应 Java `WxMaApiUrlConstants` 各子域常量（Rust 侧为 config 参数 +
//! api_host 前缀函数，域名自定义由 host_config 覆盖）。本文件由脚本
//! 机械生成：逐函数断言 `默认域名 + 路径`，保证每个 URL 函数都被执行
//! （占位参数按类型填充）。

use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

/// 默认域名（`WxMaHostConfig::new().api_host`）。
const HOST: &str = "https://api.weixin.qq.com";

// ════════════════════════════════════════════════════════════════════════════════
// G1 核心服务组：url_g1_core（共 50 个函数）
// ════════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaApiUrlConstants `analysis` 子域地址常量。
#[test]
fn url_g1_core_analysis_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::analysis::get_daily_summary_trend_url(&config),
        format!("{HOST}/datacube/getweanalysisappiddailysummarytrend"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::analysis::get_daily_visit_trend_url(&config),
        format!("{HOST}/datacube/getweanalysisappiddailyvisittrend"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::analysis::get_weekly_visit_trend_url(&config),
        format!("{HOST}/datacube/getweanalysisappidweeklyvisittrend"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::analysis::get_monthly_visit_trend_url(&config),
        format!("{HOST}/datacube/getweanalysisappidmonthlyvisittrend"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::analysis::get_visit_distribution_url(&config),
        format!("{HOST}/datacube/getweanalysisappidvisitdistribution"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::analysis::get_daily_retain_info_url(&config),
        format!("{HOST}/datacube/getweanalysisappiddailyretaininfo"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::analysis::get_weekly_retain_info_url(&config),
        format!("{HOST}/datacube/getweanalysisappidweeklyretaininfo"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::analysis::get_monthly_retain_info_url(&config),
        format!("{HOST}/datacube/getweanalysisappidmonthlyretaininfo"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::analysis::get_visit_page_url(&config),
        format!("{HOST}/datacube/getweanalysisappidvisitpage"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::analysis::get_user_portrait_url(&config),
        format!("{HOST}/datacube/getweanalysisappiduserportrait"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `code` 子域地址常量。
#[test]
fn url_g1_core_code_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::commit_url(&config),
        format!("{HOST}/wxa/commit"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::get_qrcode_url(&config),
        format!("{HOST}/wxa/get_qrcode"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::get_category_url(&config),
        format!("{HOST}/wxa/get_category"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::get_page_url(&config),
        format!("{HOST}/wxa/get_page"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::submit_audit_url(&config),
        format!("{HOST}/wxa/submit_audit"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::get_audit_status_url(&config),
        format!("{HOST}/wxa/get_auditstatus"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::get_latest_audit_status_url(&config),
        format!("{HOST}/wxa/get_latest_auditstatus"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::release_url(&config),
        format!("{HOST}/wxa/release"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::change_visit_status_url(&config),
        format!("{HOST}/wxa/change_visitstatus"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::revert_code_release_url(&config),
        format!("{HOST}/wxa/revertcoderelease"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::get_support_version_url(&config),
        format!("{HOST}/cgi-bin/wxopen/getweappsupportversion"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::set_support_version_url(&config),
        format!("{HOST}/cgi-bin/wxopen/setweappsupportversion"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::undo_code_audit_url(&config),
        format!("{HOST}/wxa/undocodeaudit"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::code::get_version_info_url(&config),
        format!("{HOST}/wxa/getversioninfo"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `express` 子域地址常量。
#[test]
fn url_g1_core_express_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::express::all_delivery_url(&config),
        format!("{HOST}/cgi-bin/express/business/delivery/getall"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::express::all_account_url(&config),
        format!("{HOST}/cgi-bin/express/business/account/getall"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::express::bind_account_url(&config),
        format!("{HOST}/cgi-bin/express/business/account/bind"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::express::get_quota_url(&config),
        format!("{HOST}/cgi-bin/express/business/quota/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::express::update_printer_url(&config),
        format!("{HOST}/cgi-bin/express/business/printer/update"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::express::get_printer_url(&config),
        format!("{HOST}/cgi-bin/express/business/printer/getall"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::express::add_order_url(&config),
        format!("{HOST}/cgi-bin/express/business/order/add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::express::batch_get_order_url(&config),
        format!("{HOST}/cgi-bin/express/business/order/batchget"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::express::cancel_order_url(&config),
        format!("{HOST}/cgi-bin/express/business/order/cancel"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::express::get_order_url(&config),
        format!("{HOST}/cgi-bin/express/business/order/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::express::get_path_url(&config),
        format!("{HOST}/cgi-bin/express/business/path/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::express::test_update_order_url(&config),
        format!("{HOST}/cgi-bin/express/business/test_update_order"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `kefu` 子域地址常量。
#[test]
fn url_g1_core_kefu_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::kefu::get_kf_list_url(&config),
        format!("{HOST}/cgi-bin/customservice/getkflist"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::kefu::kf_account_add_url(&config),
        format!("{HOST}/customservice/kfaccount/add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::kefu::kf_account_update_url(&config),
        format!("{HOST}/customservice/kfaccount/update"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::kefu::kf_account_del_url(&config, "kf1@test"),
        format!("{HOST}/customservice/kfaccount/del?kf_account=kf1@test"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::kefu::kf_session_create_url(&config),
        format!("{HOST}/customservice/kfsession/create"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::kefu::kf_session_close_url(&config),
        format!("{HOST}/customservice/kfsession/close"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::kefu::kf_session_get_url(&config, "oTEST1"),
        format!("{HOST}/customservice/kfsession/getsession?openid=oTEST1"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::kefu::kf_session_list_url(&config, "kf1@test"),
        format!("{HOST}/customservice/kfsession/getsessionlist?kf_account=kf1@test"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `media` 子域地址常量。
#[test]
fn url_g1_core_media_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::media::media_upload_url(&config, "image"),
        format!("{HOST}/cgi-bin/media/upload?type=image"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::media::media_get_url(&config),
        format!("{HOST}/cgi-bin/media/get"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `setting` 子域地址常量。
#[test]
fn url_g1_core_setting_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::setting::modify_domain_url(&config),
        format!("{HOST}/wxa/modify_domain"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::setting::set_web_view_domain_url(&config),
        format!("{HOST}/wxa/setwebviewdomain"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::setting::bind_tester_url(&config),
        format!("{HOST}/wxa/bind_tester"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g1_core::setting::unbind_tester_url(&config),
        format!("{HOST}/wxa/unbind_tester"),
    );
}
// ════════════════════════════════════════════════════════════════════════════════
// G3 电商服务组：url_g3_shop（共 122 个函数）
// ════════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaApiUrlConstants `shop_account` 子域地址常量。
#[test]
fn url_g3_shop_shop_account_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_account::get_category_list_url(&config),
        format!("{HOST}/shop/account/get_category_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_account::get_brand_list_url(&config),
        format!("{HOST}/shop/account/get_brand_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_account::update_info_url(&config),
        format!("{HOST}/shop/account/update_info"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_account::get_info_url(&config),
        format!("{HOST}/shop/account/get_info"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `shop_aftersale` 子域地址常量。
#[test]
fn url_g3_shop_shop_aftersale_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::add_url(&config),
        format!("{HOST}/shop/ecaftersale/add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::cancel_url(&config),
        format!("{HOST}/shop/ecaftersale/cancel"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::update_url(&config),
        format!("{HOST}/shop/aftersale/update"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::ec_update_url(&config),
        format!("{HOST}/shop/ecaftersale/update"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::upload_return_info_url(&config),
        format!("{HOST}/shop/ecaftersale/uploadreturninfo"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::accept_refund_url(&config),
        format!("{HOST}/shop/ecaftersale/acceptrefund"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::accept_return_url(&config),
        format!("{HOST}/shop/ecaftersale/acceptreturn"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::reject_url(&config),
        format!("{HOST}/shop/ecaftersale/reject"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::upload_certificates_url(&config),
        format!("{HOST}/shop/ecaftersale/upload_certificates"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::update_deadline_url(&config),
        format!("{HOST}/shop/aftersale/update_deadline"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::get_list_url(&config),
        format!("{HOST}/shop/ecaftersale/get_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::get_url(&config),
        format!("{HOST}/shop/aftersale/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_aftersale::ec_get_url(&config),
        format!("{HOST}/shop/ecaftersale/get"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `shop_audit` 子域地址常量。
#[test]
fn url_g3_shop_shop_audit_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_audit::audit_brand_url(&config),
        format!("{HOST}/shop/audit/audit_brand"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_audit::audit_category_url(&config),
        format!("{HOST}/shop/audit/audit_category"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_audit::audit_result_url(&config),
        format!("{HOST}/shop/audit/result"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_audit::get_miniapp_certificate_url(&config),
        format!("{HOST}/shop/audit/get_miniapp_certificate"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `shop_cat` 子域地址常量。
#[test]
fn url_g3_shop_shop_cat_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_cat::get_cat_url(&config),
        format!("{HOST}/shop/cat/get"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `shop_coupon` 子域地址常量。
#[test]
fn url_g3_shop_shop_coupon_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_coupon::add_coupon_url(&config),
        format!("{HOST}/shop/coupon/add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_coupon::get_coupon_url(&config),
        format!("{HOST}/shop/coupon/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_coupon::get_coupon_list_url(&config),
        format!("{HOST}/shop/coupon/get_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_coupon::update_coupon_url(&config),
        format!("{HOST}/shop/coupon/update"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_coupon::update_coupon_status_url(&config),
        format!("{HOST}/shop/coupon/update_status"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_coupon::update_coupon_stock_url(&config),
        format!("{HOST}/shop/coupon/update_coupon_stock"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_coupon::add_user_coupon_url(&config),
        format!("{HOST}/shop/coupon/add_user_coupon"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_coupon::get_user_coupon_list_url(&config),
        format!("{HOST}/shop/coupon/get_usercoupon_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_coupon::update_user_coupon_url(&config),
        format!("{HOST}/shop/coupon/update_user_coupon"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_coupon::update_user_coupon_status_url(&config),
        format!("{HOST}/shop/coupon/update_usercoupon_status"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `shop_delivery` 子域地址常量。
#[test]
fn url_g3_shop_shop_delivery_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_delivery::get_company_list_url(&config),
        format!("{HOST}/shop/delivery/get_company_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_delivery::delivery_send_url(&config),
        format!("{HOST}/shop/delivery/send"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_delivery::delivery_receive_url(&config),
        format!("{HOST}/shop/delivery/recieve"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `shop_img` 子域地址常量。
#[test]
fn url_g3_shop_shop_img_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_img::img_upload_url(&config),
        format!("{HOST}/shop/img/upload"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `shop_order` 子域地址常量。
#[test]
fn url_g3_shop_shop_order_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_order::check_scene_url(&config),
        format!("{HOST}/shop/scene/check"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_order::order_add_url(&config),
        format!("{HOST}/shop/order/add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_order::order_pay_url(&config),
        format!("{HOST}/shop/order/pay"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_order::order_get_url(&config),
        format!("{HOST}/shop/order/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_order::order_get_list_url(&config),
        format!("{HOST}/shop/order/get_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_order::order_get_payment_params_url(&config),
        format!("{HOST}/shop/order/getpaymentparams"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `shop_pay` 子域地址常量。
#[test]
fn url_g3_shop_shop_pay_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_pay::create_order_url(&config),
        format!("{HOST}/shop/pay/createorder"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_pay::get_order_url(&config),
        format!("{HOST}/shop/pay/getorder"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_pay::refund_order_url(&config),
        format!("{HOST}/shop/pay/refundorder"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `shop_register` 子域地址常量。
#[test]
fn url_g3_shop_shop_register_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_register::register_apply_url(&config),
        format!("{HOST}/shop/register/apply"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_register::register_check_url(&config),
        format!("{HOST}/shop/register/check"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_register::register_finish_access_info_url(
            &config
        ),
        format!("{HOST}/shop/register/finish_access_info"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_register::register_apply_scene_url(&config),
        format!("{HOST}/shop/register/apply_scene"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `shop_sharer` 子域地址常量。
#[test]
fn url_g3_shop_shop_sharer_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_sharer::bind_url(&config),
        format!("{HOST}/shop/sharer/bind"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_sharer::get_sharer_data_summary_url(&config),
        format!("{HOST}/shop/sharer/get_sharer_data_summary"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_sharer::get_sharer_list_url(&config),
        format!("{HOST}/shop/sharer/get_sharer_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_sharer::get_sharer_live_order_list_url(&config),
        format!("{HOST}/shop/sharer/get_sharer_live_order_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_sharer::get_sharer_live_summary_list_url(&config),
        format!("{HOST}/shop/sharer/get_sharer_live_summary_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_sharer::search_sharer_url(&config),
        format!("{HOST}/shop/sharer/search_sharer"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_sharer::unbind_url(&config),
        format!("{HOST}/shop/sharer/unbind"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `shop_spu` 子域地址常量。
#[test]
fn url_g3_shop_shop_spu_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_spu::spu_add_url(&config),
        format!("{HOST}/shop/spu/add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_spu::spu_del_url(&config),
        format!("{HOST}/shop/spu/del"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_spu::spu_get_url(&config),
        format!("{HOST}/shop/spu/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_spu::spu_get_list_url(&config),
        format!("{HOST}/shop/spu/get_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_spu::spu_update_url(&config),
        format!("{HOST}/shop/spu/update"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_spu::spu_update_without_url(&config),
        format!("{HOST}/shop/spu/update_without_audit"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_spu::spu_listing_url(&config),
        format!("{HOST}/shop/spu/listing"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_spu::spu_delisting_url(&config),
        format!("{HOST}/shop/spu/delisting"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::shop_spu::spu_del_audit_url(&config),
        format!("{HOST}/shop/spu/del_audit"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `product` 子域地址常量。
#[test]
fn url_g3_shop_product_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::spu::add_url(&config),
        format!("{HOST}/product/spu/add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::spu::del_url(&config),
        format!("{HOST}/product/spu/del"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::spu::get_url(&config),
        format!("{HOST}/product/spu/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::spu::get_list_url(&config),
        format!("{HOST}/product/spu/get_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::spu::update_url(&config),
        format!("{HOST}/product/spu/update"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::spu::listing_url(&config),
        format!("{HOST}/product/spu/listing"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::spu::delisting_url(&config),
        format!("{HOST}/product/spu/delisting"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::sku::add_url(&config),
        format!("{HOST}/product/sku/add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::sku::batch_add_url(&config),
        format!("{HOST}/product/sku/batch_add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::sku::del_url(&config),
        format!("{HOST}/product/sku/del"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::sku::update_url(&config),
        format!("{HOST}/product/sku/update"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::sku::update_price_url(&config),
        format!("{HOST}/product/sku/update_price"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::sku::update_stock_url(&config),
        format!("{HOST}/product/stock/update"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::sku::get_list_url(&config),
        format!("{HOST}/product/sku/get_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::order::get_list_url(&config),
        format!("{HOST}/product/order/get_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::order::detail_url(&config),
        format!("{HOST}/product/order/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::order::change_merchant_notes_url(&config),
        format!("{HOST}/product/order/change_merchant_notes"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::order::delivery_send_url(&config),
        format!("{HOST}/product/delivery/send"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::order::get_after_sale_order_url(&config),
        format!("{HOST}/product/order/getaftersaleorder"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::order::batch_get_after_sale_order_url(
            &config
        ),
        format!("{HOST}/product/order/batchgetaftersaleorder"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::order::after_sale_accept_apply_url(&config),
        format!("{HOST}/product/order/acceptapply"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::order::after_sale_reject_apply_url(&config),
        format!("{HOST}/product/order/rejectrefund"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::other::get_category_url(&config),
        format!("{HOST}/product/category/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::other::get_brand_url(&config),
        format!("{HOST}/product/brand/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::other::get_freight_template_url(&config),
        format!("{HOST}/product/delivery/get_freight_template"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::product::other::img_upload_url(&config),
        format!("{HOST}/product/img/upload"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `order_management` 子域地址常量。
#[test]
fn url_g3_shop_order_management_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::order_management::get_order_detail_path_url(&config),
        format!("{HOST}/wxa/sec/order/get_order_detail_path"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::order_management::update_order_detail_path_url(
            &config
        ),
        format!("{HOST}/wxa/sec/order/update_order_detail_path"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `order_shipping` 子域地址常量。
#[test]
fn url_g3_shop_order_shipping_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::order_shipping::is_trade_managed_url(&config),
        format!("{HOST}/wxa/sec/order/is_trade_managed"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::order_shipping::upload_shipping_info_url(&config),
        format!("{HOST}/wxa/sec/order/upload_shipping_info"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::order_shipping::upload_combined_shipping_info_url(
            &config
        ),
        format!("{HOST}/wxa/sec/order/upload_combined_shipping_info"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::order_shipping::get_shipping_info_url(&config),
        format!("{HOST}/wxa/sec/order/get_order"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::order_shipping::get_shipping_info_list_url(&config),
        format!("{HOST}/wxa/sec/order/get_order_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::order_shipping::notify_confirm_receive_url(&config),
        format!("{HOST}/wxa/sec/order/notify_confirm_receive"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::order_shipping::set_msg_jump_path_url(&config),
        format!("{HOST}/wxa/sec/order/set_msg_jump_path"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::order_shipping::is_trade_management_confirmation_completed_url(&config),
        format!("{HOST}/wxa/sec/order/is_trade_management_confirmation_completed"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::order_shipping::op_special_order_url(&config),
        format!("{HOST}/wxa/sec/order/opspecialorder"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `express_delivery_return` 子域地址常量。
#[test]
fn url_g3_shop_express_delivery_return_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::express_delivery_return::add_delivery_return_url(
            &config
        ),
        format!("{HOST}/cgi-bin/express/delivery/return/add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::express_delivery_return::get_delivery_return_url(
            &config
        ),
        format!("{HOST}/cgi-bin/express/delivery/return/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::express_delivery_return::unbind_delivery_return_url(
            &config
        ),
        format!("{HOST}/cgi-bin/express/delivery/return/unbind"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `instant_delivery` 子域地址常量。
#[test]
fn url_g3_shop_instant_delivery_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::instant_delivery::get_bind_account_url(&config),
        format!("{HOST}/cgi-bin/express/local/business/shop/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::instant_delivery::get_order_url(&config),
        format!("{HOST}/cgi-bin/express/local/business/order/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::instant_delivery::mock_update_order_url(&config),
        format!("{HOST}/cgi-bin/express/local/business/test_update_order"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::instant_delivery::trace_waybill_url(&config),
        format!("{HOST}/cgi-bin/express/delivery/open_msg/trace_waybill"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::instant_delivery::query_waybill_trace_url(&config),
        format!("{HOST}/cgi-bin/express/delivery/open_msg/query_trace"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::instant_delivery::follow_waybill_url(&config),
        format!("{HOST}/cgi-bin/express/delivery/open_msg/follow_waybill"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::instant_delivery::query_follow_trace_url(&config),
        format!("{HOST}/cgi-bin/express/delivery/open_msg/query_follow_trace"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::instant_delivery::get_delivery_list_url(&config),
        format!("{HOST}/cgi-bin/express/delivery/open_msg/get_delivery_list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::instant_delivery::update_waybill_goods_url(&config),
        format!("{HOST}/cgi-bin/express/delivery/open_msg/update_waybill_goods"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::instant_delivery::add_order_url(&config),
        format!("{HOST}/cgi-bin/express/local/business/order/add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::instant_delivery::cancel_order_url(&config),
        format!("{HOST}/cgi-bin/express/local/business/order/cancel"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::instant_delivery::abnormal_confirm_url(&config),
        format!("{HOST}/cgi-bin/express/local/business/order/confirm_return"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `employee` 子域地址常量。
#[test]
fn url_g3_shop_employee_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::employee::unbind_employee_url(&config),
        format!("{HOST}/wxa/business/unbinduserb2cauthinfo"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::employee::send_employee_msg_url(&config),
        format!("{HOST}/cgi-bin/message/wxopen/employeerelationmsg/send"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `customservice_work` 子域地址常量。
#[test]
fn url_g3_shop_customservice_work_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::customservice_work::get_customservice_url(&config),
        format!("{HOST}/customservice/work/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::customservice_work::bind_customservice_url(&config),
        format!("{HOST}/customservice/work/bind"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g3_shop::customservice_work::unbind_customservice_url(&config),
        format!("{HOST}/customservice/work/unbind"),
    );
}
// ════════════════════════════════════════════════════════════════════════════════
// G4 能力服务组：url_g4_ability（共 166 个函数）
// ════════════════════════════════════════════════════════════════════════════════

/// 对应 Java: WxMaApiUrlConstants `live` 子域地址常量。
#[test]
fn url_g4_ability_live_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::create_room_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/create"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::get_live_info_url(&config),
        format!("{HOST}/wxa/business/getliveinfo"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::add_goods_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/addgoods"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::delete_room_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/deleteroom"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::edit_room_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/editroom"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::get_push_url_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/getpushurl"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::get_shared_code_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/getsharedcode"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::add_assistant_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/addassistant"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::modify_assistant_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/modifyassistant"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::remove_assistant_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/removeassistant"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::get_assistant_list_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/getassistantlist"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::add_subanchor_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/addsubanchor"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::modify_subanchor_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/modifysubanchor"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::delete_subanchor_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/deletesubanchor"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::get_subanchor_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/getsubanchor"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::update_feed_public_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/updatefeedpublic"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::update_replay_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/updatereplay"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::update_kf_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/updatekf"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::update_comment_url(&config),
        format!("{HOST}/wxaapi/broadcast/room/updatecomment"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::onsale_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/onsale"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::delete_in_room_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/deleteInRoom"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::push_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/push"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::sort_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/sort"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::room::get_video_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/getVideo"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::goods::add_goods_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::goods::reset_audit_goods_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/resetaudit"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::goods::audit_goods_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/audit"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::goods::delete_goods_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/delete"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::goods::update_goods_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/update"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::goods::get_goods_ware_house_url(&config),
        format!("{HOST}/wxa/business/getgoodswarehouse"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::goods::get_approved_goods_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/getapproved"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::goods::set_key_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/setkey"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::goods::get_key_url(&config),
        format!("{HOST}/wxaapi/broadcast/goods/getkey"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::role::add_role_url(&config),
        format!("{HOST}/wxaapi/broadcast/role/addrole"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::role::delete_role_url(&config),
        format!("{HOST}/wxaapi/broadcast/role/deleterole"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::live::role::list_by_role_url(&config),
        format!("{HOST}/wxaapi/broadcast/role/getrolelist"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `cloud` 子域地址常量。
#[test]
fn url_g4_ability_cloud_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::invoke_cloud_function_url(
            &config, "env-1", "fn-1"
        ),
        format!("{HOST}/tcb/invokecloudfunction?env=env-1&name=fn-1"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::database_collection_get_url(&config),
        format!("{HOST}/tcb/databasecollectionget"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::database_collection_delete_url(&config),
        format!("{HOST}/tcb/databasecollectiondelete"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::database_collection_add_url(&config),
        format!("{HOST}/tcb/databasecollectionadd"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::get_qcloud_token_url(&config),
        format!("{HOST}/tcb/getqcloudtoken"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::batch_delete_file_url(&config),
        format!("{HOST}/tcb/batchdeletefile"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::batch_download_file_url(&config),
        format!("{HOST}/tcb/batchdownloadfile"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::upload_file_url(&config),
        format!("{HOST}/tcb/uploadfile"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::database_migrate_query_info_url(&config),
        format!("{HOST}/tcb/databasemigratequeryinfo"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::database_migrate_export_url(&config),
        format!("{HOST}/tcb/databasemigrateexport"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::database_migrate_import_url(&config),
        format!("{HOST}/tcb/databasemigrateimport"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::update_index_url(&config),
        format!("{HOST}/tcb/updateindex"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::database_count_url(&config),
        format!("{HOST}/tcb/databasecount"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::database_aggregate_url(&config),
        format!("{HOST}/tcb/databaseaggregate"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::database_query_url(&config),
        format!("{HOST}/tcb/databasequery"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::database_update_url(&config),
        format!("{HOST}/tcb/databaseupdate"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::database_delete_url(&config),
        format!("{HOST}/tcb/databasedelete"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::database_add_url(&config),
        format!("{HOST}/tcb/databaseadd"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::cloud::send_sms_v2_url(&config),
        format!("{HOST}/tcb/sendsmsv2"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `marketing` 子域地址常量。
#[test]
fn url_g4_ability_marketing_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::marketing::user_action_sets_add_url(&config),
        format!("{HOST}/marketing/user_action_sets/add?version=v1.0"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::marketing::user_actions_add_url(&config),
        format!("{HOST}/marketing/user_actions/add?version=v1.0"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `promotion` 子域地址常量。
#[test]
fn url_g4_ability_promotion_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::add_role_url(&config),
        format!("{HOST}/promoter/addrole"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::get_role_url(&config),
        format!("{HOST}/promoter/getrole"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::update_role_url(&config),
        format!("{HOST}/promoter/updaterole"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::add_promoter_url(&config),
        format!("{HOST}/promoter/addpromoter"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::get_promoter_url(&config),
        format!("{HOST}/promoter/getpromoter"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::update_promoter_url(&config),
        format!("{HOST}/promoter/updatepromoter"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::get_invitation_material_url(&config),
        format!("{HOST}/promoter/getinvitationmaterial"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::send_msg_url(&config),
        format!("{HOST}/promoter/sendmsg"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::single_send_msg_url(&config),
        format!("{HOST}/promoter/singlesendmsg"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::get_msg_url(&config),
        format!("{HOST}/promoter/getmsg"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::get_msg_click_data_url(&config),
        format!("{HOST}/promoter/getmsgclickdata"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::get_share_material_url(&config),
        format!("{HOST}/promoter/getsharematerial"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::get_relation_url(&config),
        format!("{HOST}/promoter/getrelation"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::promotion::get_order_url(&config),
        format!("{HOST}/promoter/getorder"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `intracity` 子域地址常量。
#[test]
fn url_g4_ability_intracity_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::apply_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/apply"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::create_store_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/createstore"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::query_store_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/querystore"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::update_store_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/updatestore"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::store_charge_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/storecharge"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::store_refund_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/storerefund"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::query_flow_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/queryflow"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::balance_query_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/balancequery"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::get_pay_mode_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/getpaymode"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::set_pay_mode_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/setpaymode"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::pre_add_order_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/preaddorder"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::add_order_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/addorder"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::query_order_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/queryorder"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::cancel_order_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/cancelorder"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::intracity::get_city_url(&config),
        format!("{HOST}/cgi-bin/express/intracity/getcity"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `complaint` 子域地址常量。
#[test]
fn url_g4_ability_complaint_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::complaint::query_complaints_url(&config),
        format!("{HOST}/cgi-bin/miniapp/complaint/list"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::complaint::get_complaint_url(&config),
        format!("{HOST}/cgi-bin/miniapp/complaint/detail"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::complaint::query_negotiation_history_url(&config),
        format!("{HOST}/cgi-bin/miniapp/complaint/negotiation/history"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::complaint::add_complaint_notify_url(&config),
        format!("{HOST}/cgi-bin/miniapp/complaint/notify/add"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::complaint::get_complaint_notify_url(&config),
        format!("{HOST}/cgi-bin/miniapp/complaint/notify/get"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::complaint::update_complaint_notify_url(&config),
        format!("{HOST}/cgi-bin/miniapp/complaint/notify/update"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::complaint::delete_complaint_notify_url(&config),
        format!("{HOST}/cgi-bin/miniapp/complaint/notify/delete"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::complaint::submit_response_url(&config),
        format!("{HOST}/cgi-bin/miniapp/complaint/response"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::complaint::complete_complaint_url(&config),
        format!("{HOST}/cgi-bin/miniapp/complaint/complete"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::complaint::upload_response_image_url(&config),
        format!("{HOST}/cgi-bin/miniapp/complaint/upload"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `device_subscribe` 子域地址常量。
#[test]
fn url_g4_ability_device_subscribe_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::device_subscribe::get_sn_ticket_url(&config),
        format!("{HOST}/wxa/getsnticket"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::device_subscribe::send_device_subscribe_msg_url(
            &config
        ),
        format!("{HOST}/cgi-bin/message/device/subscribe/send"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::device_subscribe::create_iot_group_id_url(&config),
        format!("{HOST}/wxa/business/group/createid"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::device_subscribe::add_iot_group_device_url(&config),
        format!("{HOST}/wxa/business/group/adddevice"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::device_subscribe::remove_iot_group_device_url(
            &config
        ),
        format!("{HOST}/wxa/business/group/removedevice"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::device_subscribe::get_iot_group_info_url(&config),
        format!("{HOST}/wxa/business/group/getinfo"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `face` 子域地址常量。
#[test]
fn url_g4_ability_face_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::face::get_verify_id_url(&config),
        format!("{HOST}/cityservice/face/identify/getverifyid"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::face::query_verify_info_url(&config),
        format!("{HOST}/cityservice/face/identify/queryverifyinfo"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `invoice` 子域地址常量。
#[test]
fn url_g4_ability_invoice_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::invoice::get_invoice_info_url(&config),
        format!("{HOST}/card/invoice/reimburse/getinvoiceinfo"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::invoice::get_invoice_batch_url(&config),
        format!("{HOST}/card/invoice/reimburse/getinvoicebatch"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::invoice::update_invoice_status_url(&config),
        format!("{HOST}/card/invoice/reimburse/updateinvoicestatus"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::invoice::update_status_batch_url(&config),
        format!("{HOST}/card/invoice/reimburse/updatestatusbatch"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `qrcode_jump` 子域地址常量。
#[test]
fn url_g4_ability_qrcode_jump_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::qrcode_jump::add_rule_url(&config),
        format!("{HOST}/wxaapi/wxaqrcodefast/addcategoryrule"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::qrcode_jump::get_rules_url(&config),
        format!("{HOST}/wxaapi/wxaqrcodefast/getcategory"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::qrcode_jump::get_rule_list_url(&config),
        format!("{HOST}/wxaapi/wxaqrcodefast/getcategorybypage"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::qrcode_jump::delete_rule_url(&config),
        format!("{HOST}/wxaapi/wxaqrcodefast/deletecategoryrule"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `vod` 子域地址常量。
#[test]
fn url_g4_ability_vod_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::list_media_url(&config),
        format!("{HOST}/wxa/sec/vod/listmedia"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::get_media_url(&config),
        format!("{HOST}/wxa/sec/vod/getmedia"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::get_media_link_url(&config),
        format!("{HOST}/wxa/sec/vod/getmedialink"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::delete_media_url(&config),
        format!("{HOST}/wxa/sec/vod/deletemedia"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::audit_drama_url(&config),
        format!("{HOST}/wxa/sec/vod/auditdrama"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::list_dramas_url(&config),
        format!("{HOST}/wxa/sec/vod/listdramas"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::get_drama_url(&config),
        format!("{HOST}/wxa/sec/vod/getdrama"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::single_file_upload_url(&config),
        format!("{HOST}/wxa/sec/vod/singlefileupload"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::pull_upload_url(&config),
        format!("{HOST}/wxa/sec/vod/pullupload"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::get_task_url(&config),
        format!("{HOST}/wxa/sec/vod/gettask"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::apply_upload_url(&config),
        format!("{HOST}/wxa/sec/vod/applyupload"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::upload_part_url(&config),
        format!("{HOST}/wxa/sec/vod/uploadpart"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::commit_upload_url(&config),
        format!("{HOST}/wxa/sec/vod/commitupload"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::get_cdn_usage_data_url(&config),
        format!("{HOST}/wxa/sec/vod/getcdnusagedata"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::vod::get_cdn_logs_url(&config),
        format!("{HOST}/wxa/sec/vod/getcdnlogs"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `xpay` 子域地址常量。
#[test]
fn url_g4_ability_xpay_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::query_user_balance_url(&config),
        format!("{HOST}/xpay/query_user_balance?pay_sig=%s&signature=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::currency_pay_url(&config),
        format!("{HOST}/xpay/currency_pay?pay_sig=%s&signature=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::query_order_url(&config),
        format!("{HOST}/xpay/query_order?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::cancel_currency_pay_url(&config),
        format!("{HOST}/xpay/cancel_currency_pay?pay_sig=%s&signature=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::notify_provide_goods_url(&config),
        format!("{HOST}/xpay/notify_provide_goods?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::present_currency_url(&config),
        format!("{HOST}/xpay/present_currency?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::present_goods_url(&config),
        format!("{HOST}/xpay/present_goods?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::download_bill_url(&config),
        format!("{HOST}/xpay/download_bill?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::refund_order_url(&config),
        format!("{HOST}/xpay/refund_order?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::create_withdraw_order_url(&config),
        format!("{HOST}/xpay/create_withdraw_order?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::query_withdraw_order_url(&config),
        format!("{HOST}/xpay/query_withdraw_order?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::start_upload_goods_url(&config),
        format!("{HOST}/xpay/start_upload_goods?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::query_upload_goods_url(&config),
        format!("{HOST}/xpay/query_upload_goods?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::start_publish_goods_url(&config),
        format!("{HOST}/xpay/start_publish_goods?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::query_publish_goods_url(&config),
        format!("{HOST}/xpay/query_publish_goods?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::query_biz_balance_url(&config),
        format!("{HOST}/xpay/query_biz_balance?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::query_transfer_account_url(&config),
        format!("{HOST}/xpay/query_transfer_account?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::query_adver_funds_url(&config),
        format!("{HOST}/xpay/query_adver_funds?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::create_funds_bill_url(&config),
        format!("{HOST}/xpay/create_funds_bill?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::bind_transfer_account_url(&config),
        format!("{HOST}/xpay/bind_transfer_accout?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::query_funds_bill_url(&config),
        format!("{HOST}/xpay/query_funds_bill?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::query_recover_bill_url(&config),
        format!("{HOST}/xpay/query_recover_bill?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::get_complaint_list_url(&config),
        format!("{HOST}/xpay/get_complaint_list?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::get_complaint_detail_url(&config),
        format!("{HOST}/xpay/get_complaint_detail?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::get_negotiation_history_url(&config),
        format!("{HOST}/xpay/get_negotiation_history?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::response_complaint_url(&config),
        format!("{HOST}/xpay/response_complaint?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::complete_complaint_url(&config),
        format!("{HOST}/xpay/complete_complaint?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::upload_vp_file_url(&config),
        format!("{HOST}/xpay/upload_vp_file?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::get_upload_file_sign_url(&config),
        format!("{HOST}/xpay/get_upload_file_sign?pay_sig=%s"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::xpay::download_adverfunds_order_url(&config),
        format!("{HOST}/xpay/download_adverfunds_order?pay_sig=%s"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `ocr` 子域地址常量。
#[test]
fn url_g4_ability_ocr_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::ocr::id_card_url(&config, "T1"),
        format!("{HOST}/cv/ocr/idcard?img_url=T1"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::ocr::bank_card_url(&config, "T1"),
        format!("{HOST}/cv/ocr/bankcard?img_url=T1"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::ocr::driving_url(&config, "T1"),
        format!("{HOST}/cv/ocr/driving?img_url=T1"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::ocr::driving_license_url(&config, "T1"),
        format!("{HOST}/cv/ocr/drivinglicense?img_url=T1"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::ocr::biz_license_url(&config, "T1"),
        format!("{HOST}/cv/ocr/bizlicense?img_url=T1"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::ocr::comm_url(&config, "T1"),
        format!("{HOST}/cv/ocr/comm?img_url=T1"),
    );
}

/// 对应 Java: WxMaApiUrlConstants `img_proc` 子域地址常量。
#[test]
fn url_g4_ability_img_proc_urls() {
    let config = WxMaDefaultConfig::new("wxappid", "secret");
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::img_proc::qrcode_url(&config, "T1"),
        format!("{HOST}/cv/img/qrcode?img_url=T1"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::img_proc::super_resolution_url(&config, "T1"),
        format!("{HOST}/cv/img/superresolution?img_url=T1"),
    );
    assert_eq!(
        wx_rust_miniapp::enums::url_g4_ability::img_proc::ai_crop_url(&config, "T1", "T1"),
        format!("{HOST}/cv/img/aicrop?img_url=T1&ratios=T1"),
    );
}
