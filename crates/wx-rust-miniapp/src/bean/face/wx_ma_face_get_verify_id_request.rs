//! 对应 Java `cn.binarywang.wx.miniapp.bean.face.WxMaFaceGetVerifyIdRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaFaceGetVerifyIdRequest {
    #[serde(rename = "out_seq_no", default)]
    pub out_seq_no: String,
    #[serde(rename = "cert_info", default)]
    pub cert_info: CertInfo,
    #[serde(rename = "openid", default)]
    pub openid: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CertInfo {
    #[serde(rename = "cert_type", default)]
    pub cert_type: String,
    #[serde(rename = "cert_name", default)]
    pub cert_name: String,
    #[serde(rename = "cert_no", default)]
    pub cert_no: String,
}

impl WxMaFaceGetVerifyIdRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaFaceGetVerifyIdRequest 序列化失败: {e}"))
    }
}
