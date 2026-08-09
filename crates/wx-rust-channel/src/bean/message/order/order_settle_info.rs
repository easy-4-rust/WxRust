//! 订单结算信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.order.OrderSettleInfo.java`
//! （继承 `OrderIdInfo`；Rust 扁平展开）。

use serde::{Deserialize, Serialize};

/// 订单结算信息（对应 Java `OrderSettleInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderSettleInfo {
    /// 订单ID（对应 Java 继承自 `OrderIdInfo.orderId`）。
    #[serde(rename = "order_id", default)]
    pub order_id: Option<String>,
    /// 结算时间（对应 Java `settleTime`）。
    #[serde(
        rename = "settle_time",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub settle_time: Option<i64>,
}
