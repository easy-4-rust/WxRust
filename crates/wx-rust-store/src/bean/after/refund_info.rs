//! 对应 Java `com.binarywang.wxjava.store.bean.after.RefundInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 RefundInfo；对应 Java com.binarywang.wxjava.store.bean.after.RefundInfo.java。
pub struct RefundInfo {
    #[serde(rename = "amount", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "refund_reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_reason: Option<i32>,
}
