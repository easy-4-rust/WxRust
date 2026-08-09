//! 对应 Java `me.chanjar.weixin.channel.bean.window.request.AddWindowProductRequest.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::window::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddWindowProductRequest {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "is_hide_for_window", default)]
    pub is_hide_for_window: bool,
}
