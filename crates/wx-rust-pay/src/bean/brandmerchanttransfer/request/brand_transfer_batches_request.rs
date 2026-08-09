//! 对应 Java `com.github.binarywang.wxpay.bean.brandmerchanttransfer.request.BrandTransferBatchesRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandTransferBatchesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "brand_id")]
    pub brand_id: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "brand_appid"
    )]
    pub brand_appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scene")]
    pub scene: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "template_id"
    )]
    pub template_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_batch_no"
    )]
    pub out_batch_no: Option<String>,
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
        rename = "total_amount"
    )]
    pub total_amount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total_num")]
    pub total_num: Option<i32>,
    #[serde(default, rename = "detail_list")]
    pub detail_list: Vec<BrandTransferDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandTransferDetail {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_detail_no"
    )]
    pub out_detail_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "user_name")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remark")]
    pub remark: Option<String>,
}
