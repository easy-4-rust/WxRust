//! 对应 Java `com.github.binarywang.wxpay.bean.bank.BankingResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankingResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_count"
    )]
    pub total_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "count")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offset")]
    pub offset: Option<i32>,
    #[serde(default, rename = "data")]
    pub data: Vec<BankInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "links")]
    pub links: Option<Link>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Link {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "next")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prev")]
    pub prev: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "self")]
    pub self_: Option<String>,
}
