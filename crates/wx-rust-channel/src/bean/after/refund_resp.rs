//! 对应 Java `me.chanjar.weixin.channel.bean.after.RefundResp.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefundResp {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "ret", default)]
    pub ret: i32,
    #[serde(rename = "message", default)]
    pub message: String,
}
