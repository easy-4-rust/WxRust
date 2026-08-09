//! 智能对话 API。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.api` 包：门面服务 `WxAispeechService`
//! 与其承载的执行引擎（Java `WxAispeechServiceImpl` 的请求头签名体系），
//! 以及 `WxAispeechDialogService` / `WxAispeechKnowledgeService` 两个子域。

pub mod r#impl;
pub mod wx_aispeech_dialog_service;
pub mod wx_aispeech_knowledge_service;
pub mod wx_aispeech_service;

pub use wx_aispeech_dialog_service::WxAispeechDialogService;
pub use wx_aispeech_knowledge_service::WxAispeechKnowledgeService;
pub use wx_aispeech_service::WxAispeechService;
