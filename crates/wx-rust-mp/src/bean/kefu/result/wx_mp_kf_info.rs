//! 对应 Java `bean.kefu.result.WxMpKfInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpKfInfo {
    #[serde(rename = "kf_account", default)]
    pub account: String,
    #[serde(rename = "kf_headimgurl", default)]
    pub head_img_url: String,
    #[serde(rename = "kf_id", default)]
    pub id: String,
    #[serde(rename = "kf_nick", default)]
    pub nick: String,
    #[serde(rename = "kf_wx", default)]
    pub wx_account: String,
    #[serde(rename = "invite_wx", default)]
    pub invite_wx: String,
    #[serde(rename = "invite_expire_time", default)]
    pub invite_expire_time: i64,
    #[serde(rename = "invite_status", default)]
    pub invite_status: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "accepted_case", default)]
    pub accepted_case: i32,
}
