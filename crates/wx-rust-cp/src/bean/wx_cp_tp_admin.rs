//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpAdmin.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpAdmin {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "admin", default)]
    pub admin: Vec<Admin>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Admin {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "open_userid", default)]
    pub open_user_id: String,
    #[serde(rename = "auth_type", default)]
    pub auth_type: i32,
}

impl WxCpTpAdmin {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTpAdmin 解析失败: {e}"))
    }
}

impl WxCpTpAdmin {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTpAdmin 序列化失败: {e}"))
    }
}
