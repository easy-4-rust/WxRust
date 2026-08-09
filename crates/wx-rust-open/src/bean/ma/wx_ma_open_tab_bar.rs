//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxMaOpenTabBar.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOpenTabBar {
    #[serde(rename = "color", default)]
    pub color: String,
    #[serde(rename = "selectedColor", default)]
    pub selected_color: String,
    #[serde(rename = "backgroundColor", default)]
    pub background_color: String,
    #[serde(rename = "borderStyle", default)]
    pub border_style: String,
    #[serde(rename = "list", default)]
    pub tab_list: Vec<WxMaOpenTab>,
    #[serde(rename = "position", default)]
    pub position: String,
}
