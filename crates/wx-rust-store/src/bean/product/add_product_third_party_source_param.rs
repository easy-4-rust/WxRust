//! 对应 Java `com.binarywang.wxjava.store.bean.product.AddProductThirdPartySourceParam.java`。

/// 新增第三方货源信息请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddProductThirdPartySourceParam {
    /// 场景值。
    #[serde(rename = "scene_value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_value: Option<i32>,
    /// 发布方式。
    #[serde(rename = "publish_method", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_method: Option<i32>,
    /// 供应商信息（JSON 对象）。
    #[serde(rename = "supplier", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier: Option<serde_json::Value>,
    /// 供应商店铺表现（JSON 对象）。
    #[serde(rename = "supplier_shop_performance", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_shop_performance: Option<serde_json::Value>,
    /// 商品来源信息（JSON 对象）。
    #[serde(rename = "product_source_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_source_info: Option<serde_json::Value>,
}
