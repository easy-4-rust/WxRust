//! 对应 Java `me.chanjar.weixin.common.bean.subscribemsg.PubTemplateTitleListResult`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PubTemplateTitleListResult {
    /// count
    #[serde(rename = "count", default)]
    pub count: i32,
    /// data
    #[serde(rename = "data", default)]
    pub data: Vec<TemplateItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateItem {
    /// type
    #[serde(rename = "type", default)]
    pub r#type: i32,
    /// tid
    #[serde(rename = "tid", default)]
    pub tid: i32,
    /// categoryId
    #[serde(rename = "categoryId", default)]
    pub category_id: String,
    /// title
    #[serde(rename = "title", default)]
    pub title: String,
}
