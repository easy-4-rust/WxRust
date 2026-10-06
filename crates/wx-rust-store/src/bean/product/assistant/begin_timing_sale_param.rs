//! 对应 Java `com.binarywang.wxjava.store.bean.product.ProductTimingSaleParam.java`。

/// 商品立即开售/定时开售请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BeginTimingSaleParam {
    /// 商品 ID。
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// 定时任务 ID。
    #[serde(rename = "task_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}
