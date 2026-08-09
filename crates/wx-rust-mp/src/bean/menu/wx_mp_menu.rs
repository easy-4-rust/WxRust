//! 公众号专用菜单（可能包含个性化菜单）。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.menu.WxMpMenu`。

use serde::{Deserialize, Serialize};
use wx_rust_common::bean::menu::{WxMenuButton, WxMenuRule};

/// 公众号专用菜单。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpMenu {
    /// 默认菜单。
    #[serde(rename = "menu", skip_serializing_if = "Option::is_none")]
    pub menu: Option<WxMpConditionalMenu>,
    /// 个性化菜单列表。
    #[serde(rename = "conditionalmenu", default)]
    pub conditional_menu: Vec<WxMpConditionalMenu>,
}

impl WxMpMenu {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("菜单解析失败: {e}"))
    }

    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

/// 菜单（默认或个性化，对应 Java `WxMpMenu.WxMpConditionalMenu`）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpConditionalMenu {
    /// 菜单按钮列表。
    #[serde(rename = "button", default)]
    pub buttons: Vec<WxMenuButton>,
    /// 个性化菜单匹配规则。
    #[serde(rename = "matchrule", skip_serializing_if = "Option::is_none")]
    pub rule: Option<WxMenuRule>,
    /// 菜单 id（微信返回数字或字符串，统一转字符串，对应 Java Gson 数字→String 强转）。
    #[serde(
        rename = "menuid",
        default,
        deserialize_with = "deserialize_string_or_int"
    )]
    pub menu_id: Option<String>,
}

/// 数字或字符串统一解析为 `Option<String>`（对应 Java Gson 的 String 字段接收数字）。
fn deserialize_string_or_int<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(v.map(|v| match v {
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => s,
        _ => String::new(),
    }))
}
