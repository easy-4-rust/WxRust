//! 对应 Java `com.binarywang.wxjava.store.bean.product.ProductAuditStrategyInfo.java`。

/// 商品上架策略信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductAuditStrategyInfo {
    /// 隐藏错误字段标记。
    #[serde(rename = "hide_err_field_flag", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_err_field_flag: Option<i32>,
    /// 命中重复标记。
    #[serde(rename = "hit_duplicated_flag", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_duplicated_flag: Option<i32>,
    /// 命中低风险规则标记。
    #[serde(rename = "hit_low_risk_rule_flag", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_low_risk_rule_flag: Option<i32>,
}
