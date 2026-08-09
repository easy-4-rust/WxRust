//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaPhoneNumberInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPhoneNumberInfo {
    #[serde(rename = "phoneNumber", default)]
    pub phone_number: String,
    #[serde(rename = "purePhoneNumber", default)]
    pub pure_phone_number: String,
    #[serde(rename = "countryCode", default)]
    pub country_code: String,
    #[serde(rename = "watermark", default)]
    pub watermark: Watermark,
}

impl WxMaPhoneNumberInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaPhoneNumberInfo 解析失败: {e}"))
    }
}
