//! 对应 Java `me.chanjar.weixin.common.bean.menu.WxMenu`（由 gen_bean_structs.py 生成）。

use super::wx_menu_button::WxMenuButton;
use super::wx_menu_rule::WxMenuRule;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMenu {
    /// buttons
    #[serde(rename = "buttons", default)]
    pub buttons: Vec<WxMenuButton>,
    /// matchRule
    #[serde(rename = "matchRule", default)]
    pub match_rule: Option<WxMenuRule>,
}

impl WxMenu {
    /// 从 JSON 字符串解析菜单。
    ///
    /// 对应 Java `WxMenu.fromJson(String)`。
    ///
    /// # 参数
    /// - `json`：微信菜单 JSON
    ///
    /// # 返回
    /// 解析出的菜单。
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// 序列化为 JSON 字符串。
    ///
    /// 对应 Java `WxMenu.toJson()`。
    ///
    /// # 返回
    /// JSON 字符串。
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
