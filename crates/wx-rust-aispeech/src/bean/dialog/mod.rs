//! 对话机器人数据对象。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.bean.dialog` 包。

pub mod aispeech_api_response;
pub mod async_task_result;
pub mod bot_intent;
pub mod dialog_query_request;
pub mod dialog_result;
pub mod publish_progress;

pub use aispeech_api_response::AispeechApiResponse;
pub use async_task_result::AsyncTaskResult;
pub use bot_intent::BotIntent;
pub use dialog_query_request::DialogQueryRequest;
pub use dialog_result::DialogResult;
pub use publish_progress::PublishProgress;
