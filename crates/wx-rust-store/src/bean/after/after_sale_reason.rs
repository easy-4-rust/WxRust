//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleReason.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AfterSaleReason；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleReason.java。
pub struct AfterSaleReason {
    #[serde(rename = "reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<i32>,
    #[serde(rename = "reason_text", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_text: Option<String>,
}
