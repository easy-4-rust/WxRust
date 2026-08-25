//! 对应 Java `me.chanjar.weixin.channel.bean.supplier.DropshipInfo.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DropshipInfo {
    /// 订单号
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// 供货商 ID
    #[serde(rename = "supplier_id", default)]
    pub supplier_id: String,
    /// 代发单状态
    #[serde(rename = "status", default)]
    pub status: i32,
}
