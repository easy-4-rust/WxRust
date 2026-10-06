//! 对应 Java `com.binarywang.wxjava.store.bean.brand.BrandInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{BrandApplicationDetail, BrandGrantDetail, BrandRegisterDetail};

/// 微信小店 BrandInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.brand.BrandInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandInfo {
    #[serde(rename = "brand_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
    #[serde(rename = "ch_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch_name: Option<String>,
    #[serde(rename = "en_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(rename = "classification_no", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_no: Option<String>,
    #[serde(rename = "trade_mark_symbol", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_mark_symbol: Option<i32>,
    #[serde(rename = "register_details", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_detail: Option<BrandRegisterDetail>,
    #[serde(rename = "application_details", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_detail: Option<BrandApplicationDetail>,
    #[serde(rename = "grant_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<i32>,
    #[serde(rename = "grant_details", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_detail: Option<BrandGrantDetail>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "create_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "update_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(rename = "audit_result", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_result: Option<AuditResult>,
}

/// 微信小店 AuditResult 数据类型；对应 Java com.binarywang.wxjava.store.bean.brand.BrandInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuditResult {
    #[serde(rename = "audit_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_id: Option<String>,
    #[serde(rename = "reject_reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
}
