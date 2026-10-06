//! 对应 Java `com.binarywang.wxjava.store.bean.product.ProductAuditStrategySetParam.java`。

use super::ProductAuditStrategyInfo;

/// 设置商品上架策略请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductAuditStrategySetParam {
    /// 上架策略信息。
    #[serde(rename = "audit_strategy", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_strategy: Option<ProductAuditStrategyInfo>,
}
