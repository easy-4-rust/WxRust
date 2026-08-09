//! 视频号小店 商品 回调消息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.product` 包。

pub mod brand_message;
pub mod category_audit_message;
pub mod spu_audit_message;
pub mod spu_status_message;
pub mod spu_stock_message;

pub use brand_message::BrandMessage;
pub use category_audit_message::CategoryAuditMessage;
pub use spu_audit_message::SpuAuditMessage;
pub use spu_status_message::SpuStatusMessage;
pub use spu_stock_message::SpuStockMessage;
