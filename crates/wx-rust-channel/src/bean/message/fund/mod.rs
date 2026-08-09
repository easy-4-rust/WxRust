//! 视频号小店 资金 回调消息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.fund` 包。

pub mod account_notify_message;
pub mod bank_notify_info;
pub mod qr_notify_info;
pub mod qr_notify_message;
pub mod withdraw_notify_info;
pub mod withdraw_notify_message;

pub use account_notify_message::AccountNotifyMessage;
pub use bank_notify_info::BankNotifyInfo;
pub use qr_notify_info::QrNotifyInfo;
pub use qr_notify_message::QrNotifyMessage;
pub use withdraw_notify_info::WithdrawNotifyInfo;
pub use withdraw_notify_message::WithdrawNotifyMessage;
