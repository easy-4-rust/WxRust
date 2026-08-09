//! 小程序内容服务组接口地址。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.constant.WxMaApiUrlConstants` 中
//! Scheme / Jsapi / Plugin / OpenApi 子域地址（内容服务组 G2 各子服务使用）。
//! 函数风格与 `url_business` / `url_core` 一致：config 参数 + api_host 前缀
//! 模式（自定义域名替换由执行引擎在 token 注入时统一处理）。
//! 本组其余子域（subscribe/link/qrcode/internet）地址已在 `url_business`
//! 中就绪，直接复用，不在此重复定义。

use crate::config::{DEFAULT_API_HOST_URL, WxMaConfig};

/// 生成完整接口地址：域名前缀 + 路径。
fn url(_config: &dyn WxMaConfig, host: &str, path: &str) -> String {
    format!("{host}{path}")
}

/// Scheme 码接口地址（对应 Java `WxMaApiUrlConstants.Scheme`）。
pub mod scheme {
    use super::*;

    /// 获取小程序 scheme 码（对应 Java `Scheme.GENERATE_SCHEME_URL`）。
    pub fn generate_scheme_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/generatescheme")
    }

    /// 获取 NFC 的小程序 scheme（对应 Java `Scheme.GENERATE_NFC_SCHEME_URL`）。
    pub fn generate_nfc_scheme_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/generatenfcscheme")
    }
}

/// jsapi 接口地址（对应 Java `WxMaApiUrlConstants.Jsapi`）。
pub mod jsapi {
    use super::*;

    /// 获得 jsapi_ticket 的 url（对应 Java `Jsapi.GET_JSAPI_TICKET_URL`；
    /// `?type=` 查询参数由调用方按 ticket 类型拼接，如 `?type=jsapi`）。
    pub fn get_jsapi_ticket_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/ticket/getticket")
    }
}

/// 插件管理接口地址（对应 Java `WxMaApiUrlConstants.Plugin`）。
pub mod plugin {
    use super::*;

    /// 插件管理（申请/查询/删除/更新，对应 Java `Plugin.PLUGIN_URL`；
    /// 动作由请求体 `action` 字段区分）。
    pub fn plugin_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/plugin")
    }
}

/// openApi 管理接口地址（对应 Java `WxMaApiUrlConstants.OpenApi`）。
pub mod openapi {
    use super::*;

    /// 重置 API 调用次数（对应 Java `OpenApi.CLEAR_QUOTA`）。
    pub fn clear_quota_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/clear_quota")
    }

    /// 查询 API 调用额度（对应 Java `OpenApi.GET_API_QUOTA`）。
    pub fn get_api_quota_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/openapi/quota/get")
    }

    /// 查询 rid 信息（对应 Java `OpenApi.GET_RID_INFO`）。
    pub fn get_rid_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/openapi/rid/get")
    }

    /// 使用 AppSecret 重置 API 调用次数（对应 Java
    /// `OpenApi.CLEAR_QUOTA_BY_APP_SECRET`，`%s` 依次为 appid/appsecret）。
    pub fn clear_quota_by_app_secret_url(
        config: &dyn WxMaConfig,
        appid: &str,
        appsecret: &str,
    ) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cgi-bin/clear_quota/v2?appid={appid}&appsecret={appsecret}"),
        )
    }
}

/// 默认 API 域名字面量（与 `url_core::API_HOST` 一致，供本模块内部使用）。
#[allow(unused)]
const API_HOST: &str = DEFAULT_API_HOST_URL;
