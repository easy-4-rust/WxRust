//! 对应 Java `me.chanjar.weixin.open.bean.minishop.limitdiscount.LimitDiscountSku.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitDiscountSku {
    #[serde(rename = "skuId", default)]
    pub sku_id: i64,
    #[serde(rename = "salePrice", default)]
    pub sale_price: String,
    #[serde(rename = "saleStock", default)]
    pub sale_stock: i32,
}
