//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopDeliveryTemplate.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopDeliveryTemplate {
    #[serde(rename = "templateId", default)]
    pub template_id: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "valuationType", default)]
    pub valuation_type: ValuationType,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ValuationType {
    #[default]
    #[serde(rename = "PACKAGE")]
    Package,
    #[serde(rename = "WEIGHT")]
    Weight,
}
