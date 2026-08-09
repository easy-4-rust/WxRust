//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenVersioninfoResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenVersioninfoResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "exp_info", default)]
    pub exp_info: ExpInfo,
    #[serde(rename = "release_info", default)]
    pub release_info: ReleaseInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReleaseInfo {
    #[serde(rename = "release_time", default)]
    pub release_time: i64,
    #[serde(rename = "release_version", default)]
    pub release_version: String,
    #[serde(rename = "release_desc", default)]
    pub release_desc: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExpInfo {
    #[serde(rename = "exp_time", default)]
    pub exp_time: i64,
    #[serde(rename = "exp_version", default)]
    pub exp_version: String,
    #[serde(rename = "exp_desc", default)]
    pub exp_desc: String,
}
