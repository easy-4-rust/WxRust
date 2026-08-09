//! 对应 Java `me.chanjar.weixin.common.bean.subscribemsg.CategoryData`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryData {
    /// id
    #[serde(rename = "id", default)]
    pub id: i32,
    /// name
    #[serde(rename = "name", default)]
    pub name: String,
}
