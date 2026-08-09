//! 对应 Java `cn.binarywang.wx.miniapp.bean.urllink.GenerateUrlLinkRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenerateUrlLinkRequest {
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "query", default)]
    pub query: String,
    #[serde(rename = "env_version", default)]
    pub env_version: String,
    #[serde(rename = "is_expire", default)]
    pub is_expire: bool,
    #[serde(rename = "expire_type", default)]
    pub expire_type: i32,
    #[serde(rename = "expire_time", default)]
    pub expire_time: i64,
    #[serde(rename = "expire_interval", default)]
    pub expire_interval: i32,
    #[serde(rename = "cloud_base", default)]
    pub cloud_base: CloudBase,
}
