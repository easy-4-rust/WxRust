//! 微信支付配置存储。
//!
//! 对应 Java `com.github.binarywang.wxpay.config` 包。

pub mod r#impl;
pub mod wx_pay_config;

pub use wx_pay_config::{DEFAULT_PAY_BASE_URL, WxPayConfig};
