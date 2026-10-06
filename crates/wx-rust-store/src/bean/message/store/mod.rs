//! 微信小店 店铺 回调消息。
//!
//! 对应 Java `com.binarywang.wxjava.store.bean.message.store` 包。

pub mod close_store_message;
pub mod nickname_update_message;

pub use close_store_message::CloseStoreMessage;
pub use nickname_update_message::NicknameUpdateMessage;
