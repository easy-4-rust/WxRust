//! WxRust 开放平台（微信第三方平台）模块（对应 WxJava `weixin-java-open`）。
//!
//! 覆盖 `me.chanjar.weixin.open.*` 的基础设施：
//! - 门面服务（`WxOpenService`）：component_access_token 双检锁缓存
//!   （用 component_verify_ticket 换 component_access_token）、GET/POST
//!   执行引擎与 token 自动刷新（与 mp/ma 同一模式）；
//! - 组件子服务（`WxOpenComponentService`）：预授权码、授权事件、代
//!   公众号/小程序服务桥接的签名冻结骨架；
//! - 配置存储（`WxOpenConfigStorage` / `WxOpenDefaultConfigImpl`）、
//!   回调消息加解密（`WxOpenCryptUtils`）、URL 常量等。
//!
//! 业务子域（授权信息、代 mp/ma 服务、小程序管理、minishop 等）随
//! 后续波次按迁移路线图实现；`bean` 模块当前仅包含 token 核心 bean。

#![forbid(unsafe_code)]

pub mod api;
pub mod bean;
pub mod config;
pub mod constant;
pub mod enums;
pub mod util;
