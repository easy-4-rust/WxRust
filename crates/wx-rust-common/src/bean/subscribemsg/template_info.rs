//! 对应 Java `me.chanjar.weixin.common.bean.subscribemsg.TemplateInfo`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateInfo {
    /// priTmplId
    #[serde(rename = "priTmplId", default)]
    pub pri_tmpl_id: String,
    /// title
    #[serde(rename = "title", default)]
    pub title: String,
    /// content
    #[serde(rename = "content", default)]
    pub content: String,
    /// example
    #[serde(rename = "example", default)]
    pub example: String,
    /// type
    #[serde(rename = "type", default)]
    pub r#type: i32,
}
