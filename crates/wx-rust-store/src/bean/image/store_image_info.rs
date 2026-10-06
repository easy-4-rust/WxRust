//! 对应 Java `com.binarywang.wxjava.store.bean.image.StoreImageInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 StoreImageInfo；对应 Java com.binarywang.wxjava.store.bean.image.StoreImageInfo.java。
pub struct StoreImageInfo {
    #[serde(rename = "media_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    #[serde(rename = "img_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "pay_media_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_media_id: Option<String>,
}
