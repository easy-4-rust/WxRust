//! 视频号小店常量。
//!
//! 对应 Java `me.chanjar.weixin.channel.constant` 包。接口地址常量按子域
//! 拆分至 `crate::enums::url_*`（与 miniapp 一致的布局约定）。

pub mod wx_channel_message_event_constants;

pub use wx_channel_message_event_constants::MessageEventConstants;
