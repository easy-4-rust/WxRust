//! 对应 Java `com.binarywang.wxjava.store.bean.shop.ShopInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ShopInfo；对应 Java com.binarywang.wxjava.store.bean.shop.ShopInfo.java。
pub struct ShopInfo {
    #[serde(rename = "nickname", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(rename = "headimg_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_img_url: Option<String>,
    #[serde(rename = "subject_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<String>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "username", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
