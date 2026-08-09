//! 对应 Java `me.chanjar.weixin.cp.bean.school.user.WxCpBatchCreateParentRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::school::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpBatchCreateParentRequest {
    #[serde(rename = "parents", default)]
    pub parents: Vec<Parent>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Parent {
    #[serde(rename = "parent_userid", default)]
    pub parent_user_id: String,
    #[serde(rename = "mobile", default)]
    pub mobile: String,
    #[serde(rename = "to_invite", default)]
    pub to_invite: bool,
    #[serde(rename = "children", default)]
    pub children: Vec<crate::bean::school::user::wx_cp_batch_create_parent_request::Children>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Children {
    #[serde(rename = "student_userid", default)]
    pub student_user_id: String,
    #[serde(rename = "relation", default)]
    pub relation: String,
}

impl WxCpBatchCreateParentRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpBatchCreateParentRequest 解析失败: {e}"))
    }
}

impl WxCpBatchCreateParentRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpBatchCreateParentRequest 序列化失败: {e}"))
    }
}
