//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpUserExternalUserBehaviorStatistic.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUserExternalUserBehaviorStatistic {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "behavior_data", default)]
    pub behavior_list: Vec<Behavior>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Behavior {
    #[serde(rename = "stat_time", default)]
    pub stat_time: i64,
    #[serde(rename = "chat_cnt", default)]
    pub chat_cnt: i32,
    #[serde(rename = "message_cnt", default)]
    pub message_cnt: i32,
    #[serde(rename = "reply_percentage", default)]
    pub reply_percentage: f64,
    #[serde(rename = "avg_reply_time", default)]
    pub avg_reply_time: i32,
    #[serde(rename = "negative_feedback_cnt", default)]
    pub negative_feedback_cnt: i32,
    #[serde(rename = "new_apply_cnt", default)]
    pub new_apply_cnt: i32,
    #[serde(rename = "new_contact_cnt", default)]
    pub new_contact_cnt: i32,
}

impl WxCpUserExternalUserBehaviorStatistic {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpUserExternalUserBehaviorStatistic 解析失败: {e}"))
    }
}
