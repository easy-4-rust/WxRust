//! 对应 Java `me.chanjar.weixin.channel.bean.brand.BrandInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandInfo {
    #[serde(rename = "brand_id", default)]
    pub brand_id: String,
    #[serde(rename = "ch_name", default)]
    pub ch_name: String,
    #[serde(rename = "en_name", default)]
    pub en_name: String,
    #[serde(rename = "classification_no", default)]
    pub classification_no: String,
    #[serde(rename = "trade_mark_symbol", default)]
    pub trade_mark_symbol: i32,
    #[serde(rename = "register_details", default)]
    pub register_detail: BrandRegisterDetail,
    #[serde(rename = "application_details", default)]
    pub application_detail: BrandApplicationDetail,
    #[serde(rename = "grant_type", default)]
    pub grant_type: i32,
    #[serde(rename = "grant_details", default)]
    pub grant_detail: BrandGrantDetail,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "audit_result", default)]
    pub audit_result: AuditResult,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuditResult {
    #[serde(rename = "audit_id", default)]
    pub audit_id: String,
    #[serde(rename = "reject_reason", default)]
    pub reject_reason: String,
}
