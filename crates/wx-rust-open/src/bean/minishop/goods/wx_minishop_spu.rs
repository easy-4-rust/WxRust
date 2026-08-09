//! 对应 Java `me.chanjar.weixin.open.bean.minishop.goods.WxMinishopSpu.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::MinishopShopCat;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopSpu {
    #[serde(rename = "outProductId", default)]
    pub out_product_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "subTitle", default)]
    pub sub_title: String,
    #[serde(rename = "headImgs", default)]
    pub head_imgs: Vec<String>,
    #[serde(rename = "descInfoImgs", default)]
    pub desc_info_imgs: Vec<String>,
    #[serde(rename = "brandId", default)]
    pub brand_id: i64,
    #[serde(rename = "shopCats", default)]
    pub shop_cats: Vec<MinishopShopCat>,
    #[serde(rename = "attrs", default)]
    pub attrs: Vec<WxMinishopGoodsSkuAttr>,
    #[serde(rename = "model", default)]
    pub model: String,
    #[serde(rename = "expressTemplateId", default)]
    pub express_template_id: i64,
    #[serde(rename = "skus", default)]
    pub skus: Vec<WxMinishopSku>,
}
