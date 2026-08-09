//! 对应 Java `me.chanjar.weixin.cp.bean.school.WxCpSchoolUnwatchStat.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpSchoolUnwatchStat {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "ending", default)]
    pub ending: i32,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
    #[serde(rename = "stat_info", default)]
    pub stat_info: StatInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatInfo {
    #[serde(rename = "students", default)]
    pub students: Vec<crate::bean::school::wx_cp_school_unwatch_stat::Student>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Student {
    #[serde(rename = "student_userid", default)]
    pub student_user_id: String,
    #[serde(rename = "parent_userid", default)]
    pub parent_user_id: String,
    #[serde(rename = "partyids", default)]
    pub party_ids: Vec<i32>,
}

impl WxCpSchoolUnwatchStat {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpSchoolUnwatchStat 解析失败: {e}"))
    }
}

impl WxCpSchoolUnwatchStat {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpSchoolUnwatchStat 序列化失败: {e}"))
    }
}
