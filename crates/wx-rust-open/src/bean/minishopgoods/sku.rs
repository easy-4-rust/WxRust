//! 对应 Java `me.chanjar.weixin.open.bean.minishopgoods.Sku.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Sku {
    #[serde(rename = "outProductId", default)]
    pub out_product_id: String,
    #[serde(rename = "outSkuId", default)]
    pub out_sku_id: String,
    #[serde(rename = "thumbImg", default)]
    pub thumb_img: String,
    #[serde(rename = "salePrice", default)]
    pub sale_price: i32,
    #[serde(rename = "marketPrice", default)]
    pub market_price: i32,
    #[serde(rename = "stockNum", default)]
    pub stock_num: i32,
    #[serde(rename = "barcode", default)]
    pub barcode: String,
    #[serde(rename = "skuCode", default)]
    pub sku_code: String,
    #[serde(rename = "skuAttr", default)]
    pub sku_attr: Vec<Attr>,
}
