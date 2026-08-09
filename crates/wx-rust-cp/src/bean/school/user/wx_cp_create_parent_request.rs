//! 对应 Java `me.chanjar.weixin.cp.bean.school.user.WxCpCreateParentRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::school::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCreateParentRequest {
    #[serde(rename = "parent_userid", default)]
    pub parent_user_id: String,
    #[serde(rename = "mobile", default)]
    pub mobile: String,
    #[serde(rename = "to_invite", default)]
    pub to_invite: bool,
    #[serde(rename = "children", default)]
    pub children: Vec<Children>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Children {
    #[serde(rename = "student_userid", default)]
    pub student_user_id: String,
    #[serde(rename = "relation", default)]
    pub relation: String,
}

impl WxCpCreateParentRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpCreateParentRequest 解析失败: {e}"))
    }
}

impl WxCpCreateParentRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpCreateParentRequest 序列化失败: {e}"))
    }
}
