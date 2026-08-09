//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopSkuInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopSkuInfo {
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "out_sku_id", default)]
    pub out_sku_id: String,
    #[serde(rename = "thumb_img", default)]
    pub thumb_img: String,
    #[serde(rename = "sale_price", default)]
    pub sale_price: i32,
    #[serde(rename = "market_price", default)]
    pub market_price: i32,
    #[serde(rename = "stock_num", default)]
    pub stock_num: i32,
    #[serde(rename = "barcode", default)]
    pub barcode: String,
    #[serde(rename = "sku_code", default)]
    pub sku_code: String,
    #[serde(rename = "sku_attrs", default)]
    pub sku_attribute_list: Vec<WxMaShopSkuAttribute>,
}
