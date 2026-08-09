//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopGetBrandResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopGetBrandResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "brands", default)]
    pub brands: Vec<MinishopBrandItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopBrandItem {
    #[serde(rename = "first_cat_id", default)]
    pub first_cat_id: i32,
    #[serde(rename = "second_cat_id", default)]
    pub second_cat_id: i32,
    #[serde(rename = "third_cat_id", default)]
    pub third_cat_id: i32,
    #[serde(rename = "brand_info", default)]
    pub brand_info: MinishopBrandInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopBrandInfo {
    #[serde(rename = "brand_id", default)]
    pub brand_id: i64,
    #[serde(rename = "brand_name", default)]
    pub brand_name: String,
}
