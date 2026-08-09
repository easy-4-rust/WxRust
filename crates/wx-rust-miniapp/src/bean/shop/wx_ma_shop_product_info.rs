//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopProductInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopProductInfo {
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "out_sku_id", default)]
    pub out_sku_id: String,
    #[serde(rename = "product_cnt", default)]
    pub product_cnt: i32,
    #[serde(rename = "sale_price", default)]
    pub sale_price: i32,
    #[serde(rename = "head_img", default)]
    pub head_img: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "product_id", default)]
    pub product_id: i32,
    #[serde(rename = "sku_id", default)]
    pub sku_id: i32,
    #[serde(rename = "real_price", default)]
    pub real_price: i32,
    #[serde(rename = "sku_real_price", default)]
    pub sku_real_price: i32,
}
