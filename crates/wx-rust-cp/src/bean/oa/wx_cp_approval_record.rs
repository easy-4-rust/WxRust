//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpApprovalRecord.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpApprovalRecord {
    #[serde(rename = "sp_status", default)]
    pub status: WxCpRecordSpStatus,
    #[serde(rename = "approverattr", default)]
    pub approver_attr: WxCpApproverAttr,
    #[serde(rename = "details", default)]
    pub details: Vec<crate::bean::oa::wx_cp_approval_record_detail::WxCpApprovalRecordDetail>,
}
