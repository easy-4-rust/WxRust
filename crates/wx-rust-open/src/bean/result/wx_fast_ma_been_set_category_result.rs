//! 对应 Java `me.chanjar.weixin.open.bean.result.WxFastMaBeenSetCategoryResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxFastMaBeenSetCategoryResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "limit", default)]
    pub limit: i32,
    #[serde(rename = "quota", default)]
    pub quota: i32,
    #[serde(rename = "category_limit", default)]
    pub category_limit: i32,
    #[serde(rename = "categories", default)]
    pub categories: Vec<CategoriesBean>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoriesBean {
    #[serde(rename = "first", default)]
    pub first: i32,
    #[serde(rename = "first_name", default)]
    pub first_name: String,
    #[serde(rename = "second", default)]
    pub second: i32,
    #[serde(rename = "second_name", default)]
    pub second_name: String,
    #[serde(rename = "audit_status", default)]
    pub audit_status: i32,
    #[serde(rename = "audit_reason", default)]
    pub audit_reason: String,
}
