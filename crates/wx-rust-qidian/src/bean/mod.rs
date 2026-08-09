//! 腾讯企点数据对象。
//!
//! 对应 Java `me.chanjar.weixin.qidian.bean` 包：`call`（通话数据）、
//! `common`（响应基类）、`dial`（IVR 呼叫）三个子包与 `WxQidianHostConfig`
//! 域名配置。serde 派生替代 Gson 数据绑定；Gson 默认省略 null 字段，
//! 以 `#[serde(skip_serializing_if = "Option::is_none")]` 对齐。

pub mod call;
pub mod common;
pub mod dial;
pub mod wx_qidian_host_config;

pub use wx_qidian_host_config::{
    API_DEFAULT_HOST_URL, OPEN_DEFAULT_HOST_URL, QIDIAN_DEFAULT_HOST_URL, WxQidianHostConfig,
};
