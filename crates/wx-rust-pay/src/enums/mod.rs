//! 微信支付枚举。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.result.enums` 中的
//! `TradeTypeEnum`/`GlobalTradeTypeEnum` 与 `WxPayConstants.SignType`
//! （Java 侧无顶层 enums 包，已确认不存在 `WxPayType`/`WxPayApiType`
//! 枚举；v2 接口路径常量集中在本模块 `pay_url`）。

pub mod global_trade_type;
pub mod pay_url;
pub mod sign_type;
pub mod trade_type;

pub use global_trade_type::GlobalTradeTypeEnum;
pub use pay_url::*;
pub use sign_type::SignType;
pub use trade_type::TradeTypeEnum;
