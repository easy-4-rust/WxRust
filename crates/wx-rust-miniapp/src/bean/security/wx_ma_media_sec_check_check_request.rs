//! 对应 Java `cn.binarywang.wx.miniapp.bean.security.WxMaMediaSecCheckCheckRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaMediaSecCheckCheckRequest {
    #[serde(rename = "media_url", default)]
    pub media_url: String,
    #[serde(rename = "media_type", default)]
    pub media_type: i32,
    #[serde(rename = "version", default)]
    pub version: i32,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "scene", default)]
    pub scene: i32,
}
