//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenMaDomainResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaDomainResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "requestdomain", default)]
    pub request_domain: Vec<String>,
    #[serde(rename = "wsrequestdomain", default)]
    pub ws_request_domain: Vec<String>,
    #[serde(rename = "uploaddomain", default)]
    pub upload_domain: Vec<String>,
    #[serde(rename = "downloaddomain", default)]
    pub download_domain: Vec<String>,
    #[serde(rename = "invalid_requestdomain", default)]
    pub invalid_request_domain: Vec<String>,
    #[serde(rename = "invalid_wsrequestdomain", default)]
    pub invalid_ws_request_domain: Vec<String>,
    #[serde(rename = "invalid_uploaddomain", default)]
    pub invalid_upload_domain: Vec<String>,
    #[serde(rename = "invalid_downloaddomain", default)]
    pub invalid_download_domain: Vec<String>,
}

impl WxOpenMaDomainResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxOpenMaDomainResult 解析失败: {e}"))
    }
}
