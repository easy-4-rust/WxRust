//! 对应 Java `me.chanjar.weixin.open.bean.result.WxFastMaCanSetCategoryResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxFastMaCanSetCategoryResult {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "categories_list", default)]
    pub categories_list: CategoriesListBean,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoriesListBean {
    #[serde(rename = "categories", default)]
    pub categories: Vec<CategoriesBean>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoriesBean {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "qualify", default)]
    pub qualify: QualifyBean,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "level", default)]
    pub level: i32,
    #[serde(rename = "father", default)]
    pub father: i32,
    #[serde(rename = "sensitive_type", default)]
    pub sensitive_type: i32,
    #[serde(rename = "available_for_plugin", default)]
    pub available_for_plugin: bool,
    #[serde(rename = "is_hidden", default)]
    pub is_hidden: bool,
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "need_report", default)]
    pub need_report: i32,
    #[serde(rename = "can_use_cityserivce", default)]
    pub can_use_city_service: i32,
    #[serde(rename = "children", default)]
    pub children: Vec<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QualifyBean {
    #[serde(rename = "available_api_list", default)]
    pub remark: String,
}
