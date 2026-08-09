//! 企业微信配置存储。
//!
//! 对应 Java `me.chanjar.weixin.cp.config` 包（`WxCpConfigStorage` +
//! 企业互联 `WxCpCorpGroupConfigStorage` + 第三方应用 `WxCpTpConfigStorage`）。

pub mod r#impl;
pub mod wx_cp_config_storage;
pub mod wx_cp_corp_group_config_storage;
pub mod wx_cp_host_config;
pub mod wx_cp_tp_config_storage;

pub use crate::enums::url_core::DEFAULT_CP_BASE_URL;
pub use wx_cp_config_storage::WxCpConfigStorage;
pub use wx_cp_corp_group_config_storage::WxCpCorpGroupConfigStorage;
pub use wx_cp_host_config::{MP_DEFAULT_HOST_URL, OPEN_DEFAULT_HOST_URL, WxCpHostConfig};
pub use wx_cp_tp_config_storage::WxCpTpConfigStorage;
