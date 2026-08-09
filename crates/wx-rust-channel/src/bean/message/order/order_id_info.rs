//! 订单id信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.order.OrderIdInfo.java`
//! （订单类 Info 的基类；Java 继承在 Rust 中扁平展开为同层字段）。

use serde::{Deserialize, Serialize};

/// 订单id信息（对应 Java `OrderIdInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderIdInfo {
    /// 订单ID（对应 Java `orderId`）。
    #[serde(rename = "order_id", default)]
    pub order_id: Option<String>,
}
