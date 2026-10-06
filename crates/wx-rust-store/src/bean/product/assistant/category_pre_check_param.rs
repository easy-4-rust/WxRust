//! 对应 Java `com.binarywang.wxjava.store.bean.product.assistant.CategoryPreCheckParam.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 CategoryPreCheckParam；对应 Java com.binarywang.wxjava.store.bean.product.assistant.CategoryPreCheckParam.java。
pub struct CategoryPreCheckParam {
    /// 类目 ID
    #[serde(rename = "category_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 上游字段 cat_id。
    #[serde(rename = "cat_id", skip_serializing_if = "Option::is_none")]
    pub cat_id: Option<i64>,
}
