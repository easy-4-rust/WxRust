//! 售后信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.after.AfterSaleStatusInfo.java`。

use serde::{Deserialize, Serialize};

/// 售后信息（对应 Java `AfterSaleStatusInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AfterSaleStatusInfo {
    /// 售后单号（对应 Java `afterSaleOrderId`）。
    #[serde(rename = "after_sale_order_id", default)]
    pub after_sale_order_id: Option<String>,
    /// 售后单状态（对应 Java `status`）。
    #[serde(rename = "status", default)]
    pub status: Option<String>,
    /// 订单id（对应 Java `orderId`）。
    #[serde(rename = "order_id", default)]
    pub order_id: Option<String>,
}
