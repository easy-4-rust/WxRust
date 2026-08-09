//! 对应 Java `com.github.binarywang.wxpay.bean.transfer.ReservationTransferBatchGetResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReservationTransferBatchGetResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mch_id")]
    pub mch_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_batch_no"
    )]
    pub out_batch_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "reservation_batch_no"
    )]
    pub reservation_batch_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "batch_remark"
    )]
    pub batch_remark: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_scene_id"
    )]
    pub transfer_scene_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "batch_state"
    )]
    pub batch_state: Option<String>,
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
        rename = "close_reason"
    )]
    pub close_reason: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "need_query_detail"
    )]
    pub need_query_detail: Option<bool>,
    #[serde(default, rename = "transfer_detail_list")]
    pub transfer_detail_list: Vec<TransferDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferDetail {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_detail_no"
    )]
    pub out_detail_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_bill_no"
    )]
    pub transfer_bill_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "detail_state"
    )]
    pub detail_state: Option<String>,
}
