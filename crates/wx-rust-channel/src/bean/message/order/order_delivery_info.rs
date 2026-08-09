//! 订单发货信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.order.OrderDeliveryInfo.java`
//! （继承 `OrderIdInfo`；Rust 扁平展开）。

use serde::{Deserialize, Serialize};

/// 订单发货信息（对应 Java `OrderDeliveryInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderDeliveryInfo {
    /// 订单ID（对应 Java 继承自 `OrderIdInfo.orderId`）。
    #[serde(rename = "order_id", default)]
    pub order_id: Option<String>,
    /// 0:尚未全部发货；1:全部商品发货完成（对应 Java `finishDelivery`）。
    #[serde(
        rename = "finish_delivery",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub finish_delivery: Option<i32>,
}
