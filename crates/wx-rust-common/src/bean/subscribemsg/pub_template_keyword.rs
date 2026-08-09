//! 对应 Java `me.chanjar.weixin.common.bean.subscribemsg.PubTemplateKeyword`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PubTemplateKeyword {
    /// kid
    #[serde(rename = "kid", default)]
    pub kid: i32,
    /// name
    #[serde(rename = "name", default)]
    pub name: String,
    /// example
    #[serde(rename = "example", default)]
    pub example: String,
    /// rule
    #[serde(rename = "rule", default)]
    pub rule: String,
}
