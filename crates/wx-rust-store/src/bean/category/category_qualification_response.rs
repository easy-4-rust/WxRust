//! 对应 Java `com.binarywang.wxjava.store.bean.category.CategoryQualificationResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::CategoryAndQualificationList;

/// 微信小店 CategoryQualificationResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryQualificationResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryQualificationResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "cats", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<CategoryAndQualificationList>>,
    #[serde(rename = "cats_v2", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cats_v2: Option<Vec<CategoryAndQualificationList>>,
}
