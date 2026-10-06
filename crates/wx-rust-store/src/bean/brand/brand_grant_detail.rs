//! 对应 Java `com.binarywang.wxjava.store.bean.brand.BrandGrantDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 BrandGrantDetail；对应 Java com.binarywang.wxjava.store.bean.brand.BrandGrantDetail.java。
pub struct BrandGrantDetail {
    #[serde(rename = "grant_certifications", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_certifications: Option<Vec<String>>,
    #[serde(rename = "grant_level", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_level: Option<i32>,
    #[serde(rename = "start_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(rename = "end_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(rename = "is_permanent", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
    #[serde(rename = "brand_owner_id_photos", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_owner_id_photos: Option<Vec<String>>,
}
