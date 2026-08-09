//! 智能对话服务实现。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.api.impl` 包。

pub mod wx_aispeech_dialog_service_impl;
pub mod wx_aispeech_knowledge_service_impl;
pub mod wx_aispeech_service_impl;

pub use wx_aispeech_dialog_service_impl::WxAispeechDialogServiceImpl;
pub use wx_aispeech_knowledge_service_impl::WxAispeechKnowledgeServiceImpl;
pub use wx_aispeech_service_impl::WxAispeechServiceImpl;
