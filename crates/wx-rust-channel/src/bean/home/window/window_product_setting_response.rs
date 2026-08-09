//! 对应 Java `me.chanjar.weixin.channel.bean.home.window.WindowProductSettingResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::home::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WindowProductSettingResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "products", default)]
    pub products: Vec<WindowProductSetting>,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
    #[serde(rename = "total_num", default)]
    pub total_num: i32,
}
