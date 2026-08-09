//! 客服消息 builder。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.builder` 包：`BaseBuilder` +
//! `TextMessageBuilder`/`ImageMessageBuilder`/`LinkMessageBuilder`/
//! `MaPageMessageBuilder`（消息类型常量见 Java `WxMaConstants.KefuMsgType`）。

pub mod base_builder;
pub mod image_message_builder;
pub mod link_message_builder;
pub mod ma_page_message_builder;
pub mod text_message_builder;

pub use base_builder::BaseBuilder;
pub use image_message_builder::ImageMessageBuilder;
pub use link_message_builder::LinkMessageBuilder;
pub use ma_page_message_builder::MaPageMessageBuilder;
pub use text_message_builder::TextMessageBuilder;
