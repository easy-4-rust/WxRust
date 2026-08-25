//! 对应 Java `me.chanjar.weixin.channel.bean.ewaybill.BatchPrintOrderRequest.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchPrintOrderRequest {
    /// 运单 ID 列表
    #[serde(rename = "ewaybill_order_ids", default)]
    pub ewaybill_order_ids: Vec<String>,
    /// 模板 ID（可选）
    #[serde(rename = "template_id", default)]
    pub template_id: String,
}
