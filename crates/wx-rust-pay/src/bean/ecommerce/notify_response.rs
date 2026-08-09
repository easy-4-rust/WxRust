//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.NotifyResponse.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "id")]
    pub id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "create_time"
    )]
    pub create_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "event_type"
    )]
    pub event_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "resource_type"
    )]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resource")]
    pub resource: Option<Resource>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "summary")]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "algorithm")]
    pub algorithm: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ciphertext"
    )]
    pub ciphertext: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "associated_data"
    )]
    pub associated_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonce")]
    pub nonce: Option<String>,
}
