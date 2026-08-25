//! 对应 Java `me.chanjar.weixin.channel.bean.qic.SubmitInspectRequest.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubmitInspectRequest {
    /// 订单号
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// 质检码
    #[serde(rename = "inspect_code", default)]
    pub inspect_code: String,
}
