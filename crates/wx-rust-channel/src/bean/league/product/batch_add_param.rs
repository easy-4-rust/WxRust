//! 对应 Java `me.chanjar.weixin.channel.bean.league.product.BatchAddParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchAddParam {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "list", default)]
    pub list: Vec<Product>,
    #[serde(rename = "finder_ids", default)]
    pub finder_ids: Vec<String>,
    #[serde(rename = "begin_time", default)]
    pub begin_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "is_forerver", default)]
    pub forever: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Product {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "ratio", default)]
    pub ratio: i32,
}
