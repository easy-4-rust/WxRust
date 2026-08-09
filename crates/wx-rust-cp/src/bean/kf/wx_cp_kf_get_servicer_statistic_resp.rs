//! 对应 Java `me.chanjar.weixin.cp.bean.kf.WxCpKfGetServicerStatisticResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfGetServicerStatisticResp {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "statistic_list", default)]
    pub statistic_list: Vec<crate::bean::kf::wx_cp_kf_get_servicer_statistic_resp::StatisticList>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatisticList {
    #[serde(rename = "stat_time", default)]
    pub stat_time: i64,
    #[serde(rename = "statistic", default)]
    pub statistic: crate::bean::kf::wx_cp_kf_get_servicer_statistic_resp::Statistic,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Statistic {
    #[serde(rename = "session_cnt", default)]
    pub session_cnt: i32,
    #[serde(rename = "customer_cnt", default)]
    pub customer_cnt: i32,
    #[serde(rename = "customer_msg_cnt", default)]
    pub customer_msg_cnt: i32,
    #[serde(rename = "reply_rate", default)]
    pub reply_rate: f32,
    #[serde(rename = "first_reply_average_sec", default)]
    pub first_reply_average_sec: f32,
    #[serde(rename = "satisfaction_investgate_cnt", default)]
    pub satisfaction_investgate_cnt: i32,
    #[serde(rename = "satisfaction_participation_rate", default)]
    pub satisfaction_participation_rate: f32,
    #[serde(rename = "satisfied_rate", default)]
    pub satisfied_rate: f32,
    #[serde(rename = "middling_rate", default)]
    pub middling_rate: f32,
    #[serde(rename = "dissatisfied_rate", default)]
    pub dissatisfied_rate: f32,
    #[serde(rename = "upgrade_service_customer_cnt", default)]
    pub upgrade_service_customer_cnt: i32,
    #[serde(rename = "upgrade_service_member_invite_cnt", default)]
    pub upgrade_service_member_invite_cnt: i32,
    #[serde(rename = "upgrade_service_member_customer_cnt", default)]
    pub upgrade_service_member_customer_cnt: i32,
    #[serde(rename = "upgrade_service_groupchat_invite_cnt", default)]
    pub upgrade_service_groupchat_invite_cnt: i32,
    #[serde(rename = "upgrade_service_groupchat_customer_cnt", default)]
    pub upgrade_service_groupchat_customer_cnt: i32,
    #[serde(rename = "msg_rejected_customer_cnt", default)]
    pub msg_rejected_customer_cnt: i32,
}

impl WxCpKfGetServicerStatisticResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpKfGetServicerStatisticResp 解析失败: {e}"))
    }
}
