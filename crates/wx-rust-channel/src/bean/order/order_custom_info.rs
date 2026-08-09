//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderCustomInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderCustomInfo {
    #[serde(rename = "custom_img_url", default)]
    pub custom_img_url: String,
    #[serde(rename = "custom_word", default)]
    pub custom_word: String,
    #[serde(rename = "custom_type", default)]
    pub custom_type: i32,
    #[serde(rename = "custom_preview_img_url", default)]
    pub custom_preview_img_url: String,
}
