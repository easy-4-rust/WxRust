//! 对应 Java `me.chanjar.weixin.channel.bean.compass.shop.ShopProductInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopProductInfo {
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
    pub data: ShopProductCompassData,
}
