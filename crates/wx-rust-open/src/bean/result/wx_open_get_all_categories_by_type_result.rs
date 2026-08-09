//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenGetAllCategoriesByTypeResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenGetAllCategoriesByTypeResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "categories_list", default)]
    pub categorieslist: CategoriesList,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoriesList {
    #[serde(rename = "categories", default)]
    pub categories: Vec<Categories>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Categories {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "level", default)]
    pub level: i32,
    #[serde(rename = "father", default)]
    pub father: i32,
    #[serde(rename = "children", default)]
    pub children: Vec<i32>,
    #[serde(rename = "sensitive_type", default)]
    pub sensitive_type: i32,
    #[serde(rename = "qualify", default)]
    pub qualify: Qualify,
    #[serde(rename = "scope", default)]
    pub scope: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Qualify {
    #[serde(rename = "exter_list", default)]
    pub exter_list: Vec<Exter>,
    #[serde(rename = "remark", default)]
    pub remark: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Exter {
    #[serde(rename = "inner_list", default)]
    pub inner_list: Vec<Inner>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inner {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "url", default)]
    pub url: String,
}
