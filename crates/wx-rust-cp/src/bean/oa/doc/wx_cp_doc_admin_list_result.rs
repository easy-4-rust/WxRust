//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocAdminListResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocAdminListResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "docid", default)]
    pub doc_id: String,
    #[serde(rename = "admin_list", default)]
    pub admin_list: Vec<Admin>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Admin {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "open_userid", default)]
    pub open_user_id: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
}

impl WxCpDocAdminListResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDocAdminListResult 解析失败: {e}"))
    }
}

impl WxCpDocAdminListResult {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpDocAdminListResult 序列化失败: {e}"))
    }
}
