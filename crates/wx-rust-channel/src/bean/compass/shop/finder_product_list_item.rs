//! 对应 Java `me.chanjar.weixin.channel.bean.compass.shop.FinderProductListItem.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinderProductListItem {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "head_img_url", default)]
    pub head_img_url: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "price", default)]
    pub price: String,
    #[serde(rename = "first_category_id", default)]
    pub first_category_id: String,
    #[serde(rename = "second_category_id", default)]
    pub second_category_id: String,
    #[serde(rename = "third_category_id", default)]
    pub third_category_id: String,
    #[serde(rename = "data", default)]
    pub data: GmvData,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GmvData {
    #[serde(rename = "commission_ratio", default)]
    pub commission_ratio: f64,
    #[serde(rename = "pay_gmv", default)]
    pub pay_gmv: String,
}
