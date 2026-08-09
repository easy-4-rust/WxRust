//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxMaOpenTab.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOpenTab {
    #[serde(rename = "pagePath", default)]
    pub page_path: String,
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "iconPath", default)]
    pub icon_path: String,
    #[serde(rename = "selectedIconPath", default)]
    pub selected_icon_path: String,
}
