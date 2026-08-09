//! 对应 Java `me.chanjar.weixin.cp.bean.oa.wedrive.WxCpFileList.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpFileList {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "has_more", default)]
    pub has_more: bool,
    #[serde(rename = "next_start", default)]
    pub next_start: i32,
    #[serde(rename = "file_list", default)]
    pub file_list: FileList,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FileList {
    #[serde(rename = "item", default)]
    pub item: Vec<crate::bean::oa::wedrive::wx_cp_file_list::Item>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
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

impl WxCpFileList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpFileList 解析失败: {e}"))
    }
}

impl WxCpFileList {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpFileList 序列化失败: {e}"))
    }
}
