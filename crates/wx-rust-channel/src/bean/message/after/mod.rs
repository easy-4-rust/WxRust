//! 视频号小店 售后/纠纷 回调消息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.after` 包。

pub mod after_sale_message;
pub mod after_sale_status_info;
pub mod complaint_info;
pub mod complaint_message;

pub use after_sale_message::AfterSaleMessage;
pub use after_sale_status_info::AfterSaleStatusInfo;
pub use complaint_info::ComplaintInfo;
pub use complaint_message::ComplaintMessage;
