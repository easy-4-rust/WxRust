//! 对应 Java `me.chanjar.weixin.channel.bean.lead.component.response.GetFinderLiveLeadsDataResponse.java`。
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
pub struct GetFinderLiveLeadsDataResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "item", default)]
    pub items: Vec<LeadCountItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LeadCountItem {
    #[serde(rename = "component_type", default)]
    pub component_type: i32,
    #[serde(rename = "traffic_type", default)]
    pub traffic_type: i32,
    #[serde(rename = "leads_count", default)]
    pub leads_count: i32,
}
