//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaPluginListResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPluginListResult {
    #[serde(rename = "plugin_list", default)]
    pub plugin_list: Vec<PluginInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginInfo {
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "nickname", default)]
    pub nick_name: String,
    #[serde(rename = "headimgurl", default)]
    pub head_img_url: String,
}

impl WxMaPluginListResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaPluginListResult 解析失败: {e}"))
    }
}
