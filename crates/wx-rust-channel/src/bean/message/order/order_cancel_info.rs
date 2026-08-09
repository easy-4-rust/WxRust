//! 订单取消信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.order.OrderCancelInfo.java`
//! （继承 `OrderIdInfo`；Rust 扁平展开）。

use serde::{Deserialize, Serialize};

/// 订单取消信息（对应 Java `OrderCancelInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderCancelInfo {
    /// 订单ID（对应 Java 继承自 `OrderIdInfo.orderId`）。
    #[serde(rename = "order_id", default)]
    pub order_id: Option<String>,
    /// 1:用户取消；2:超时取消；3:全部商品售后完成,订单取消；4:超卖商家取消订单
    /// （对应 Java `cancelType`）。
    #[serde(
        rename = "cancel_type",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub cancel_type: Option<i32>,
}
