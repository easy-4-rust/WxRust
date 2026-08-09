//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopCatGetDetail.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopCatGetDetail {
    #[serde(rename = "third_cat_id", default)]
    pub third_cat_id: i64,
    #[serde(rename = "third_cat_name", default)]
    pub third_cat_name: String,
    #[serde(rename = "qualification", default)]
    pub qualification: String,
    #[serde(rename = "qualification_type", default)]
    pub qualification_type: i32,
    #[serde(rename = "product_qualification", default)]
    pub product_qualification: String,
    #[serde(rename = "product_qualification_type", default)]
    pub product_qualification_type: i32,
    #[serde(rename = "first_cat_id", default)]
    pub first_cat_id: i64,
    #[serde(rename = "first_cat_name", default)]
    pub first_cat_name: String,
    #[serde(rename = "second_cat_id", default)]
    pub second_cat_id: i64,
    #[serde(rename = "second_cat_name", default)]
    pub second_cat_name: String,
}
