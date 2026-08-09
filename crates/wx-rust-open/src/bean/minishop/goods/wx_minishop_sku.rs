//! 对应 Java `me.chanjar.weixin.open.bean.minishop.goods.WxMinishopSku.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopSku {
    #[serde(rename = "productId", default)]
    pub product_id: i64,
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
    #[serde(rename = "skuCode", default)]
    pub sku_code: String,
    #[serde(rename = "barCode", default)]
    pub bar_code: String,
    #[serde(rename = "skuAttrs", default)]
    pub sku_attrs: Vec<WxMinishopGoodsSkuAttr>,
}
