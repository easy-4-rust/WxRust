//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.ShopLiveData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ShopLiveData；对应 Java com.binarywang.wxjava.store.bean.compass.shop.ShopLiveData.java。
pub struct ShopLiveData {
    #[serde(rename = "live_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_id: Option<String>,
    #[serde(rename = "live_title", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_title: Option<String>,
    #[serde(rename = "live_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_time: Option<String>,
    #[serde(rename = "live_duration", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_duration: Option<String>,
    #[serde(rename = "live_cover_img_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_cover_img_url: Option<String>,
}
