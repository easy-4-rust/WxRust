//! 对应 Java `cn.binarywang.wx.miniapp.bean.urllink.CloudBase.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CloudBase {
    #[serde(rename = "env", default)]
    pub env: String,
    #[serde(rename = "domain", default)]
    pub domain: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "query", default)]
    pub query: String,
    #[serde(rename = "resource_appid", default)]
    pub resource_appid: String,
}
