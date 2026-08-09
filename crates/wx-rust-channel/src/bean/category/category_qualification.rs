//! 对应 Java `me.chanjar.weixin.channel.bean.category.CategoryQualification.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryQualification {
    #[serde(rename = "cat", default)]
    pub category: ShopCategory,
    #[serde(rename = "qua", default)]
    pub info: QualificationInfo,
    #[serde(rename = "product_qua", default)]
    pub product_info: QualificationInfo,
    #[serde(rename = "brand_qua", default)]
    pub brand_qua: QualificationInfo,
    #[serde(rename = "product_qua_list", default)]
    pub product_qua_list: Vec<QualificationInfo>,
    #[serde(rename = "is_confidence_require_bad_must_pay", default)]
    pub confidence_require_bad_must_pay: bool,
}
