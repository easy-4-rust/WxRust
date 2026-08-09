//! 被动回复消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.outxmlbuilder` 子包。用法：
//! `WxCpXmlOutMessage::text().to_user("..").from_user("..").content("..").build()`。

pub mod base_builder;
pub mod event_builder;
pub mod image_builder;
pub mod news_builder;
pub mod task_card_builder;
pub mod text_builder;
pub mod update_button_builder;
pub mod video_builder;
pub mod voice_builder;

pub use base_builder::BaseBuilder;
pub use event_builder::EventBuilder;
pub use image_builder::ImageBuilder;
pub use news_builder::NewsBuilder;
pub use task_card_builder::TaskCardBuilder;
pub use text_builder::TextBuilder;
pub use update_button_builder::UpdateButtonBuilder;
pub use video_builder::VideoBuilder;
pub use voice_builder::VoiceBuilder;
