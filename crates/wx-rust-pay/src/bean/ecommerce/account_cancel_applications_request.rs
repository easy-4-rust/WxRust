//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.AccountCancelApplicationsRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountCancelApplicationsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_apply_no"
    )]
    pub out_apply_no: Option<String>,
    #[serde(default, rename = "application_info")]
    pub application_info: Vec<CancelApplicationInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelApplicationInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "application_type"
    )]
    pub application_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "application_media_id"
    )]
    pub application_media_id: Option<String>,
}
