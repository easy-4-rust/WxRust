//! 对应 Java `bean.guide.WxMpGuideConfig`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideConfig {
    #[serde(rename = "guide_fast_reply_list", default)]
    pub guide_fast_reply_list: Vec<WxMpGuideFastReply>,
    #[serde(rename = "guide_auto_reply", default)]
    pub guide_auto_reply: WxMpGuideAutoReply,
    #[serde(rename = "guide_auto_reply_plus", default)]
    pub guide_auto_reply_plus: WxMpGuideAutoReply,
}
