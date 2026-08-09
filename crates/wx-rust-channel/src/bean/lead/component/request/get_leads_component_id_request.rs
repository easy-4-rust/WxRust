//! 对应 Java `me.chanjar.weixin.channel.bean.lead.component.request.GetLeadsComponentIdRequest.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::lead::component::*;
#[allow(unused_imports)]
use crate::bean::lead::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetLeadsComponentIdRequest {
    #[serde(rename = "last_buffer", default)]
    pub last_buffer: String,
}
