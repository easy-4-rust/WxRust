//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayGetNegotiationHistoryResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayGetNegotiationHistoryResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "total", default)]
    pub total: i32,
    #[serde(rename = "history", default)]
    pub history: Vec<History>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct History {
    #[serde(rename = "log_id", default)]
    pub log_id: String,
    #[serde(rename = "operator", default)]
    pub operator: String,
    #[serde(rename = "operate_time", default)]
    pub operate_time: String,
    #[serde(rename = "operate_type", default)]
    pub operate_type: String,
    #[serde(rename = "operate_details", default)]
    pub operate_details: String,
    #[serde(rename = "complaint_media_list", default)]
    pub complaint_media_list: Vec<ComplaintMedia>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComplaintMedia {
    #[serde(rename = "media_type", default)]
    pub media_type: String,
    #[serde(rename = "media_url", default)]
    pub media_url: Vec<String>,
}

impl WxMaXPayGetNegotiationHistoryResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayGetNegotiationHistoryResponse 序列化失败: {e}"))
    }
}
