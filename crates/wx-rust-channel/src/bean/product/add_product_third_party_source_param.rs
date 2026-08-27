//! 对应 Java `me.chanjar.weixin.channel.bean.product.AddProductThirdPartySourceParam.java`。

#[allow(unused_imports)]
use super::*;

/// 新增第三方货源信息请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddProductThirdPartySourceParam {
    /// 场景值。
    #[serde(rename = "scene_value", default)]
    pub scene_value: i32,
    /// 发布方式。
    #[serde(rename = "publish_method", default)]
    pub publish_method: i32,
    /// 供应商信息（JSON 对象）。
    #[serde(rename = "supplier", default)]
    pub supplier: serde_json::Value,
    /// 供应商店铺表现（JSON 对象）。
    #[serde(rename = "supplier_shop_performance", default)]
    pub supplier_shop_performance: serde_json::Value,
    /// 商品来源信息（JSON 对象）。
    #[serde(rename = "product_source_info", default)]
    pub product_source_info: serde_json::Value,
}
