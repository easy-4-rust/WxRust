//! 企业微信企业互联服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.corpgroup.service` 包：`WxCpCgService`
//! （集团服务：corp access token + 通用执行通道 + 小程序 session）与
//! `WxCpLinkedCorpService`（互联企业服务）。

pub mod wx_cp_cg_service;
pub mod wx_cp_linked_corp_service;

pub mod r#impl;

pub use wx_cp_cg_service::WxCpCgService;
pub use wx_cp_linked_corp_service::WxCpLinkedCorpService;
