//! 查询服务商邀请开通电子发票能力的商户条件。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.InviteMerchantQuery`。

use serde::{Deserialize, Serialize};

/// 查询服务商邀请开通电子发票能力的商户条件。
///
/// 对应 Java: `InviteMerchantQuery`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteMerchantQuery {
    /// 查询开始时间。
    #[serde(rename = "query_time_start", skip_serializing_if = "Option::is_none")]
    pub query_time_start: Option<String>,

    /// 查询结束时间。
    #[serde(rename = "query_time_end", skip_serializing_if = "Option::is_none")]
    pub query_time_end: Option<String>,

    /// 偏移量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// 每页数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// 邀请码。
    #[serde(rename = "invite_code", skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,

    /// 商户邀请状态。
    #[serde(rename = "mch_invite_status", skip_serializing_if = "Option::is_none")]
    pub mch_invite_status: Option<String>,
}
