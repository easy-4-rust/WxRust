//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayGetUploadFileSignRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayGetUploadFileSignRequest {
    #[serde(rename = "env", default)]
    pub env: i32,
    #[serde(rename = "wxpay_url", default)]
    pub wxpay_url: String,
    #[serde(rename = "convert_cos", default)]
    pub convert_cos: bool,
    #[serde(rename = "complaint_id", default)]
    pub complaint_id: String,
}

impl WxMaXPayGetUploadFileSignRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayGetUploadFileSignRequest 序列化失败: {e}"))
    }
}
