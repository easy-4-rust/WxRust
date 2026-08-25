//! 对应 Java `me.chanjar.weixin.channel.bean.qic.InspectConfigResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InspectConfigResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 是否开通质检仓
    #[serde(rename = "is_opened", default)]
    pub is_opened: bool,
}
