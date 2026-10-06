//! 对应 Java `com.binarywang.wxjava.store.bean.order.ChangeSkuInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ChangeSkuInfo；对应 Java com.binarywang.wxjava.store.bean.order.ChangeSkuInfo.java。
pub struct ChangeSkuInfo {
    #[serde(rename = "preshipment_change_sku_state", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preshipment_change_sku_state: Option<i32>,
    #[serde(rename = "old_sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_sku_id: Option<String>,
    #[serde(rename = "new_sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sku_id: Option<String>,
    #[serde(rename = "ddl_time_stamp", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline_time_stamp: Option<i32>,
}
