//! 对应 Java `me.chanjar.weixin.channel.bean.ewaybill.PrintOrderRequest.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrintOrderRequest {
    /// 运单 ID
    #[serde(rename = "ewaybill_order_id", default)]
    pub ewaybill_order_id: String,
}
