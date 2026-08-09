//! 对应 Java `me.chanjar.weixin.cp.bean.kf.msg.WxCpKfChannelsMsg.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::kf::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfChannelsMsg {
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "desc", default)]
    pub desc: String,
    #[serde(rename = "cover_url", default)]
    pub cover_url: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "find_username", default)]
    pub find_username: String,
}
