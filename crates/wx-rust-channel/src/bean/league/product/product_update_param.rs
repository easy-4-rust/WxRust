//! 对应 Java `me.chanjar.weixin.channel.bean.league.product.ProductUpdateParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductUpdateParam {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "info_id", default)]
    pub info_id: String,
    #[serde(rename = "operate_type", default)]
    pub operate_type: i32,
    #[serde(rename = "ratio", default)]
    pub ratio: i32,
    #[serde(rename = "exclusive_info", default)]
    pub exclusive_info: ExclusiveInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExclusiveInfo {
    #[serde(rename = "begin_time", default)]
    pub begin_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "is_forerver", default)]
    pub forever: bool,
    #[serde(rename = "add_finder_ids", default)]
    pub add_finder_ids: Vec<String>,
    #[serde(rename = "del_finder_ids", default)]
    pub del_finder_ids: Vec<String>,
}
