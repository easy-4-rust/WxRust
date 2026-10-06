//! 对应 Java `com.binarywang.wxjava.store.bean.category.CategoryAndQualificationList.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::CategoryQualification;

/// 微信小店 CategoryAndQualificationList 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryAndQualificationList.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryAndQualificationList {
    #[serde(rename = "cat_and_qua", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<CategoryQualification>>,
}
