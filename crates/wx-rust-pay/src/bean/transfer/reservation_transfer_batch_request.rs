//! 对应 Java `com.github.binarywang.wxpay.bean.transfer.ReservationTransferBatchRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReservationTransferBatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_batch_no"
    )]
    pub out_batch_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_scene_id"
    )]
    pub transfer_scene_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "batch_remark"
    )]
    pub batch_remark: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_amount"
    )]
    pub total_amount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total_num")]
    pub total_num: Option<i32>,
    #[serde(default, rename = "transfer_detail_list")]
    pub transfer_detail_list: Vec<TransferDetail>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "notify_url"
    )]
    pub notify_url: Option<String>,
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
        rename = "transfer_amount"
    )]
    pub transfer_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_remark"
    )]
    pub transfer_remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "user_name")]
    pub user_name: Option<String>,
    #[serde(default, rename = "transfer_scene_report_infos")]
    pub transfer_scene_report_infos: Vec<TransferSceneReportInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferSceneReportInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "info_type")]
    pub info_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "info_content"
    )]
    pub info_content: Option<String>,
}
