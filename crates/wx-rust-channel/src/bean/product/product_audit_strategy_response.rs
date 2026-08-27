//! 对应 Java `me.chanjar.weixin.channel.bean.product.ProductAuditStrategyResponse.java`。

#[allow(unused_imports)]
use super::*;

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
    pub audit_strategy: ProductAuditStrategyInfo,
}
