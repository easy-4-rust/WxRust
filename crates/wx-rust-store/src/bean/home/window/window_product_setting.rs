//! 对应 Java `com.binarywang.wxjava.store.bean.home.window.WindowProductSetting.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 WindowProductSetting；对应 Java com.binarywang.wxjava.store.bean.home.window.WindowProductSetting.java。
pub struct WindowProductSetting {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "is_set_hide", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_hide: Option<i32>,
    #[serde(rename = "is_set_top", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_top: Option<i32>,
}
