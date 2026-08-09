//! 对应 Java `me.chanjar.weixin.channel.bean.window.request.GetWindowProductListRequest.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::window::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetWindowProductListRequest {
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "branch_id", default)]
    pub branch_id: i32,
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    #[serde(rename = "page_index", default)]
    pub page_index: i32,
    #[serde(rename = "last_buffer", default)]
    pub last_buffer: String,
    #[serde(rename = "need_total_num", default)]
    pub need_total_num: i32,
}
