//! 小程序配置存储接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.config.WxMaConfig`，在
//! `wx-rust-common::config::WxConfigStorage`（token/ticket/锁/代理）基础上
//! 扩展小程序专属配置项。

use wx_rust_common::config::WxConfigStorage;

use crate::config::WxMaHostConfig;

/// 微信 API 默认主机地址（对应 Java `WxMaConfig.DEFAULT_API_HOST_URL`）。
pub const DEFAULT_API_HOST_URL: &str = "https://api.weixin.qq.com";
/// 微信云托管使用的 HTTP 协议主机地址（对应 Java `WxMaConfig.CLOUD_RUN_API_HOST_URL`）。
pub const CLOUD_RUN_API_HOST_URL: &str = "http://api.weixin.qq.com";

/// 小程序配置存储。
///
/// Java 接口以 getter/setter 形式暴露；Rust trait 以只读方法 + 可变方法
/// 表达同一契约。app_id/secret/access_token 缓存/过期/锁语义继承自
/// common 的 `WxConfigStorage`。
pub trait WxMaConfig: WxConfigStorage + Send + Sync {
    /// 设置是否使用稳定版 access token 接口（查询见 common `WxConfigStorage`）。
    fn use_stable_access_token(&self, use_stable_access_token: bool);

    /// 消息校验 token（对应 Java `getToken()`）。
    fn token(&self) -> Option<&str>;

    /// 消息加解密 aes key（对应 Java `getAesKey()`）。
    fn aes_key(&self) -> Option<&str>;

    /// 原始 ID（原始公众号/小程序 ID，对应 Java `getOriginalId()`）。
    fn original_id(&self) -> Option<&str>;

    /// 云开发（Cloud）环境标识（对应 Java `getCloudEnv()`）。
    fn cloud_env(&self) -> Option<&str>;

    /// 消息数据的格式，如 JSON/XML（对应 Java `getMsgDataFormat()`）。
    fn msg_data_format(&self) -> Option<&str>;

    /// HTTP 请求重试间隔（毫秒），对应 Java `getRetrySleepMillis()`。
    fn retry_sleep_millis(&self) -> i32 {
        1000
    }

    /// HTTP 请求最大重试次数，对应 Java `getMaxRetryTimes()`。
    fn max_retry_times(&self) -> i32 {
        5
    }

    /// 自定义接口域名配置。
    fn host_config(&self) -> WxMaHostConfig;

    /// 设置自定义接口域名配置。
    fn set_host_config(&self, host_config: WxMaHostConfig);

    /// 自定义 apiHost 地址（对应 Java `getApiHostUrl()`）。
    ///
    /// 返回 owned 值（Rust 适配：实现侧可能以锁保护字段，无法返回借用）。
    fn api_host_url(&self) -> Option<String>;

    /// 设置自定义 apiHost 地址（对应 Java `setApiHostUrl(String)`）。
    fn set_api_host_url(&self, api_host_url: &str);

    /// 自定义获取 accessToken 地址（对应 Java `getAccessTokenUrl()`）。
    fn access_token_url(&self) -> Option<String>;

    /// 设置自定义获取 accessToken 地址（对应 Java `setAccessTokenUrl(String)`）。
    fn set_access_token_url(&self, access_token_url: &str);

    /// 服务端 API 签名用到的 RSA 私钥（pkcs8 格式，对应 Java `getApiSignatureRsaPrivateKey()`）。
    fn api_signature_rsa_private_key(&self) -> Option<String>;

    /// 服务端 API 签名用到的 AES 密钥（对应 Java `getApiSignatureAesKey()`）。
    fn api_signature_aes_key(&self) -> Option<String>;

    /// API 签名 AES 密钥对应的序号（对应 Java `getApiSignatureAesKeySn()`）。
    fn api_signature_aes_key_sn(&self) -> Option<String>;

    /// API 签名 RSA 私钥对应的序号（对应 Java `getApiSignatureRsaPrivateKeySn()`）。
    fn api_signature_rsa_private_key_sn(&self) -> Option<String>;

    /// 签名用的小程序 ID：普通小程序为 appId，托管第三方平台为 componentAppId
    /// （对应 Java `getWechatMpAppid()`）。
    fn wechat_mp_appid(&self) -> Option<String>;

    /// 是否使用微信云托管内网模式（对应 Java `isUseWxCloudRun()`）。
    ///
    /// 开启后 SDK 自动将 `https://api.weixin.qq.com` 替换为
    /// `http://api.weixin.qq.com`。
    fn is_use_wx_cloud_run(&self) -> bool {
        false
    }

    /// 设置是否使用微信云托管内网模式（对应 Java `setUseWxCloudRun(boolean)`）。
    fn set_use_wx_cloud_run(&self, _use_wx_cloud_run: bool) {}

    /// 根据配置获取实际应使用的 API 主机地址（对应 Java `getEffectiveApiHostUrl()`）。
    ///
    /// 优先级：自定义 apiHostUrl > 微信云托管模式 > 默认 HTTPS 地址。
    fn effective_api_host_url(&self) -> String {
        if let Some(api_host_url) = self.api_host_url() {
            if !api_host_url.is_empty() {
                return api_host_url;
            }
        }
        if self.is_use_wx_cloud_run() {
            return CLOUD_RUN_API_HOST_URL.to_string();
        }
        DEFAULT_API_HOST_URL.to_string()
    }
}
