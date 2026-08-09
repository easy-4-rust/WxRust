//! 境外微信支付方式枚举（v3 下单）。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.result.enums.GlobalTradeTypeEnum`。

/// 境外微信支付方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalTradeTypeEnum {
    /// APP
    App,
    /// JSAPI 或小程序
    Jsapi,
    /// NATIVE
    Native,
    /// H5
    H5,
}

impl GlobalTradeTypeEnum {
    /// 境外下单 url（对应 Java `getUrl()`）。
    pub fn url(&self) -> &'static str {
        match self {
            GlobalTradeTypeEnum::App => "/global/v3/transactions/app",
            GlobalTradeTypeEnum::Jsapi => "/global/v3/transactions/jsapi",
            GlobalTradeTypeEnum::Native => "/global/v3/transactions/native",
            GlobalTradeTypeEnum::H5 => "/global/v3/transactions/h5",
        }
    }
}
