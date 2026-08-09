//! 消息构建器（发送企业微信应用消息）。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.messagebuilder` 子包。用法：
//! `WxCpMessage::text().to_user("..").content("..").build()`。

pub mod base_builder;
pub mod file_builder;
pub mod image_builder;
pub mod markdown_msg_builder;
pub mod mini_program_notice_msg_builder;
pub mod mpnews_builder;
pub mod news_builder;
pub mod task_card_builder;
pub mod template_card_builder;
pub mod text_builder;
pub mod text_card_builder;
pub mod video_builder;
pub mod voice_builder;

pub use base_builder::BaseBuilder;
pub use file_builder::FileBuilder;
pub use image_builder::ImageBuilder;
pub use markdown_msg_builder::MarkdownMsgBuilder;
pub use mini_program_notice_msg_builder::MiniProgramNoticeMsgBuilder;
pub use mpnews_builder::MpnewsBuilder;
pub use news_builder::NewsBuilder;
pub use task_card_builder::TaskCardBuilder;
pub use template_card_builder::TemplateCardBuilder;
pub use text_builder::TextBuilder;
pub use text_card_builder::TextCardBuilder;
pub use video_builder::VideoBuilder;
pub use voice_builder::VoiceBuilder;
