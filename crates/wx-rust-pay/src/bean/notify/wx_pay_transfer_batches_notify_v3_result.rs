//! 对应 Java `com.github.binarywang.wxpay.bean.notify.WxPayTransferBatchesNotifyV3Result.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayTransferBatchesNotifyV3Result {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rawData")]
    pub raw_data: Option<OriginNotifyResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "result")]
    pub result: Option<DecryptNotifyResult>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecryptNotifyResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_batch_no"
    )]
    pub out_batch_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batch_id")]
    pub batch_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "batch_status"
    )]
    pub batch_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total_num")]
    pub total_num: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_amount"
    )]
    pub total_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "success_amount"
    )]
    pub success_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "success_num"
    )]
    pub success_num: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "fail_amount"
    )]
    pub fail_amount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fail_num")]
    pub fail_num: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "close_reason"
    )]
    pub close_reason: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "update_time"
    )]
    pub update_time: Option<String>,
}
