//! 对应 Java `com.binarywang.wxjava.store.bean.home.banner.BannerInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::BannerItem;

/// 微信小店 BannerInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.home.banner.BannerInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BannerInfo {
    #[serde(rename = "scale", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<i32>,
    #[serde(rename = "banner", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<Vec<BannerItem>>,
}
