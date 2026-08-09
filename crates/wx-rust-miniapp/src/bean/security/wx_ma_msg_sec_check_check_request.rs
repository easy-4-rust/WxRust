//! 对应 Java `cn.binarywang.wx.miniapp.bean.security.WxMaMsgSecCheckCheckRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaMsgSecCheckCheckRequest {
    #[serde(rename = "version", default)]
    pub version: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "scene", default)]
    pub scene: i32,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
}
