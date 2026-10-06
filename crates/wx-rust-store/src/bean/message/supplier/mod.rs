//! 微信小店 团长商品 回调消息。
//!
//! 对应 Java `com.binarywang.wxjava.store.bean.message.supplier` 包。

pub mod supplier_item_info;
pub mod supplier_item_message;

pub use supplier_item_info::SupplierItemInfo;
pub use supplier_item_message::SupplierItemMessage;
