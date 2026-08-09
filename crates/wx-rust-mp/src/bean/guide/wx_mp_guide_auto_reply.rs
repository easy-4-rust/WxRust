//! 对应 Java `bean.guide.WxMpGuideAutoReply`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideAutoReply {
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "msgtype", default)]
    pub msg_type: i32,
    #[serde(rename = "updatetime", default)]
    pub update_time: i64,
}
