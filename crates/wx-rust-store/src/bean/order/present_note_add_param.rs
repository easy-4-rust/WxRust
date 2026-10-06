//! 对应 Java `com.binarywang.wxjava.store.bean.order.PresentNoteAddParam.java`。

/// 礼物订单新增备注信息请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PresentNoteAddParam {
    /// 礼物订单 ID。
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 备注内容。
    #[serde(rename = "note", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}
