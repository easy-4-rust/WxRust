//! 对应 Java `me.chanjar.weixin.channel.bean.product.ProductAuditQuotaResponse.java`。

#[allow(unused_imports)]
use super::*;

/// 商品提审限额响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductAuditQuotaResponse {
    /// 错误码。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 提审限额信息。
    #[serde(rename = "audit_quota", default)]
    pub audit_quota: AuditQuota,
}

/// 提审限额详情。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuditQuota {
    /// 封禁状态。
    #[serde(rename = "block_status", default)]
    pub block_status: i32,
    /// 可用配额。
    #[serde(rename = "avail_quota", default)]
    pub avail_quota: i32,
    /// 总配额。
    #[serde(rename = "total_quota", default)]
    pub total_quota: i32,
    /// 无限类型。
    #[serde(rename = "unlimited_type", default)]
    pub unlimited_type: i32,
    /// 审核总配额。
    #[serde(rename = "audit_total_quota", default)]
    pub audit_total_quota: i32,
    /// 审核总剩余。
    #[serde(rename = "audit_total_remaining", default)]
    pub audit_total_remaining: i32,
    /// 新商品总配额。
    #[serde(rename = "new_product_total_quota", default)]
    pub new_product_total_quota: i32,
    /// 新商品剩余。
    #[serde(rename = "new_product_remaining", default)]
    pub new_product_remaining: i32,
}
