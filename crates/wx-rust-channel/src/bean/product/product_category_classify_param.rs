//! 对应 Java `me.chanjar.weixin.channel.bean.product.ProductCategoryClassifyParam.java`。

#[allow(unused_imports)]
use super::*;

/// 商品类目推荐请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductCategoryClassifyParam {
    /// 请求类型。
    #[serde(rename = "req_type", default)]
    pub req_type: i32,
    /// 商品标题。
    #[serde(rename = "title", default)]
    pub title: String,
    /// 商品头图列表。
    #[serde(rename = "head_imgs", default)]
    pub head_imgs: Vec<String>,
    /// 类目 ID。
    #[serde(rename = "cat_id", default)]
    pub cat_id: String,
}
