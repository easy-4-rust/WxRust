//! 对应 Java `com.github.binarywang.wxpay.bean.complaint.UpdateRefundProgressRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateRefundProgressRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_id"
    )]
    pub complaint_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "action")]
    pub action: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "launch_refund_day"
    )]
    pub launch_refund_day: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "reject_reason"
    )]
    pub reject_reason: Option<String>,
    #[serde(default, rename = "reject_media_list")]
    pub reject_media_list: Vec<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remark")]
    pub remark: Option<String>,
}
