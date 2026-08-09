//! 对应 Java `me.chanjar.weixin.cp.bean.oa.wedrive.WxCpFileRename.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpFileRename {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "file", default)]
    pub file: File,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct File {
    #[serde(rename = "fileid", default)]
    pub file_id: String,
    #[serde(rename = "file_name", default)]
    pub file_name: String,
    #[serde(rename = "spaceid", default)]
    pub space_id: String,
    #[serde(rename = "fatherid", default)]
    pub father_id: String,
    #[serde(rename = "file_size", default)]
    pub file_size: i64,
    #[serde(rename = "ctime", default)]
    pub c_time: i64,
    #[serde(rename = "mtime", default)]
    pub m_time: i64,
    #[serde(rename = "file_type", default)]
    pub file_type: i32,
    #[serde(rename = "file_status", default)]
    pub file_status: i32,
    #[serde(rename = "create_userid", default)]
    pub create_user_id: String,
    #[serde(rename = "update_userid", default)]
    pub update_user_id: String,
    #[serde(rename = "sha", default)]
    pub sha: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "md5", default)]
    pub md5: String,
}

impl WxCpFileRename {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpFileRename 解析失败: {e}"))
    }
}

impl WxCpFileRename {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpFileRename 序列化失败: {e}"))
    }
}
