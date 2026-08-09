//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenMaCategoryNameListResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaCategoryNameListResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "category_name_list", default)]
    pub category_name_list: Vec<CategoryName>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryName {
    #[serde(rename = "first_id", default)]
    pub first_id: i32,
    #[serde(rename = "first_name", default)]
    pub first_name: String,
    #[serde(rename = "second_id", default)]
    pub second_id: i32,
    #[serde(rename = "second_name", default)]
    pub second_name: String,
}
