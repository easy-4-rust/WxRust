//! 小程序门面层接口地址。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaService` 中声明的 URL 常量
//! （`GET_ACCESS_TOKEN_URL`/`GET_STABLE_ACCESS_TOKEN`/`JSCODE_TO_SESSION_URL`
//! /`GET_PAID_UNION_ID_URL`/`SET_DYNAMIC_DATA_URL`）。函数风格参照
//! `crate::enums::wx_mp_api_url`（config 参数 + api_host 前缀模式）；
//! 各业务子域完整地址表（`WxMaApiUrlConstants`）随对应子服务批次补齐。

use crate::config::{DEFAULT_API_HOST_URL, WxMaConfig};

/// 生成完整接口地址：域名前缀 + 路径。
fn url(_config: &dyn WxMaConfig, host: &str, path: &str) -> String {
    format!("{host}{path}")
}

/// 获取 access_token（对应 Java `WxMaService.GET_ACCESS_TOKEN_URL`）。
pub fn get_access_token_url(config: &dyn WxMaConfig) -> String {
    let h = config.host_config();
    url(
        config,
        &h.api_host,
        &format!(
            "/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
            config.app_id(),
            config.secret()
        ),
    )
}

/// 获取稳定版接口调用凭据（对应 Java `WxMaService.GET_STABLE_ACCESS_TOKEN`）。
pub fn get_stable_access_token_url(config: &dyn WxMaConfig) -> String {
    let h = config.host_config();
    url(config, &h.api_host, "/cgi-bin/stable_token")
}

/// 登录凭证校验（code 换 session，对应 Java `WxMaService.JSCODE_TO_SESSION_URL`）。
pub fn js_code_to_session_url(config: &dyn WxMaConfig) -> String {
    let h = config.host_config();
    url(config, &h.api_host, "/sns/jscode2session")
}

/// 获取支付用户的 UnionId（对应 Java `WxMaService.GET_PAID_UNION_ID_URL`）。
pub fn get_paid_union_id_url(config: &dyn WxMaConfig) -> String {
    let h = config.host_config();
    url(config, &h.api_host, "/wxa/getpaidunionid")
}

/// 导入抽样数据（对应 Java `WxMaService.SET_DYNAMIC_DATA_URL`）。
pub fn set_dynamic_data_url(config: &dyn WxMaConfig) -> String {
    let h = config.host_config();
    url(config, &h.api_host, "/wxa/setdynamicdata")
}

/// 默认 API 域名字面量（对应 Java `WxMaConfig.DEFAULT_API_HOST_URL`）。
pub const API_HOST: &str = DEFAULT_API_HOST_URL;
