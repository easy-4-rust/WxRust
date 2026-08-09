//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenMaWeappSupportVersionResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaWeappSupportVersionResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "now_version", default)]
    pub now_version: String,
    #[serde(rename = "uv_info", default)]
    pub uv_info: UvInfoBean,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UvInfoBean {
    #[serde(rename = "items", default)]
    pub items: Vec<VersionPercentageBean>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VersionPercentageBean {
    #[serde(rename = "percentage", default)]
    pub percentage: i32,
    #[serde(rename = "version", default)]
    pub version: String,
}
