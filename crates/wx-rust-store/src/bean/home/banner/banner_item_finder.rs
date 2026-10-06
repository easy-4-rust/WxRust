//! 对应 Java `com.binarywang.wxjava.store.bean.home.banner.BannerItemFinder.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 BannerItemFinder；对应 Java com.binarywang.wxjava.store.bean.home.banner.BannerItemFinder.java。
pub struct BannerItemFinder {
    #[serde(rename = "finder_user_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finder_user_name: Option<String>,
    #[serde(rename = "feed_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_id: Option<String>,
}
