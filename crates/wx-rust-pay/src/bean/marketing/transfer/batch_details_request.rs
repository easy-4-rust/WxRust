//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.transfer.BatchDetailsRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchDetailsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batch_id")]
    pub batch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "detail_id")]
    pub detail_id: Option<String>,
}
