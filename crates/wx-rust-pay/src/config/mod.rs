//! 微信支付配置存储。
//!
//! 对应 Java `com.github.binarywang.wxpay.config` 包。

pub mod r#impl;
pub mod verifier_builder;
pub mod wx_pay_config;
pub mod wx_pay_config_holder;
pub mod wx_pay_http_proxy;

pub use verifier_builder::{build_public_cert_verifier, build_verifier};
pub use wx_pay_config::{DEFAULT_PAY_BASE_URL, WxPayConfig};
pub use wx_pay_http_proxy::WxPayHttpProxy;
