//! 对应 Java `com.github.binarywang.wxpay.bean.transfer.QueryTransferBatchesResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QueryTransferBatchesResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offset")]
    pub offset: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limit")]
    pub limit: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_batch"
    )]
    pub transfer_batch: Option<TransferBatch>,
    #[serde(default, rename = "transfer_detail_list")]
    pub transfer_detail_list: Vec<TransferDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferBatch {
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "batch_status"
    )]
    pub batch_status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "batch_type"
    )]
    pub batch_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "batch_name"
    )]
    pub batch_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "batch_remark"
    )]
    pub batch_remark: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "close_reason"
    )]
    pub close_reason: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_amount"
    )]
    pub total_amount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total_num")]
    pub total_num: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "create_time"
    )]
    pub create_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "update_time"
    )]
    pub update_time: Option<String>,
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
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferDetail {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "detail_id")]
    pub detail_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_detail_no"
    )]
    pub out_detail_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "detail_status"
    )]
    pub detail_status: Option<String>,
}
