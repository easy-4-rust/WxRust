//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpApprovalDetailResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpApprovalDetailResult {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "info", default)]
    pub info: WxCpApprovalDetail,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpApprovalDetail {
    #[serde(rename = "sp_no", default)]
    pub sp_no: String,
    #[serde(rename = "sp_name", default)]
    pub sp_name: String,
    #[serde(rename = "sp_status", default)]
    pub sp_status: WxCpSpStatus,
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "apply_time", default)]
    pub apply_time: i64,
    #[serde(rename = "applyer", default)]
    pub applier: crate::bean::oa::wx_cp_approval_applier::WxCpApprovalApplier,
    #[serde(rename = "sp_record", default)]
    pub sp_records: Vec<crate::bean::oa::wx_cp_approval_record::WxCpApprovalRecord>,
    #[serde(rename = "notifyer", default)]
    pub notifiers: Vec<crate::bean::oa::wx_cp_operator::WxCpOperator>,
    #[serde(rename = "apply_data", default)]
    pub apply_data: crate::bean::oa::wx_cp_approval_apply_data::WxCpApprovalApplyData,
    #[serde(rename = "comments", default)]
    pub comments: Vec<crate::bean::oa::wx_cp_approval_comment::WxCpApprovalComment>,
    #[serde(rename = "sum_money", default)]
    pub sum_money: i64,
}
