//! 视频号小店 团购券 回调消息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.voucher` 包。

pub mod voucher_info;
pub mod voucher_message;

pub use voucher_info::VoucherInfo;
pub use voucher_message::VoucherMessage;
