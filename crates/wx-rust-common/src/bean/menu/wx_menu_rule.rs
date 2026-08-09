//! 对应 Java `me.chanjar.weixin.common.bean.menu.WxMenuRule`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMenuRule {
    /// tagId
    #[serde(rename = "tag_id", alias = "group_id", default)]
    pub tag_id: String,
    /// sex
    #[serde(rename = "sex", default)]
    pub sex: String,
    /// country
    #[serde(rename = "country", default)]
    pub country: String,
    /// province
    #[serde(rename = "province", default)]
    pub province: String,
    /// city
    #[serde(rename = "city", default)]
    pub city: String,
    /// clientPlatformType
    #[serde(rename = "client_platform_type", default)]
    pub client_platform_type: String,
    /// language
    #[serde(rename = "language", default)]
    pub language: String,
}
