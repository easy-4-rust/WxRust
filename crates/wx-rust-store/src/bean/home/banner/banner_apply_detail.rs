//! 对应 Java `com.binarywang.wxjava.store.bean.home.banner.BannerApplyDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::BannerItem;

/// 微信小店 BannerApplyDetail 数据类型；对应 Java com.binarywang.wxjava.store.bean.home.banner.BannerApplyDetail.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BannerApplyDetail {
    #[serde(rename = "audit_state", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_state: Option<i32>,
    #[serde(rename = "audit_desc", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_desc: Option<String>,
    #[serde(rename = "banner", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<BannerItem>,
}
