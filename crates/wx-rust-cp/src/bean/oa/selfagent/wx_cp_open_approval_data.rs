//! 对应 Java `me.chanjar.weixin.cp.bean.oa.selfagent.WxCpOpenApprovalData.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOpenApprovalData {
    #[serde(rename = "ThirdNo", default)]
    pub third_no: String,
    #[serde(rename = "OpenTemplateId", default)]
    pub open_template_id: String,
    #[serde(rename = "OpenSpName", default)]
    pub open_sp_name: String,
    #[serde(rename = "OpenSpstatus", default)]
    pub open_spstatus: i32,
    #[serde(rename = "ApplyTime", default)]
    pub apply_time: i64,
    #[serde(rename = "ApplyUsername", default)]
    pub apply_user_name: String,
    #[serde(rename = "ApplyUserParty", default)]
    pub apply_user_party: String,
    #[serde(rename = "ApplyUserImage", default)]
    pub apply_user_image: String,
    #[serde(rename = "ApplyUserId", default)]
    pub apply_user_id: String,
    #[serde(rename = "ApprovalNodes", default)]
    pub approval_nodes: ApprovalNodes,
    #[serde(rename = "NotifyNodes", default)]
    pub notify_nodes: NotifyNodes,
    #[serde(rename = "ApproverStep", default)]
    pub approver_step: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApprovalNodes {
    #[serde(rename = "ApprovalNode", default)]
    pub approval_node: Vec<crate::bean::oa::selfagent::wx_cp_open_approval_data::ApprovalNode>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApprovalNode {
    #[serde(rename = "NodeStatus", default)]
    pub node_status: i32,
    #[serde(rename = "NodeAttr", default)]
    pub node_attr: i32,
    #[serde(rename = "NodeType", default)]
    pub node_type: i32,
    #[serde(rename = "Items", default)]
    pub items: crate::bean::oa::selfagent::wx_cp_open_approval_data::Items,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyNodes {
    #[serde(rename = "NotifyNode", default)]
    pub notify_node: Vec<crate::bean::oa::selfagent::wx_cp_open_approval_data::NotifyNode>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyNode {
    #[serde(rename = "ItemName", default)]
    pub item_name: String,
    #[serde(rename = "ItemParty", default)]
    pub item_party: String,
    #[serde(rename = "ItemImage", default)]
    pub item_image: String,
    #[serde(rename = "ItemUserId", default)]
    pub item_user_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Items {
    #[serde(rename = "Item", default)]
    pub item: Vec<crate::bean::oa::selfagent::wx_cp_open_approval_data::Item>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "ItemName", default)]
    pub item_name: String,
    #[serde(rename = "ItemParty", default)]
    pub item_party: String,
    #[serde(rename = "ItemImage", default)]
    pub item_image: String,
    #[serde(rename = "ItemUserId", default)]
    pub item_user_id: String,
    #[serde(rename = "ItemSpeech", default)]
    pub item_speech: String,
    #[serde(rename = "ItemStatus", default)]
    pub item_status: i32,
    #[serde(rename = "ItemOpTime", default)]
    pub item_op_time: i64,
}

impl WxCpOpenApprovalData {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpOpenApprovalData 解析失败: {e}"))
    }
}

impl WxCpOpenApprovalData {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpOpenApprovalData 序列化失败: {e}"))
    }
}
