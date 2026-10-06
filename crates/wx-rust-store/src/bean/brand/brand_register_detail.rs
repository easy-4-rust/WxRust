//! 对应 Java `com.binarywang.wxjava.store.bean.brand.BrandRegisterDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 BrandRegisterDetail；对应 Java com.binarywang.wxjava.store.bean.brand.BrandRegisterDetail.java。
pub struct BrandRegisterDetail {
    #[serde(rename = "registrant", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant: Option<String>,
    #[serde(rename = "register_no", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_no: Option<String>,
    #[serde(rename = "start_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(rename = "end_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(rename = "is_permanent", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
    #[serde(rename = "register_certifications", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_certifications: Option<Vec<String>>,
    #[serde(rename = "renew_certifications", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew_certifications: Option<Vec<String>>,
}
