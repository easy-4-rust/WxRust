//! 对应 Java `com.binarywang.wxjava.store.bean.window.request.AddWindowProductRequest.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AddWindowProductRequest；对应 Java com.binarywang.wxjava.store.bean.window.request.AddWindowProductRequest.java。
pub struct AddWindowProductRequest {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "appid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    #[serde(rename = "is_hide_for_window", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hide_for_window: Option<bool>,
}
