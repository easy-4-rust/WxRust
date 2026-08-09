//! 订单支付信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.order.OrderPayInfo.java`
//! （继承 `OrderIdInfo`；Rust 扁平展开）。

use serde::{Deserialize, Serialize};

/// 订单支付信息（对应 Java `OrderPayInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderPayInfo {
    /// 订单ID（对应 Java 继承自 `OrderIdInfo.orderId`）。
    #[serde(rename = "order_id", default)]
    pub order_id: Option<String>,
    /// 支付时间，秒级时间戳（对应 Java `payTime`）。
    #[serde(
        rename = "pay_time",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub pay_time: Option<i64>,
}
