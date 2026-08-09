//! 对应 Java `me.chanjar.weixin.open.bean.minishopgoods.AddMinishopGoodsSPU.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddMinishopGoodsSPU {
    #[serde(rename = "outProductId", default)]
    pub out_product_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "subTitle", default)]
    pub sub_title: String,
    #[serde(rename = "headImg", default)]
    pub head_img: Vec<String>,
    #[serde(rename = "descInfo", default)]
    pub desc_info: DescInfo,
    #[serde(rename = "brandId", default)]
    pub brand_id: i32,
    #[serde(rename = "cats", default)]
    pub cats: Vec<Cat>,
    #[serde(rename = "attrs", default)]
    pub attrs: Vec<Attr>,
    #[serde(rename = "model", default)]
    pub model: String,
    #[serde(rename = "expressInfo", default)]
    pub express_info: ExpressInfo,
    #[serde(rename = "skus", default)]
    pub skus: Vec<Sku>,
}
