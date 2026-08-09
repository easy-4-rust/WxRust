//! 对应 Java `me.chanjar.weixin.channel.bean.league.supplier.ProductInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[allow(unused_imports)]
use crate::bean::league::ExpressInfo;
#[allow(unused_imports)]
use crate::bean::league::SimpleProductInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductInfo {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "sub_title", default)]
    pub sub_title: String,
    #[serde(rename = "head_imgs", default)]
    pub head_imgs: Vec<String>,
    #[serde(rename = "desc_info", default)]
    pub desc_info: DescInfo,
    #[serde(rename = "cats", default)]
    pub cats: Vec<CatInfo>,
    #[serde(rename = "express_info", default)]
    pub express_info: ExpressInfo,
    #[serde(rename = "skus", default)]
    pub skus: Vec<SkuInfo>,
}
