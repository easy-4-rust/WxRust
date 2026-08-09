//! 对应 Java `me.chanjar.weixin.channel.bean.league.product.ProductDetailResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductDetailResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "item", default)]
    pub item: Item,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "ratio", default)]
    pub ratio: i32,
    #[serde(rename = "exclusive_info", default)]
    pub exclusive_info: ExclusiveInfo,
    #[serde(rename = "ext_info", default)]
    pub ext_info: ExtInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExclusiveInfo {
    #[serde(rename = "info_id", default)]
    pub info_id: String,
    #[serde(rename = "begin_time", default)]
    pub begin_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "is_forerver", default)]
    pub forever: bool,
    #[serde(rename = "finder_ids", default)]
    pub finder_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExtInfo {
    #[serde(rename = "is_sale_forbidden", default)]
    pub sale_forbidden: bool,
    #[serde(rename = "is_banned", default)]
    pub banned: bool,
}
