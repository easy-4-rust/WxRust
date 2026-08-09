//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpGetApprovalData.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGetApprovalData {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "count", default)]
    pub count: i32,
    #[serde(rename = "total", default)]
    pub total: i32,
    #[serde(rename = "next_spnum", default)]
    pub next_sp_num: i64,
    #[serde(rename = "data", default)]
    pub data: Vec<ApprovalData>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApprovalData {
    #[serde(rename = "spname", default)]
    pub sp_name: String,
    #[serde(rename = "apply_name", default)]
    pub apply_name: String,
    #[serde(rename = "apply_org", default)]
    pub apply_org: String,
    #[serde(rename = "approval_name", default)]
    pub approval_name: Vec<String>,
    #[serde(rename = "notify_name", default)]
    pub notify_name: Vec<String>,
    #[serde(rename = "mediaids", default)]
    pub media_ids: Vec<String>,
    #[serde(rename = "sp_status", default)]
    pub sp_status: i32,
    #[serde(rename = "sp_num", default)]
    pub sp_num: i64,
    #[serde(rename = "apply_time", default)]
    pub apply_time: i64,
    #[serde(rename = "apply_user_id", default)]
    pub apply_user_id: String,
    #[serde(rename = "expense", default)]
    pub expense: crate::bean::oa::wx_cp_get_approval_data::Expense,
    #[serde(rename = "comm", default)]
    pub comm: crate::bean::oa::wx_cp_get_approval_data::Comm,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Expense {
    #[serde(rename = "expense_type", default)]
    pub expense_type: i32,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "item", default)]
    pub item: Vec<crate::bean::oa::wx_cp_get_approval_data::Item>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Comm {
    #[serde(rename = "apply_data", default)]
    pub apply_data: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "expenseitem_type", default)]
    pub expense_item_type: i32,
    #[serde(rename = "time", default)]
    pub time: i64,
    #[serde(rename = "sums", default)]
    pub sums: i32,
    #[serde(rename = "reason", default)]
    pub reason: String,
}

impl WxCpGetApprovalData {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpGetApprovalData 解析失败: {e}"))
    }
}

impl WxCpGetApprovalData {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpGetApprovalData 序列化失败: {e}"))
    }
}
