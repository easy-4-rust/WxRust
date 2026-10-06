//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.BatchPrintOrderRequest.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 BatchPrintOrderRequest；对应 Java com.binarywang.wxjava.store.bean.ewaybill.BatchPrintOrderRequest.java。
pub struct BatchPrintOrderRequest {
    /// 批量打印请求。对应 Java: BatchPrintOrderRequest#reqList。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_list: Option<Vec<super::PrintOrderRequest>>,
    /// 运单 ID 列表
    #[serde(rename = "ewaybill_order_ids", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ewaybill_order_ids: Option<Vec<String>>,
    /// 模板 ID（可选）
    #[serde(rename = "template_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}
