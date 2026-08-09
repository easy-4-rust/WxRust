//! 对应 Java `cn.binarywang.wx.miniapp.bean.vod.WxMaVodCdnLogResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVodCdnLogResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "total_count", default)]
    pub total_count: i32,
    #[serde(rename = "domestic_cdn_logs", default)]
    pub domestic_cdn_logs: Vec<CdnLogInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CdnLogInfo {
    #[serde(rename = "date", default)]
    pub date: i64,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
}
