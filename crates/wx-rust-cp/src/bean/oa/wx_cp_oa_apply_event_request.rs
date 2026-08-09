//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpOaApplyEventRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOaApplyEventRequest {
    #[serde(rename = "creator_userid", default)]
    pub creator_user_id: String,
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "use_template_approver", default)]
    pub use_template_approver: i32,
    #[serde(rename = "choose_department", default)]
    pub choose_department: i32,
    #[serde(rename = "process", default)]
    pub process: Process,
    #[serde(rename = "approver", default)]
    pub approvers: Vec<Approver>,
    #[serde(rename = "notifyer", default)]
    pub notifiers: Vec<String>,
    #[serde(rename = "notify_type", default)]
    pub notify_type: i32,
    #[serde(rename = "apply_data", default)]
    pub apply_data: ApplyData,
    #[serde(rename = "summary_list", default)]
    pub summary_list: Vec<crate::bean::oa::summary_info::SummaryInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Approver {
    #[serde(rename = "attr", default)]
    pub attr: i32,
    #[serde(rename = "userid", default)]
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplyData {
    #[serde(rename = "contents", default)]
    pub contents: Vec<crate::bean::oa::applydata::apply_data_content::ApplyDataContent>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Process {
    #[serde(rename = "node_list", default)]
    pub node_list: Vec<crate::bean::oa::wx_cp_oa_apply_event_request::ProcessNode>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProcessNode {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "apv_rel", default)]
    pub apv_rel: i32,
    #[serde(rename = "userid", default)]
    pub user_ids: Vec<String>,
}

impl WxCpOaApplyEventRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpOaApplyEventRequest 序列化失败: {e}"))
    }
}
