//! 对应 Java `com.github.binarywang.wxpay.bean.brandmerchanttransfer.result.BrandBatchesQueryResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandBatchesQueryResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "brand_mchid"
    )]
    pub brand_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batch_no")]
    pub batch_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_batch_no"
    )]
    pub out_batch_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "brand_id")]
    pub brand_id: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "template_id"
    )]
    pub template_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "brand_appid"
    )]
    pub brand_appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "batch_state"
    )]
    pub batch_state: Option<String>,
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
    #[serde(default, rename = "detail_list")]
    pub detail_list: Vec<BrandDetailResult>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandDetailResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_detail_no"
    )]
    pub transfer_detail_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_detail_no"
    )]
    pub out_detail_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "detail_state"
    )]
    pub detail_state: Option<String>,
}
