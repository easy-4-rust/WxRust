//! 对应 Java `cn.binarywang.wx.miniapp.bean.urllink.response.QueryUrlLinkResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::urllink::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QueryUrlLinkResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "visit_openid", default)]
    pub visit_openid: String,
    #[serde(rename = "url_link_info", default)]
    pub url_link_info: UrlLinkInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UrlLinkInfo {
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "query", default)]
    pub query: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "expire_time", default)]
    pub expire_time: i64,
    #[serde(rename = "env_version", default)]
    pub env_version: String,
}
