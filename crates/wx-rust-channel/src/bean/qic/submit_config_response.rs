//! 对应 Java `me.chanjar.weixin.channel.bean.qic.SubmitConfigResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubmitConfigResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 送检配置模板信息
    #[serde(rename = "submit_config_info", default)]
    pub submit_config_info: SubmitConfigInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubmitConfigInfo {
    /// 模板 ID
    #[serde(rename = "template_id", default)]
    pub template_id: String,
}
