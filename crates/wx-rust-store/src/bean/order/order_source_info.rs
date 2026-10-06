//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderSourceInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderSourceInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderSourceInfo.java。
pub struct OrderSourceInfo {
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "account_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<i32>,
    #[serde(rename = "account_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "sale_channel", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_channel: Option<i32>,
    #[serde(rename = "account_nickname", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_nickname: Option<String>,
    #[serde(rename = "content_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "content_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "promoter_head_supplier_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promoter_head_supplier_id: Option<String>,
}
