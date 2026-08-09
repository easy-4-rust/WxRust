//! 一次微信支付接口请求的请求/响应/错误信息。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.WxPayApiData`（Java 以
//! `ThreadLocal<WxPayApiData>` 保存，Rust 以 impl 内 `RwLock` 保存）。
//! Wave 0 已定型（生成器 HAND_WRITTEN 保护，不覆盖）。

/// 一次微信支付接口请求的请求/响应/错误信息。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WxPayApiData {
    /// 请求地址（对应 Java `url`）
    pub url: Option<String>,
    /// 请求数据（对应 Java `requestData`）
    pub request_data: Option<String>,
    /// 响应数据（对应 Java `responseData`）
    pub response_data: Option<String>,
    /// 错误信息（对应 Java `exceptionMsg`）
    pub error_msg: Option<String>,
}

impl WxPayApiData {
    /// 构建接口请求数据。
    pub fn new(
        url: Option<String>,
        request_data: Option<String>,
        response_data: Option<String>,
        error_msg: Option<String>,
    ) -> Self {
        Self {
            url,
            request_data,
            response_data,
            error_msg,
        }
    }
}
