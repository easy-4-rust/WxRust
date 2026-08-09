//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopSpu.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopSpu {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "sub_title", default)]
    pub sub_title: String,
    #[serde(rename = "head_img", default)]
    pub head_imgs: Vec<String>,
    #[serde(rename = "desc_info", default)]
    pub desc_info: DescInfo,
    #[serde(rename = "brand_id", default)]
    pub brand_id: i64,
    #[serde(rename = "cats", default)]
    pub shop_cats: Vec<MinishopShopCat>,
    #[serde(rename = "attrs", default)]
    pub attrs: Vec<WxMinishopGoodsSkuAttr>,
    #[serde(rename = "model", default)]
    pub model: String,
    #[serde(rename = "express_info", default)]
    pub express_info: ExpressInfo,
    #[serde(rename = "skus", default)]
    pub skus: Vec<WxMinishopSku>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DescInfo {
    #[serde(rename = "imgs", default)]
    pub imgs: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExpressInfo {
    #[serde(rename = "template_id", default)]
    pub template_id: i64,
}
