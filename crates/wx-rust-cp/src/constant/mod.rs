//! 企业微信常量。
//!
//! 对应 Java `me.chanjar.weixin.cp.constant` 包（`WxCpConsts` 全量常量 +
//! `WxCpTpConsts` 服务商常量 + `WxCpApiPathConsts` API URL 常量；API URL
//! 按子域拆分在 `crate::enums::url_*`）。

pub mod wx_cp_constants;
pub mod wx_cp_tp_consts;

pub use wx_cp_constants::*;
