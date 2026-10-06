//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleRejectReasonResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::AfterSaleRejectReason;

/// 微信小店 AfterSaleRejectReasonResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleRejectReasonResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleRejectReasonResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "reject_reason_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason_list: Option<Vec<AfterSaleRejectReason>>,
}
