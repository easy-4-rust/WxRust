//! 对应 Java `me.chanjar.weixin.cp.bean.oa.wedrive.WxCpSpaceAclDelRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpSpaceAclDelRequest {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "spaceid", default)]
    pub space_id: String,
    #[serde(rename = "auth_info", default)]
    pub auth_info: Vec<AuthInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthInfo {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "departmentid", default)]
    pub department_id: i32,
    #[serde(rename = "userid", default)]
    pub user_id: String,
}

impl WxCpSpaceAclDelRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpSpaceAclDelRequest 解析失败: {e}"))
    }
}

impl WxCpSpaceAclDelRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpSpaceAclDelRequest 序列化失败: {e}"))
    }
}
