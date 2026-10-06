//! 对应 Java `com.binarywang.wxjava.store.bean.product.ProductCategoryClassifyResponse.java`。

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<CategoryClassifyInfo>>,
    /// 是否命中错误类目。
    #[serde(rename = "wrong_cat", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrong_cat: Option<bool>,
}

/// 类目推荐信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryClassifyInfo {
    /// 类目层级列表。
    #[serde(rename = "cats", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cats: Option<Vec<CategoryLevel>>,
}

/// 类目层级。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryLevel {
    /// 类目信息。
    #[serde(rename = "cat_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_info: Option<CategoryLevelInfo>,
    /// 是否有权限。
    #[serde(rename = "has_permission", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_permission: Option<bool>,
}

/// 类目层级详细信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryLevelInfo {
    /// 类目 ID。
    #[serde(rename = "cat_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_id: Option<String>,
    /// 类目名称。
    #[serde(rename = "cat_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_name: Option<String>,
    /// 是否免审。
    #[serde(rename = "is_shop_no_audit", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shop_no_audit: Option<bool>,
}
