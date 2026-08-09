//! 对应 Java `me.chanjar.weixin.channel.bean.home.banner.BannerItem.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::home::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BannerItem {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "banner", default)]
    pub banner: BannerItemDetail,
    #[serde(rename = "product", default)]
    pub product: BannerItemProduct,
    #[serde(rename = "finder", default)]
    pub finder: BannerItemFinder,
    #[serde(rename = "official_account", default)]
    pub official_account: BannerItemOfficialAccount,
}
