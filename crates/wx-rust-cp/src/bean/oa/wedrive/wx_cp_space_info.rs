//! 对应 Java `me.chanjar.weixin.cp.bean.oa.wedrive.WxCpSpaceInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpSpaceInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "space_info", default)]
    pub space_info: SpaceInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpaceInfo {
    #[serde(rename = "spaceid", default)]
    pub space_id: String,
    #[serde(rename = "space_name", default)]
    pub space_name: String,
    #[serde(rename = "auth_list", default)]
    pub auth_list: crate::bean::oa::wedrive::wx_cp_space_info::AuthList,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthList {
    #[serde(rename = "auth_info", default)]
    pub auth_info: Vec<crate::bean::oa::wedrive::wx_cp_space_info::AuthInfo>,
    #[serde(rename = "quit_userid", default)]
    pub quit_user_id: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthInfo {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "departmentid", default)]
    pub department_id: i32,
    #[serde(rename = "auth", default)]
    pub auth: i32,
    #[serde(rename = "userid", default)]
    pub user_id: String,
}

impl WxCpSpaceInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpSpaceInfo 解析失败: {e}"))
    }
}

impl WxCpSpaceInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpSpaceInfo 序列化失败: {e}"))
    }
}
