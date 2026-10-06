//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderCustomInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderCustomInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderCustomInfo.java。
pub struct OrderCustomInfo {
    #[serde(rename = "custom_img_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_img_url: Option<String>,
    #[serde(rename = "custom_word", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_word: Option<String>,
    #[serde(rename = "custom_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_type: Option<i32>,
    #[serde(rename = "custom_preview_img_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_preview_img_url: Option<String>,
}
