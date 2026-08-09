//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxaCodeUnlimit.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxaCodeUnlimit {
    #[serde(rename = "scene", default)]
    pub scene: String,
    #[serde(rename = "page", default)]
    pub page: String,
    #[serde(rename = "check_path", default)]
    pub check_path: bool,
    #[serde(rename = "env_version", default)]
    pub env_version: String,
    #[serde(rename = "width", default)]
    pub width: i32,
    #[serde(rename = "auto_color", default)]
    pub auto_color: bool,
    #[serde(rename = "is_hyaline", default)]
    pub is_hyaline: bool,
    #[serde(rename = "line_color", default)]
    pub line_color: WxMaCodeLineColor,
}

impl WxaCodeUnlimit {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxaCodeUnlimit 解析失败: {e}"))
    }
}
