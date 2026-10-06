//! 对应 Java `com.binarywang.wxjava.store.bean.brand.BrandApplyListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::BasicBrand;

/// 微信小店 BrandApplyListResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.brand.BrandApplyListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandApplyListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "brands", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brands: Option<Vec<BasicBrand>>,
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    #[serde(rename = "total_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num: Option<i32>,
}
