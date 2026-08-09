//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpInviteResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpInviteResult {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "invaliduser", default)]
    pub invalid_users: Vec<String>,
    #[serde(rename = "invalidparty", default)]
    pub invalid_parties: Vec<String>,
    #[serde(rename = "invalidtag", default)]
    pub invalid_tags: Vec<String>,
}

impl WxCpInviteResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpInviteResult 解析失败: {e}"))
    }
}
