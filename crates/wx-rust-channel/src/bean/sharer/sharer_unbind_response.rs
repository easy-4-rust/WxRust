//! 对应 Java `me.chanjar.weixin.channel.bean.sharer.SharerUnbindResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SharerUnbindResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "success_openid", default)]
    pub success_list: Vec<String>,
    #[serde(rename = "fail_openid", default)]
    pub fail_list: Vec<String>,
    #[serde(rename = "refuse_openid", default)]
    pub refuse_list: Vec<String>,
}
