//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.busifavor.StockSendRule.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StockSendRule {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "max_coupons"
    )]
    pub max_coupons: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "max_coupons_per_user"
    )]
    pub max_coupons_per_user: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "max_coupons_by_day"
    )]
    pub max_coupons_by_day: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "natural_person_limit"
    )]
    pub natural_person_limit: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "prevent_api_abuse"
    )]
    pub prevent_api_abuse: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transferable"
    )]
    pub transferable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shareable")]
    pub shareable: Option<bool>,
}
