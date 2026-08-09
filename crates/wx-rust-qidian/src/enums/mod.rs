//! 腾讯企点枚举。
//!
//! 对应 Java `me.chanjar.weixin.qidian.enums` 包（`WxQidianApiUrl`）。

pub mod wx_qidian_api_url;

pub use wx_qidian_api_url::{ApiUrl, call_data, dial, o_auth2, other};
