//! 订单确认收货信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.order.OrderConfirmInfo.java`
//! （继承 `OrderIdInfo`；Rust 扁平展开）。

use serde::{Deserialize, Serialize};

/// 订单确认收货信息（对应 Java `OrderConfirmInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderConfirmInfo {
    /// 订单ID（对应 Java 继承自 `OrderIdInfo.orderId`）。
    #[serde(rename = "order_id", default)]
    pub order_id: Option<String>,
    /// 1:用户确认收货；2:超时自动确认收货（对应 Java `confirmType`）。
    #[serde(
        rename = "confirm_type",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub confirm_type: Option<i32>,
}
