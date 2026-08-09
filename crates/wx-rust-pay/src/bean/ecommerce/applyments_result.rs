//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.ApplymentsResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplymentsResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "applyment_id"
    )]
    pub applyment_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_request_no"
    )]
    pub out_request_no: Option<String>,
}
