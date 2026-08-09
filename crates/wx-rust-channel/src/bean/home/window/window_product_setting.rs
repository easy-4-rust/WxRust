//! 对应 Java `me.chanjar.weixin.channel.bean.home.window.WindowProductSetting.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::home::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WindowProductSetting {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "is_set_hide", default)]
    pub set_hide: i32,
    #[serde(rename = "is_set_top", default)]
    pub set_top: i32,
}
