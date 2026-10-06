//! 对应 Java `com.binarywang.wxjava.store.bean.brand.Brand.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{BrandApplicationDetail, BrandGrantDetail, BrandRegisterDetail};

/// 微信小店 Brand 数据类型；对应 Java com.binarywang.wxjava.store.bean.brand.Brand.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Brand {
    #[serde(rename = "brand_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
    #[serde(rename = "ch_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch_name: Option<String>,
    #[serde(rename = "en_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(rename = "classification_no", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_no: Option<String>,
    #[serde(rename = "trade_mark_symbol", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_mark_symbol: Option<i32>,
    #[serde(rename = "register_details", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_detail: Option<BrandRegisterDetail>,
    #[serde(rename = "application_details", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_detail: Option<BrandApplicationDetail>,
    #[serde(rename = "grant_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<i32>,
    #[serde(rename = "grant_details", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_detail: Option<BrandGrantDetail>,
}
