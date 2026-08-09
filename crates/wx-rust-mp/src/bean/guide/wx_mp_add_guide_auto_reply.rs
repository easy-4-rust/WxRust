//! 对应 Java `bean.guide.WxMpAddGuideAutoReply`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpAddGuideAutoReply {
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "msgtype", default)]
    pub msg_type: i32,
}
