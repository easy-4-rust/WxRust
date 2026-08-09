//! 客服消息 bean。

pub mod wx_mp_kefu_message;

pub use wx_mp_kefu_message::{KefuMessageBuilder, MsgMenu, WxArticle, WxMpKefuMessage};
pub mod request;
pub mod result;
pub use request::*;
pub use result::*;
