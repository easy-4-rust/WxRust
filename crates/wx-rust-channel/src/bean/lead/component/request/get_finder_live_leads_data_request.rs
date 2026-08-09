//! 对应 Java `me.chanjar.weixin.channel.bean.lead.component.request.GetFinderLiveLeadsDataRequest.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::lead::component::*;
#[allow(unused_imports)]
use crate::bean::lead::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetFinderLiveLeadsDataRequest {
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "source_type", default)]
    pub source_type: i64,
}
