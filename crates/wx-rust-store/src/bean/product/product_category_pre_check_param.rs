/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.product.ProductCategoryPreCheckParam
/// 微信小店 ProductCategoryPreCheckParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.product.ProductCategoryPreCheckParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductCategoryPreCheckParam {
    #[serde(rename = "cat_id", skip_serializing_if = "Option::is_none")]
    pub cat_id: Option<i64>,
}
