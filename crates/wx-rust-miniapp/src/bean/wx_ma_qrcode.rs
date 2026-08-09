//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaQrcode.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaQrcode {
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "width", default)]
    pub width: i32,
}

impl WxMaQrcode {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaQrcode 解析失败: {e}"))
    }
}
