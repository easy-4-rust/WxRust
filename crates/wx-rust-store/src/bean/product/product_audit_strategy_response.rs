//! 对应 Java `com.binarywang.wxjava.store.bean.product.ProductAuditStrategyResponse.java`。

use super::ProductAuditStrategyInfo;

/// 商品上架策略响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductAuditStrategyResponse {
    /// 错误码。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 上架策略信息。
    #[serde(rename = "audit_strategy", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_strategy: Option<ProductAuditStrategyInfo>,
}
