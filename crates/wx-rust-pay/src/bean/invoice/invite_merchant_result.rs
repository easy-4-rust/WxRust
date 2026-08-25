//! 服务商邀请商户查询结果。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.InviteMerchantResult`。

use serde::{Deserialize, Serialize};

/// 服务商邀请商户查询结果。
///
/// 对应 Java: `InviteMerchantResult`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteMerchantResult {
    /// 总数。
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,

    /// 偏移量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// 每页数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// 商户邀请结果列表。
    #[serde(
        rename = "mch_invite_result_list",
        skip_serializing_if = "Option::is_none"
    )]
    pub mch_invite_result_list: Option<Vec<Merchant>>,
}

/// 邀请商户信息。
///
/// 对应 Java: `InviteMerchantResult.Merchant`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Merchant {
    /// 子商户号。
    #[serde(rename = "sub_mchid", skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,

    /// 商户邀请状态。
    #[serde(rename = "mch_invite_status", skip_serializing_if = "Option::is_none")]
    pub mch_invite_status: Option<String>,

    /// 企业名称。
    #[serde(rename = "ep_name", skip_serializing_if = "Option::is_none")]
    pub ep_name: Option<String>,

    /// 税号。
    #[serde(rename = "tax_id", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,

    /// 邀请码。
    #[serde(rename = "invite_code", skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,

    /// 操作时间。
    #[serde(rename = "operate_time", skip_serializing_if = "Option::is_none")]
    pub operate_time: Option<String>,

    /// 邀请失败代码。
    #[serde(rename = "invite_failed_code", skip_serializing_if = "Option::is_none")]
    pub invite_failed_code: Option<String>,

    /// 邀请失败原因。
    #[serde(
        rename = "invite_failed_reason",
        skip_serializing_if = "Option::is_none"
    )]
    pub invite_failed_reason: Option<String>,
}
