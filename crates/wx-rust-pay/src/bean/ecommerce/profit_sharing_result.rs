//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.ProfitSharingResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProfitSharingResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_order_no"
    )]
    pub out_order_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "order_id")]
    pub order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "status")]
    pub status: Option<String>,
    #[serde(default, rename = "receivers")]
    pub receivers: Vec<Receiver>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "close_reason"
    )]
    pub close_reason: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "finish_amount"
    )]
    pub finish_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "finish_description"
    )]
    pub finish_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Receiver {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "receiver_mchid"
    )]
    pub receiver_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "description"
    )]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "result")]
    pub result: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "finish_time"
    )]
    pub finish_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "fail_reason"
    )]
    pub fail_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "receiver_account"
    )]
    pub receiver_account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "detail_id")]
    pub detail_id: Option<String>,
}
