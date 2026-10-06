//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderCommissionInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderCommissionInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderCommissionInfo.java。
pub struct OrderCommissionInfo {
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "nickname", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(rename = "type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "amount", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "finder_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finder_id: Option<String>,
    #[serde(rename = "openfinderid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_finder_id: Option<String>,
    #[serde(rename = "talent_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
}
