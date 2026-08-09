//! 对应 Java `me.chanjar.weixin.cp.bean.school.WxCpSchoolLivingInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpSchoolLivingInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "living_info", default)]
    pub living_info: LivingInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LivingInfo {
    #[serde(rename = "theme", default)]
    pub theme: String,
    #[serde(rename = "living_start", default)]
    pub living_start: i64,
    #[serde(rename = "living_duration", default)]
    pub living_duration: i64,
    #[serde(rename = "anchor_userid", default)]
    pub anchor_user_id: String,
    #[serde(rename = "living_range", default)]
    pub living_range: crate::bean::school::wx_cp_school_living_info::LivingRange,
    #[serde(rename = "viewer_num", default)]
    pub viewer_num: i32,
    #[serde(rename = "comment_num", default)]
    pub comment_num: i32,
    #[serde(rename = "open_replay", default)]
    pub open_replay: i32,
    #[serde(rename = "push_stream_url", default)]
    pub push_stream_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LivingRange {
    #[serde(rename = "partyids", default)]
    pub party_ids: Vec<i32>,
    #[serde(rename = "group_names", default)]
    pub group_names: Vec<String>,
}

impl WxCpSchoolLivingInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpSchoolLivingInfo 解析失败: {e}"))
    }
}

impl WxCpSchoolLivingInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpSchoolLivingInfo 序列化失败: {e}"))
    }
}
