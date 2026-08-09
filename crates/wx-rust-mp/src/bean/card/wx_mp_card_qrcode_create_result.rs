//! 对应 Java `bean.card.WxMpCardQrcodeCreateResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpCardQrcodeCreateResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "ticket", default)]
    pub ticket: String,
    #[serde(rename = "expire_seconds", default)]
    pub expire_seconds: i32,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "show_qrcode_url", default)]
    pub show_qrcode_url: String,
}

impl WxMpCardQrcodeCreateResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpCardQrcodeCreateResult 解析失败: {e}"))
    }
}
