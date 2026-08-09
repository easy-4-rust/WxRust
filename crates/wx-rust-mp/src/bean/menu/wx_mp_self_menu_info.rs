//! 自定义菜单信息。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.menu.WxMpSelfMenuInfo`。

use serde::{Deserialize, Serialize};

/// 自定义菜单信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpSelfMenuInfo {
    /// 菜单按钮列表。
    #[serde(rename = "button", default)]
    pub buttons: Vec<WxMpSelfMenuButton>,
}

/// 自定义菜单按钮。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpSelfMenuButton {
    /// 按钮类型。
    #[serde(rename = "type", default)]
    pub r#type: String,
    /// 按钮名称。
    #[serde(rename = "name", default)]
    pub name: String,
    /// 按钮 key。
    #[serde(rename = "key", default)]
    pub key: String,
    /// 跳转 url。
    #[serde(rename = "url", default)]
    pub url: String,
    /// 按钮值。
    #[serde(rename = "value", default)]
    pub value: String,
    /// 素材 media_id。
    #[serde(rename = "media_id", default)]
    pub media_id: String,
    /// 发布 article_id。
    #[serde(rename = "article_id", default)]
    pub article_id: String,
    /// 小程序 appid。
    #[serde(rename = "appid", default)]
    pub app_id: String,
    /// 小程序页面路径。
    #[serde(rename = "pagepath", default)]
    pub page_path: String,
    /// 子按钮列表。
    #[serde(rename = "sub_button", default)]
    pub sub_buttons: Vec<WxMpSelfMenuButton>,
}
