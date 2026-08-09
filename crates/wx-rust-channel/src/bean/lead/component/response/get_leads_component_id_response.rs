//! 对应 Java `me.chanjar.weixin.channel.bean.lead.component.response.GetLeadsComponentIdResponse.java`。
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
pub struct GetLeadsComponentIdResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "item", default)]
    pub item: Vec<LeadComponentItem>,
    #[serde(rename = "last_buffer", default)]
    pub last_buffer: String,
    #[serde(rename = "continue_flag", default)]
    pub continue_flag: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LeadComponentItem {
    #[serde(rename = "leads_component_id", default)]
    pub leads_component_id: String,
    #[serde(rename = "leads_description", default)]
    pub leads_description: String,
    #[serde(rename = "status", default)]
    pub status: i32,
}
