//! 对应 Java `bean.guide.WxMpGuideMsg`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideMsg {
    #[serde(rename = "guide_account", default)]
    pub account: String,
    #[serde(rename = "guide_openid", default)]
    pub openid: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "content_type", default)]
    pub content_type: i32,
    #[serde(rename = "direction", default)]
    pub direction: i32,
}
