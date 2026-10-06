//! 对应 Java `com.binarywang.wxjava.store.bean.product.ProductCategoryClassifyParam.java`。

/// 商品类目推荐请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductCategoryClassifyParam {
    /// 请求类型。
    #[serde(rename = "req_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_type: Option<i32>,
    /// 商品标题。
    #[serde(rename = "title", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 商品头图列表。
    #[serde(rename = "head_imgs", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_imgs: Option<Vec<String>>,
    /// 类目 ID。
    #[serde(rename = "cat_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_id: Option<String>,
}
