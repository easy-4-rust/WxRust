//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayGetComplaintListRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayGetComplaintListRequest {
    #[serde(rename = "env", default)]
    pub env: i32,
    #[serde(rename = "begin_date", default)]
    pub begin_date: String,
    #[serde(rename = "end_date", default)]
    pub end_date: String,
    #[serde(rename = "offset", default)]
    pub offset: i32,
    #[serde(rename = "limit", default)]
    pub limit: i32,
}

impl WxMaXPayGetComplaintListRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayGetComplaintListRequest 序列化失败: {e}"))
    }
}
