//! 对应 Java `com.github.binarywang.wxpay.bean.complaint.NegotiationHistoryRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NegotiationHistoryRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_id"
    )]
    pub complaint_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limit")]
    pub limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offset")]
    pub offset: Option<i32>,
}
