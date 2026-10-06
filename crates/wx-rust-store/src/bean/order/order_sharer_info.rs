//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderSharerInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderSharerInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderSharerInfo.java。
pub struct OrderSharerInfo {
    #[serde(rename = "sharer_openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_openid: Option<String>,
    #[serde(rename = "sharer_unionid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_unionid: Option<String>,
    #[serde(rename = "sharer_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_type: Option<i32>,
    #[serde(rename = "share_scene", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_scene: Option<i32>,
    #[serde(rename = "handling_progress", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handling_progress: Option<i32>,
}
