//! 对应 Java `bean.card.CustomField`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CustomField {
    #[serde(rename = "name_type", default)]
    pub name_type: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "app_brand_user_name", default)]
    pub app_brand_user_name: String,
    #[serde(rename = "app_brand_pass", default)]
    pub app_brand_pass: String,
}
