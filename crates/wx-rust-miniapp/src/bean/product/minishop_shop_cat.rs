//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.MinishopShopCat.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopShopCat {
    #[serde(rename = "cat_id", default)]
    pub shop_cat_id: i32,
    #[serde(rename = "shopCatName", default)]
    pub shop_cat_name: String,
    #[serde(rename = "fShopCatId", default)]
    pub f_shop_cat_id: i32,
    #[serde(rename = "level", default)]
    pub cat_level: i32,
}
