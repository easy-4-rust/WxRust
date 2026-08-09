//! 订单其他信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.order.OrderExtInfo.java`
//! （继承 `OrderIdInfo`；Rust 扁平展开）。

use serde::{Deserialize, Serialize};

/// 订单其他信息（对应 Java `OrderExtInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderExtInfo {
    /// 订单ID（对应 Java 继承自 `OrderIdInfo.orderId`）。
    #[serde(rename = "order_id", default)]
    pub order_id: Option<String>,
    /// 类型 1:联盟佣金信息（对应 Java `type`）。
    #[serde(
        rename = "type",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub r#type: Option<i32>,
}
