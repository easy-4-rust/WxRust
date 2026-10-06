//! 对应 Java `com.binarywang.wxjava.store.bean.home.banner.BannerItem.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{BannerItemDetail, BannerItemFinder, BannerItemOfficialAccount, BannerItemProduct};

/// 微信小店 BannerItem 数据类型；对应 Java com.binarywang.wxjava.store.bean.home.banner.BannerItem.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BannerItem {
    #[serde(rename = "type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(rename = "banner", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<BannerItemDetail>,
    #[serde(rename = "product", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<BannerItemProduct>,
    #[serde(rename = "finder", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finder: Option<BannerItemFinder>,
    #[serde(rename = "official_account", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub official_account: Option<BannerItemOfficialAccount>,
}
