//! jsapi 签名数据对象。
//!
//! 对应 Java `me.chanjar.weixin.common.bean.WxJsapiSignature`。

/// jsapi 签名结果。
///
/// 用于 JS-SDK 的前端配置签名（`wx.config`）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WxJsapiSignature {
    /// 公众号 appId
    pub app_id: String,

    /// 随机串
    pub nonce_str: String,

    /// 时间戳（秒）
    pub timestamp: i64,

    /// 需要签名的 URL
    pub url: String,

    /// 签名值
    pub signature: String,
}

impl WxJsapiSignature {
    /// 构建 jsapi 签名结果。
    ///
    /// # 参数
    /// - `app_id`：公众号 appId
    /// - `nonce_str`：随机串
    /// - `timestamp`：时间戳（秒）
    /// - `url`：需要签名的 URL
    /// - `signature`：签名值
    pub fn new(
        app_id: impl Into<String>,
        nonce_str: impl Into<String>,
        timestamp: i64,
        url: impl Into<String>,
        signature: impl Into<String>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            nonce_str: nonce_str.into(),
            timestamp,
            url: url.into(),
            signature: signature.into(),
        }
    }
}
