//! 对应 Java `com.binarywang.wxjava.store.bean.fund.FlowRelatedInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 FlowRelatedInfo；对应 Java com.binarywang.wxjava.store.bean.fund.FlowRelatedInfo.java。
pub struct FlowRelatedInfo {
    #[serde(rename = "related_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_type: Option<i32>,
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "aftersale_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_id: Option<String>,
    #[serde(rename = "withdraw_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_id: Option<String>,
    #[serde(rename = "bookkeeping_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookkeeping_time: Option<String>,
    #[serde(rename = "insurance_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_id: Option<String>,
    #[serde(rename = "transaction_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}
