//! 对应 Java `com.github.binarywang.wxpay.bean.notify.ComplaintNotifyResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComplaintNotifyResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rawData")]
    pub raw_data: Option<OriginNotifyResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "result")]
    pub result: Option<DecryptNotifyResult>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecryptNotifyResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_id"
    )]
    pub complaint_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "action_type"
    )]
    pub action_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_time"
    )]
    pub complaint_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_phone"
    )]
    pub payer_phone: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_detail"
    )]
    pub complaint_detail: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_state"
    )]
    pub complaint_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_handle_state"
    )]
    pub complaint_handle_state: Option<String>,
}
