//! 对应 Java `com.binarywang.wxjava.store.bean.after.ReturnInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ReturnInfo；对应 Java com.binarywang.wxjava.store.bean.after.ReturnInfo.java。
pub struct ReturnInfo {
    #[serde(rename = "waybill_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waybill_id: Option<String>,
    #[serde(rename = "delivery_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    #[serde(rename = "delivery_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_name: Option<String>,
}
