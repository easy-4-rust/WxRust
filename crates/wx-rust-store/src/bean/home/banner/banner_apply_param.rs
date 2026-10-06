//! 对应 Java `com.binarywang.wxjava.store.bean.home.banner.BannerApplyParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::BannerInfo;

/// 微信小店 BannerApplyParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.home.banner.BannerApplyParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BannerApplyParam {
    #[serde(rename = "banner", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<BannerInfo>,
}
