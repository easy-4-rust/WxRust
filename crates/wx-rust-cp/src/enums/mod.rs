//! 企业微信枚举与常量。
//!
//! Java 的 `WxCpApiPathConsts`（接口 + 内部接口常量）在 Rust 中按子域
//! 拆分为 `url_*.rs` 模块，常量命名与 Java 完全一致（SCREAMING_SNAKE）；
//! 完整接口地址由 `WxCpConfigStorage::api_url(path)` 拼接（baseUrl +
//! path，对应 Java `configStorage.getApiUrl(...)`）。
//!
//! 说明：Java 源码中 `WxCpApiPathConsts.Oa` 内部类的 WEDOC/微盘常量全部
//! 平铺在 `url_oa.rs`（与 Java 同一文件内部类平铺结构一致）；子域模块
//! 命名采用 `url_<子域>`，后续批次新增子域时追加 `pub mod url_*` 与
//! `pub use url_*::*` 行（与 miniapp `enums/g*_urls.rs` 注册模式一致）。

pub mod url_agent;
pub mod url_chat;
pub mod url_core;
pub mod url_corp_group;
pub mod url_department;
pub mod url_export;
pub mod url_external_contact;
pub mod url_hr;
pub mod url_id_convert;
pub mod url_intelligent_robot;
pub mod url_kf;
pub mod url_license;
pub mod url_linked_corp;
pub mod url_living;
pub mod url_media;
pub mod url_menu;
pub mod url_message;
pub mod url_msg_audit;
pub mod url_oa;
pub mod url_oauth2;
pub mod url_school;
pub mod url_tag;
pub mod url_task_card;
pub mod url_todo;
pub mod url_tp;
pub mod url_user;

pub use url_agent::{agent, work_bench};
pub use url_core::{
    BATCH_GET_RESULT, BATCH_REPLACE_PARTY, BATCH_REPLACE_USER, BATCH_SYNC_USER,
    DEFAULT_CP_BASE_URL, GET_AGENT_CONFIG_TICKET, GET_API_DOMAIN_IP, GET_CALLBACK_IP,
    GET_JSAPI_TICKET, GET_TOKEN, JSCODE_TO_SESSION, WEBHOOK_SEND,
};
