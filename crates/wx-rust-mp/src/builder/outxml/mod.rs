//! 被动回复消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.mp.builder.outxml` 包。

pub mod base_builder;
pub mod image_builder;
pub mod music_builder;
pub mod news_builder;
pub mod text_builder;
pub mod transfer_biz_ai_ivr_builder;
pub mod transfer_customer_service_builder;
pub mod video_builder;
pub mod voice_builder;

pub use base_builder::BaseBuilder;
pub use image_builder::ImageBuilder;
pub use music_builder::MusicBuilder;
pub use news_builder::NewsBuilder;
pub use text_builder::TextBuilder;
pub use transfer_biz_ai_ivr_builder::TransferBizAiIvrBuilder;
pub use transfer_customer_service_builder::TransferCustomerServiceBuilder;
pub use video_builder::VideoBuilder;
pub use voice_builder::VoiceBuilder;
pub mod device_builder;
pub use device_builder::DeviceBuilder;
