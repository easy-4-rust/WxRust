//! 纠纷信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.after.ComplaintInfo.java`。

use serde::{Deserialize, Serialize};

/// 纠纷信息（对应 Java `ComplaintInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComplaintInfo {
    /// 纠纷单号（对应 Java `complaintId`）。
    #[serde(rename = "complaint_id", default)]
    pub complaint_id: Option<String>,
    /// 小店售后单号（对应 Java `afterSaleOrderId`）。
    #[serde(rename = "after_sale_order_id", default)]
    pub after_sale_order_id: Option<String>,
    /// 纠纷单状态（对应 Java `status`）。
    #[serde(
        rename = "status",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub status: Option<i32>,
}
