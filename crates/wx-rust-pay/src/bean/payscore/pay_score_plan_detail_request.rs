//! 对应 Java `com.github.binarywang.wxpay.bean.payscore.PayScorePlanDetailRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PayScorePlanDetailRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "original_price"
    )]
    pub original_price: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "plan_discount_description"
    )]
    pub plan_discount_description: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "actual_price"
    )]
    pub actual_price: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "plan_detail_name"
    )]
    pub plan_detail_name: Option<String>,
}
