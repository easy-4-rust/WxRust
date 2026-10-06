//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.ExtInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ExtInfo；对应 Java com.binarywang.wxjava.store.bean.coupon.ExtInfo.java。
pub struct ExtInfo {
    #[serde(rename = "jump_product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_product_id: Option<String>,
    #[serde(rename = "notes", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "valid_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_time: Option<i64>,
    #[serde(rename = "invalid_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_time: Option<i64>,
}
