//! 自定义菜单配置接口返回结果。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.menu.WxMpGetSelfMenuInfoResult`。

use serde::{Deserialize, Serialize};

use super::WxMpSelfMenuInfo;

/// 自定义菜单配置接口返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpGetSelfMenuInfoResult {
    /// 菜单信息。
    #[serde(rename = "selfmenu_info", skip_serializing_if = "Option::is_none")]
    pub self_menu_info: Option<WxMpSelfMenuInfo>,
    /// 菜单是否开启。
    #[serde(rename = "is_menu_open", default)]
    pub is_menu_open: i32,
}

impl WxMpGetSelfMenuInfoResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("自定义菜单配置解析失败: {e}"))
    }
}
