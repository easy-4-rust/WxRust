//! 对应 Java `me.chanjar.weixin.channel.bean.product.ProductAuditStrategyInfo.java`。

#[allow(unused_imports)]
use super::*;

/// 商品上架策略信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductAuditStrategyInfo {
    /// 隐藏错误字段标记。
    #[serde(rename = "hide_err_field_flag", default)]
    pub hide_err_field_flag: i32,
    /// 命中重复标记。
    #[serde(rename = "hit_duplicated_flag", default)]
    pub hit_duplicated_flag: i32,
    /// 命中低风险规则标记。
    #[serde(rename = "hit_low_risk_rule_flag", default)]
    pub hit_low_risk_rule_flag: i32,
}
