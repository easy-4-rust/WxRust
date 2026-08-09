//! 对应 Java `me.chanjar.weixin.channel.bean.lead.component.response.GetLeadsComponentPromoteRecordResponse.java`。
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
pub struct GetLeadsComponentPromoteRecordResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "record_data", default)]
    pub record_data: Vec<RecordData>,
    #[serde(rename = "last_buffer", default)]
    pub last_buffer: String,
    #[serde(rename = "continue_flag", default)]
    pub continue_flag: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecordData {
    #[serde(rename = "anchor_nickname", default)]
    pub anchor_nickname: String,
    #[serde(rename = "live_description", default)]
    pub live_description: String,
    #[serde(rename = "live_start_time", default)]
    pub live_start_time: i64,
    #[serde(rename = "live_audience_count", default)]
    pub live_audience_count: String,
    #[serde(rename = "exposure_uv", default)]
    pub exposure_uv: String,
    #[serde(rename = "click_uv", default)]
    pub click_uv: String,
    #[serde(rename = "exposure_click_rate", default)]
    pub exposure_click_rate: f64,
    #[serde(rename = "leads_num", default)]
    pub leads_num: String,
}
