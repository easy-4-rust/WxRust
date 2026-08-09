//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpContactSearchResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpContactSearchResp {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "is_last", default)]
    pub is_last: bool,
    #[serde(rename = "query_result", default)]
    pub query_result: QueryResult,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QueryResult {
    #[serde(rename = "user", default)]
    pub user: User,
    #[serde(rename = "party", default)]
    pub party: Party,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct User {
    #[serde(rename = "userid", default)]
    pub userid: Vec<String>,
    #[serde(rename = "open_userid", default)]
    pub open_user_id: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Party {
    #[serde(rename = "department_id", default)]
    pub department_id: Vec<i32>,
}

impl WxCpTpContactSearchResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTpContactSearchResp 解析失败: {e}"))
    }
}
