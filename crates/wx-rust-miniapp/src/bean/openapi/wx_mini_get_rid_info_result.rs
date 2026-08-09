//! 对应 Java `cn.binarywang.wx.miniapp.bean.openapi.WxMiniGetRidInfoResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMiniGetRidInfoResult {
    #[serde(rename = "invoke_time", default)]
    pub invoke_time: i32,
    #[serde(rename = "cost_in_ms", default)]
    pub cost_in_ms: i32,
    #[serde(rename = "request_url", default)]
    pub request_url: String,
    #[serde(rename = "request_body", default)]
    pub request_body: String,
    #[serde(rename = "response_body", default)]
    pub response_body: String,
    #[serde(rename = "client_ip", default)]
    pub client_ip: String,
}
