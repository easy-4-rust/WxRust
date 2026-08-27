//! 对应 Java `me.chanjar.weixin.channel.bean.order.RealNumberViewAuditResponse.java`。

#[allow(unused_imports)]
use super::*;

/// 查看订单真实号审核状态响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RealNumberViewAuditResponse {
    /// 错误码。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 审核状态：1-审核中，2-审核通过，3-审核拒绝。
    #[serde(rename = "audit_status", default)]
    pub audit_status: i32,
    /// 真实号码（审核通过后返回）。
    #[serde(rename = "real_number", default)]
    pub real_number: String,
}
