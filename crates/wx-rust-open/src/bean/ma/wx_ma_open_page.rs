//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxMaOpenPage.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOpenPage {
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
    pub enable_pull_down_refresh: bool,
    #[serde(rename = "onReachBottomDistance", default)]
    pub on_reach_bottom_distance: i32,
    #[serde(rename = "disableScroll", default)]
    pub disable_scroll: bool,
}
