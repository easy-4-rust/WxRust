//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopProductInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopProductInfo {
    #[serde(rename = "product_id", default)]
    pub product_id: i32,
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "sku_id", default)]
    pub sku_id: i32,
    #[serde(rename = "out_sku_id", default)]
    pub out_sku_id: String,
    #[serde(rename = "sku_cnt", default)]
    pub sku_cnt: i32,
    #[serde(rename = "on_aftersale_sku_cnt", default)]
    pub on_aftersale_sku_cnt: i32,
    #[serde(rename = "finish_aftersale_sku_cnt", default)]
    pub finish_aftersale_sku_cnt: i32,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "thumb_img", default)]
    pub thumb_img: String,
    #[serde(rename = "sku_attrs", default)]
    pub sku_attrs: Vec<WxMinishopGoodsSkuAttr>,
    #[serde(rename = "sale_price", default)]
    pub sale_price: i32,
    #[serde(rename = "market_price", default)]
    pub market_price: i32,
}
