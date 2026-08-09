//! 对应 Java `cn.binarywang.wx.miniapp.bean.openapi.WxMiniGetApiQuotaResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMiniGetApiQuotaResult {
    #[serde(rename = "quota", default)]
    pub quota: WxMiniGetApiQuotaDetail,
    #[serde(rename = "rateLimit", default)]
    pub rate_limit: WxMiniGetApiQuotaRateLimit,
    #[serde(rename = "componentRateLimit", default)]
    pub component_rate_limit: WxMiniGetApiQuotaComponentRateLimit,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMiniGetApiQuotaDetail {
    #[serde(rename = "daily_limit", default)]
    pub daily_limit: i64,
    #[serde(rename = "used", default)]
    pub used: i64,
    #[serde(rename = "remain", default)]
    pub remain: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMiniGetApiQuotaRateLimit {
    #[serde(rename = "call_count", default)]
    pub call_count: i64,
    #[serde(rename = "refresh_second", default)]
    pub refresh_second: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMiniGetApiQuotaComponentRateLimit {
    #[serde(rename = "call_count", default)]
    pub call_count: i64,
    #[serde(rename = "refresh_second", default)]
    pub refresh_second: i64,
}
