//! 对应 Java `me.chanjar.weixin.channel.bean.lead.component.response.GetFinderLiveDataListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::lead::component::*;
#[allow(unused_imports)]
use crate::bean::lead::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetFinderLiveDataListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "item", default)]
    pub items: Vec<LiveStatisticsItem>,
    #[serde(rename = "last_buffer", default)]
    pub last_buffer: String,
    #[serde(rename = "continue_flag", default)]
    pub continue_flag: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveStatisticsItem {
    #[serde(rename = "export_id", default)]
    pub export_id: String,
    #[serde(rename = "live_start_time", default)]
    pub live_start_time: i64,
    #[serde(rename = "live_duration_in_seconds", default)]
    pub live_duration_in_seconds: i64,
    #[serde(rename = "total_audience_count", default)]
    pub total_audience_count: i64,
    #[serde(rename = "total_cheer_count", default)]
    pub total_cheer_count: i64,
    #[serde(rename = "forward_count", default)]
    pub forward_count: i64,
    #[serde(rename = "total_comment_count", default)]
    pub total_comment_count: i64,
    #[serde(rename = "audiences_avg_seconds", default)]
    pub audiences_avg_seconds: i64,
    #[serde(rename = "max_online_count", default)]
    pub max_online_count: i64,
    #[serde(rename = "new_follow_count", default)]
    pub new_follow_count: i64,
    #[serde(rename = "new_follow_count_biz", default)]
    pub new_follow_count_biz: i64,
}
