//! 对应 Java `bean.guide.WxMpGuideGroupInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideGroupInfo {
    #[serde(rename = "guide_account", default)]
    pub account: String,
    #[serde(rename = "guide_openid", default)]
    pub openid: String,
    #[serde(rename = "guide_nickname", default)]
    pub nick_name: String,
    #[serde(rename = "guide_headimgurl", default)]
    pub head_img_url: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
}
