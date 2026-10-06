//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AfterSaleListParam；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleListParam.java。
pub struct AfterSaleListParam {
    #[serde(rename = "begin_create_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_create_time: Option<i64>,
    #[serde(rename = "end_create_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_create_time: Option<i64>,
    #[serde(rename = "begin_update_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_update_time: Option<i64>,
    #[serde(rename = "end_update_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_update_time: Option<i64>,
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
}
