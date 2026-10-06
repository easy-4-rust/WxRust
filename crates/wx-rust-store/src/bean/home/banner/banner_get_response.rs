//! 对应 Java `com.binarywang.wxjava.store.bean.home.banner.BannerGetResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{BannerApplyInfo, BannerInfo};

/// 微信小店 BannerGetResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.home.banner.BannerGetResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BannerGetResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "banner", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<BannerInfo>,
    #[serde(rename = "apply", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply: Option<BannerApplyInfo>,
}
