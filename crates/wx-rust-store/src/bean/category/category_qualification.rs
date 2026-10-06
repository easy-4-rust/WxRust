//! 对应 Java `com.binarywang.wxjava.store.bean.category.CategoryQualification.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{QualificationInfo, ShopCategory};

/// 微信小店 CategoryQualification 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryQualification.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryQualification {
    #[serde(rename = "cat", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<ShopCategory>,
    #[serde(rename = "qua", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<QualificationInfo>,
    #[serde(rename = "product_qua", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_info: Option<QualificationInfo>,
    #[serde(rename = "brand_qua", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_qua: Option<QualificationInfo>,
    #[serde(rename = "product_qua_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_qua_list: Option<Vec<QualificationInfo>>,
    #[serde(rename = "is_confidence_require_bad_must_pay", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_require_bad_must_pay: Option<bool>,
}
