//! 支付方式枚举（v3 下单）。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.result.enums.TradeTypeEnum`：
//! 直连/合单/服务商三类 v3 下单 URL 随枚举携带（Java 构造器字段）。

/// 支付方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradeTypeEnum {
    /// APP
    App,
    /// JSAPI 或小程序
    Jsapi,
    /// NATIVE
    Native,
    /// H5
    H5,
}

impl TradeTypeEnum {
    /// 直连商户支付 url（对应 Java `getMerchantUrl()`）。
    pub fn merchant_url(&self) -> &'static str {
        match self {
            TradeTypeEnum::App => "/v3/pay/transactions/app",
            TradeTypeEnum::Jsapi => "/v3/pay/transactions/jsapi",
            TradeTypeEnum::Native => "/v3/pay/transactions/native",
            TradeTypeEnum::H5 => "/v3/pay/transactions/h5",
        }
    }

    /// 合并下单 url（对应 Java `getCombineUrl()`）。
    pub fn combine_url(&self) -> &'static str {
        match self {
            TradeTypeEnum::App => "/v3/combine-transactions/app",
            TradeTypeEnum::Jsapi => "/v3/combine-transactions/jsapi",
            TradeTypeEnum::Native => "/v3/combine-transactions/native",
            TradeTypeEnum::H5 => "/v3/combine-transactions/h5",
        }
    }

    /// 服务商支付 url（对应 Java `getPartnerUrl()`）。
    pub fn partner_url(&self) -> &'static str {
        match self {
            TradeTypeEnum::App => "/v3/pay/partner/transactions/app",
            TradeTypeEnum::Jsapi => "/v3/pay/partner/transactions/jsapi",
            TradeTypeEnum::Native => "/v3/pay/partner/transactions/native",
            TradeTypeEnum::H5 => "/v3/pay/partner/transactions/h5",
        }
    }
}
