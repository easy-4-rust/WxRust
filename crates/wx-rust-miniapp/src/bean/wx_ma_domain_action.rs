//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaDomainAction.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaDomainAction {
    #[serde(rename = "action", default)]
    pub action: String,
    #[serde(rename = "requestdomain", default)]
    pub request_domain: Vec<String>,
    #[serde(rename = "wsrequestdomain", default)]
    pub ws_request_domain: Vec<String>,
    #[serde(rename = "uploaddomain", default)]
    pub upload_domain: Vec<String>,
    #[serde(rename = "downloaddomain", default)]
    pub download_domain: Vec<String>,
    #[serde(rename = "webviewdomain", default)]
    pub web_view_domain: Vec<String>,
}

impl WxMaDomainAction {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaDomainAction 解析失败: {e}"))
    }
}

impl WxMaDomainAction {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaDomainAction 序列化失败: {e}"))
    }
}
