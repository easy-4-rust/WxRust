//! 对应 Java `me.chanjar.weixin.channel.bean.product.ProductCategoryClassifyResponse.java`。

#[allow(unused_imports)]
use super::*;

/// 商品类目推荐响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductCategoryClassifyResponse {
    /// 错误码。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 推荐类目列表。
    #[serde(rename = "categories", default)]
    pub categories: Vec<CategoryClassifyInfo>,
    /// 是否命中错误类目。
    #[serde(rename = "wrong_cat", default)]
    pub wrong_cat: bool,
}

/// 类目推荐信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryClassifyInfo {
    /// 类目层级列表。
    #[serde(rename = "cats", default)]
    pub cats: Vec<CategoryLevel>,
}

/// 类目层级。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryLevel {
    /// 类目信息。
    #[serde(rename = "cat_info", default)]
    pub cat_info: CategoryLevelInfo,
    /// 是否有权限。
    #[serde(rename = "has_permission", default)]
    pub has_permission: bool,
}

/// 类目层级详细信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryLevelInfo {
    /// 类目 ID。
    #[serde(rename = "cat_id", default)]
    pub cat_id: String,
    /// 类目名称。
    #[serde(rename = "cat_name", default)]
    pub cat_name: String,
    /// 是否免审。
    #[serde(rename = "is_shop_no_audit", default)]
    pub is_shop_no_audit: bool,
}
