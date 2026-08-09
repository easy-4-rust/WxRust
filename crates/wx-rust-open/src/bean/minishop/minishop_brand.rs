//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopBrand.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopBrand {
    #[serde(rename = "firstCatId", default)]
    pub first_cat_id: i32,
    #[serde(rename = "secondCatId", default)]
    pub second_cat_id: i32,
    #[serde(rename = "thirdCatId", default)]
    pub third_cat_id: i32,
    #[serde(rename = "brandInfo", default)]
    pub brand_info: MinishopBrandInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopBrandInfo {
    #[serde(rename = "brandId", default)]
    pub brand_id: i32,
    #[serde(rename = "brandName", default)]
    pub brand_name: String,
}
