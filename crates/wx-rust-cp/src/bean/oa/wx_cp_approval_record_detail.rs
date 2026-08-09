//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpApprovalRecordDetail.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpApprovalRecordDetail {
    #[serde(rename = "approver", default)]
    pub approver: crate::bean::oa::wx_cp_operator::WxCpOperator,
    #[serde(rename = "speech", default)]
    pub speech: String,
    #[serde(rename = "sp_status", default)]
    pub sp_status: WxCpRecordSpStatus,
    #[serde(rename = "sptime", default)]
    pub sp_time: i64,
    #[serde(rename = "media_id", default)]
    pub media_ids: Vec<String>,
}
