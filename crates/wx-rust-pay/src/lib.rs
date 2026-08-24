//! WxRust 微信支付模块（对应 WxJava `weixin-java-pay`）。
//!
//! 覆盖 `com.github.binarywang.wxpay.*`：门面服务 `WxPayService`（v2 XML 签名
//! MD5/HMAC-SHA256 + v3 JSON RSA 验签、GET/POST 执行引擎、多商户配置切换）、
//! 配置存储、签名常量/枚举、bean 序列化等基础设施；支付分/分账/红包/转账等
//! 业务子域由后续波次实现。
//!
//! Wave 0 为签名冻结骨架：`api/wx_pay_service.rs` 已按 Java `WxPayService`
//! 接口逐方法镜像（139 个方法全部出现在 trait 中，未实现者默认返回
//! `-99 未实现` 错误）；`bean/` 为占位 stub（Wave 1 生成 serde 派生真实 bean）。

#![forbid(unsafe_code)]

pub mod api;
pub mod bean;
pub mod builder;
pub mod config;
pub mod constant;
pub mod enums;
pub mod exception;
pub mod util;
pub mod v3;
