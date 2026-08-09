//! 对应 Java `cn.binarywang.wx.miniapp.bean.live.WxMaLiveAssistantInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaLiveAssistantInfo {
    #[serde(rename = "timestamp", default)]
    pub timestamp: i64,
    #[serde(rename = "headimg", default)]
    pub headimg: String,
    #[serde(rename = "username", default)]
    pub username: String,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "alias", default)]
    pub alias: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
}
