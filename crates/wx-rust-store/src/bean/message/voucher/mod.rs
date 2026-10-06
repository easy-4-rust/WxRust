//! 微信小店 团购券 回调消息。
//!
//! 对应 Java `com.binarywang.wxjava.store.bean.message.voucher` 包。

pub mod voucher_info;
pub mod voucher_message;

pub use voucher_info::VoucherInfo;
pub use voucher_message::VoucherMessage;
