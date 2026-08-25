//! 对应 Java `me.chanjar.weixin.channel.bean.qic.InspectCodeResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InspectCodeResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 质检码 URL
    #[serde(rename = "inspect_code_url", default)]
    pub inspect_code_url: String,
}
