//! 对应 Java `me.chanjar.weixin.channel.bean.product.ProductAuditStrategySetParam.java`。

#[allow(unused_imports)]
use super::*;

/// 设置商品上架策略请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductAuditStrategySetParam {
    /// 上架策略信息。
    #[serde(rename = "audit_strategy", default)]
    pub audit_strategy: ProductAuditStrategyInfo,
}
