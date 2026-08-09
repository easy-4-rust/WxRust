//! 微信支付配置存储默认实现。
//!
//! 对应 Java `com.github.binarywang.wxpay.config.impl`（Java 侧 WxPayConfig
//! 即默认实现，无独立 impl 包；Rust 遵循 miniapp 的 config/impl 结构）。

pub mod wx_pay_default_config_impl;

pub use wx_pay_default_config_impl::WxPayDefaultConfig;
