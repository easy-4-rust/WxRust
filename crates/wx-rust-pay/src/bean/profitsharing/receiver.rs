//! 对应 Java `com.github.binarywang.wxpay.bean.profitsharing.Receiver.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Receiver {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "account")]
    pub account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "description"
    )]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "name")]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "relation_type"
    )]
    pub relation_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "custom_relation"
    )]
    pub custom_relation: Option<String>,
}
