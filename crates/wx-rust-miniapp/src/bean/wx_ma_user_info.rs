//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaUserInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaUserInfo {
    #[serde(rename = "nickName", default)]
    pub nick_name: String,
    #[serde(rename = "gender", default)]
    pub gender: String,
    #[serde(rename = "language", default)]
    pub language: String,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "province", default)]
    pub province: String,
    #[serde(rename = "country", default)]
    pub country: String,
    #[serde(rename = "avatarUrl", default)]
    pub avatar_url: String,
    #[serde(rename = "unionId", default)]
    pub union_id: String,
    #[serde(rename = "watermark", default)]
    pub watermark: Watermark,
}

impl WxMaUserInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaUserInfo 解析失败: {e}"))
    }
}
