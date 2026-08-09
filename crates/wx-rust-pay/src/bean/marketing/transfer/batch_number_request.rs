//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.transfer.BatchNumberRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchNumberRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batch_id")]
    pub batch_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "need_query_detail"
    )]
    pub need_query_detail: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offset")]
    pub offset: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limit")]
    pub limit: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "detail_status"
    )]
    pub detail_status: Option<String>,
}
