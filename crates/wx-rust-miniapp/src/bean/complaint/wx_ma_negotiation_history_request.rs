//! 对应 Java `cn.binarywang.wx.miniapp.bean.complaint.WxMaNegotiationHistoryRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaNegotiationHistoryRequest {
    #[serde(rename = "complaint_id", default)]
    pub complaint_id: String,
    #[serde(rename = "limit", default)]
    pub limit: i32,
    #[serde(rename = "offset", default)]
    pub offset: i32,
}

impl WxMaNegotiationHistoryRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaNegotiationHistoryRequest 序列化失败: {e}"))
    }
}
