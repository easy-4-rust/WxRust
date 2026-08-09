//! 对应 Java `cn.binarywang.wx.miniapp.bean.face.WxMaFaceGetVerifyIdResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaFaceGetVerifyIdResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "verify_id", default)]
    pub verify_id: String,
    #[serde(rename = "expires_in", default)]
    pub expires_in: i32,
}

impl WxMaFaceGetVerifyIdResponse {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaFaceGetVerifyIdResponse 解析失败: {e}"))
    }
}
