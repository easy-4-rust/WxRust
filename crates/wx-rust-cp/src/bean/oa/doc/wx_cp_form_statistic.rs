//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpFormStatistic.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpFormStatistic {
    #[serde(rename = "fill_cnt", default)]
    pub fill_cnt: i64,
    #[serde(rename = "repeated_id", default)]
    pub repeated_id: String,
    #[serde(rename = "repeated_name", default)]
    pub repeated_name: String,
    #[serde(rename = "fill_user_cnt", default)]
    pub fill_user_cnt: i64,
    #[serde(rename = "unfill_user_cnt", default)]
    pub unfill_user_cnt: i64,
    #[serde(rename = "submit_users", default)]
    pub submit_users: Vec<SubmitUser>,
    #[serde(rename = "unfill_users", default)]
    pub unfill_users: Vec<UnfillUser>,
    #[serde(rename = "has_more", default)]
    pub has_more: bool,
    #[serde(rename = "cursor", default)]
    pub cursor: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubmitUser {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "tmp_external_userid", default)]
    pub tmp_external_user_id: String,
    #[serde(rename = "submit_time", default)]
    pub submit_time: i64,
    #[serde(rename = "answer_id", default)]
    pub answer_id: i64,
    #[serde(rename = "user_name", default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnfillUser {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "user_name", default)]
    pub user_name: String,
}

impl WxCpFormStatistic {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpFormStatistic 解析失败: {e}"))
    }
}

impl WxCpFormStatistic {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpFormStatistic 序列化失败: {e}"))
    }
}
