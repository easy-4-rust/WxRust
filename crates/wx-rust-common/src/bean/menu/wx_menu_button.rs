//! 对应 Java `me.chanjar.weixin.common.bean.menu.WxMenuButton`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMenuButton {
    /// type
    #[serde(rename = "type", default)]
    pub r#type: String,
    /// name
    #[serde(rename = "name", default)]
    pub name: String,
    /// key
    #[serde(rename = "key", default)]
    pub key: String,
    /// url
    #[serde(rename = "url", default)]
    pub url: String,
    /// mediaId
    #[serde(rename = "media_id", default)]
    pub media_id: String,
    /// articleId
    #[serde(rename = "article_id", default)]
    pub article_id: String,
    /// appId
    #[serde(rename = "appid", default)]
    pub app_id: String,
    /// pagePath
    #[serde(rename = "pagepath", default)]
    pub page_path: String,
    /// subButtons
    #[serde(rename = "sub_button", default)]
    pub sub_buttons: Vec<WxMenuButton>,
}
