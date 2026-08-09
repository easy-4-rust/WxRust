//! 对应 Java `me.chanjar.weixin.channel.bean.fund.qrcode.QrCheckResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::fund::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QrCheckResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "self_check_err_code", default)]
    pub self_check_err_code: i32,
    #[serde(rename = "self_check_err_msg", default)]
    pub self_check_err_msg: String,
    #[serde(rename = "scan_user_type", default)]
    pub scan_user_type: i32,
}
