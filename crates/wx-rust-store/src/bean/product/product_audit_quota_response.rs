//! 对应 Java `com.binarywang.wxjava.store.bean.product.ProductAuditQuotaResponse.java`。

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_quota: Option<AuditQuota>,
}

/// 提审限额详情。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuditQuota {
    /// 封禁状态。
    #[serde(rename = "block_status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_status: Option<i32>,
    /// 可用配额。
    #[serde(rename = "avail_quota", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_quota: Option<i32>,
    /// 总配额。
    #[serde(rename = "total_quota", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_quota: Option<i32>,
    /// 无限类型。
    #[serde(rename = "unlimited_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlimited_type: Option<i32>,
    /// 审核总配额。
    #[serde(rename = "audit_total_quota", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_total_quota: Option<i32>,
    /// 审核总剩余。
    #[serde(rename = "audit_total_remaining", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_total_remaining: Option<i32>,
    /// 新商品总配额。
    #[serde(rename = "new_product_total_quota", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_product_total_quota: Option<i32>,
    /// 新商品剩余。
    #[serde(rename = "new_product_remaining", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_product_remaining: Option<i32>,
}
