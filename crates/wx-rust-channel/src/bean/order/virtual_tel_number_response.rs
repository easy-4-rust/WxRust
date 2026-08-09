//! 对应 Java `me.chanjar.weixin.channel.bean.order.VirtualTelNumberResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VirtualTelNumberResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "virtual_tel_number", default)]
    pub virtual_tel_number: String,
    #[serde(rename = "virtual_tel_expire_time", default)]
    pub virtual_tel_expire_time: i64,
    #[serde(rename = "get_virtual_tel_cnt", default)]
    pub get_virtual_tel_cnt: i32,
}
