//! 客服消息 builder（对应 Java `me.chanjar.weixin.mp.builder.kefu` 包）。

pub mod base_builder;
pub mod image_builder;
pub mod mini_program_page_builder;
pub mod mp_news_article_builder;
pub mod mp_news_builder;
pub mod music_builder;
pub mod news_builder;
pub mod text_builder;
pub mod video_builder;
pub mod voice_builder;
pub mod wx_card_builder;
pub mod wx_msg_menu_builder;

pub use base_builder::{KefuMessageBuilder, MsgMenu, WxArticle, WxMpKefuMessage};
pub use image_builder::ImageBuilder;
pub use mini_program_page_builder::MiniProgramPageBuilder;
pub use mp_news_article_builder::MpNewsArticleBuilder;
pub use mp_news_builder::MpNewsBuilder;
pub use music_builder::MusicBuilder;
pub use news_builder::NewsBuilder;
pub use text_builder::TextBuilder;
pub use video_builder::VideoBuilder;
pub use voice_builder::VoiceBuilder;
pub use wx_card_builder::WxCardBuilder;
pub use wx_msg_menu_builder::WxMsgMenuBuilder;
