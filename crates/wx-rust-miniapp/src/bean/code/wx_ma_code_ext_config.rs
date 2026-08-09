//! 对应 Java `cn.binarywang.wx.miniapp.bean.code.WxMaCodeExtConfig.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCodeExtConfig {
    #[serde(rename = "extEnable", default)]
    pub ext_enable: bool,
    #[serde(rename = "extAppid", default)]
    pub ext_appid: String,
    #[serde(rename = "ext", default)]
    pub ext: serde_json::Value,
    #[serde(rename = "extPages", default)]
    pub ext_pages: std::collections::HashMap<String, PageConfig>,
    #[serde(rename = "directCommit", default)]
    pub direct_commit: bool,
    #[serde(rename = "pages", default)]
    pub pages: Vec<String>,
    #[serde(rename = "window", default)]
    pub window: PageConfig,
    #[serde(rename = "networkTimeout", default)]
    pub network_timeout: NetworkTimeout,
    #[serde(rename = "debug", default)]
    pub debug: bool,
    #[serde(rename = "tabBar", default)]
    pub tab_bar: TabBar,
    #[serde(rename = "requiredPrivateInfos", default)]
    pub required_private_infos: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PageConfig {
    #[serde(rename = "navigationBarBackgroundColor", default)]
    pub navigation_bar_background_color: String,
    #[serde(rename = "navigationBarTextStyle", default)]
    pub navigation_bar_text_style: String,
    #[serde(rename = "navigationBarTitleText", default)]
    pub navigation_bar_title_text: String,
    #[serde(rename = "backgroundColor", default)]
    pub background_color: String,
    #[serde(rename = "backgroundTextStyle", default)]
    pub background_text_style: String,
    #[serde(rename = "enablePullDownRefresh", default)]
    pub enable_pull_down_refresh: String,
    #[serde(rename = "disableScroll", default)]
    pub disable_scroll: bool,
    #[serde(rename = "onReachBottomDistance", default)]
    pub on_reach_bottom_distance: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TabBar {
    #[serde(rename = "color", default)]
    pub color: String,
    #[serde(rename = "selectedColor", default)]
    pub selected_color: String,
    #[serde(rename = "backgroundColor", default)]
    pub background_color: String,
    #[serde(rename = "borderStyle", default)]
    pub border_style: String,
    #[serde(rename = "list", default)]
    pub list: Vec<Item>,
    #[serde(rename = "position", default)]
    pub position: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "pagePath", default)]
    pub page_path: String,
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "iconPath", default)]
    pub icon_path: String,
    #[serde(rename = "selectedIconPath", default)]
    pub selected_icon_path: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NetworkTimeout {
    #[serde(rename = "request", default)]
    pub request: i32,
    #[serde(rename = "connectSocket", default)]
    pub connect_socket: i32,
    #[serde(rename = "uploadFile", default)]
    pub upload_file: i32,
    #[serde(rename = "downloadFile", default)]
    pub download_file: i32,
}
