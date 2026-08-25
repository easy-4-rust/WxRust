//! 对应 Java `me.chanjar.weixin.channel.bean.ewaybill.AddSubOrderRequest.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddSubOrderRequest {
    /// 运单 ID
    #[serde(rename = "ewaybill_order_id", default)]
    pub ewaybill_order_id: String,
    /// 子件信息
    #[serde(rename = "sub_order_list", default)]
    pub sub_order_list: Vec<SubOrderInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubOrderInfo {
    /// 子件运单 ID
    #[serde(rename = "sub_order_id", default)]
    pub sub_order_id: String,
}
