//! 对应 Java `com.github.binarywang.wxpay.bean.merchanttransfer.ElectronicBillResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ElectronicBillResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_bill_no"
    )]
    pub out_batch_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "signature_no"
    )]
    pub signature_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "state")]
    pub signature_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hash_type")]
    pub hash_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "hash_value"
    )]
    pub hash_value: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "download_url"
    )]
    pub download_url: Option<String>,
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
}
