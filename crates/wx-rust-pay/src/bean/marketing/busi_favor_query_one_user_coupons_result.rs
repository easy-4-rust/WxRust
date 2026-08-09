//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.BusiFavorQueryOneUserCouponsResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::marketing::busifavor::CustomEntrance;
#[allow(unused_imports)]
use crate::bean::marketing::busifavor::DisplayPatternInfo;
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BusiFavorQueryOneUserCouponsResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "belong_merchant"
    )]
    pub belong_merchant: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stock_name"
    )]
    pub stock_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "comment")]
    pub comment: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "goods_name"
    )]
    pub goods_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stock_type"
    )]
    pub stock_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transferable"
    )]
    pub transferable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shareable")]
    pub shareable: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_state"
    )]
    pub coupon_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "display_pattern_info"
    )]
    pub display_pattern_info: Option<DisplayPatternInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_use_rule"
    )]
    pub coupon_use_rule: Option<CouponUseRule>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "custom_entrance"
    )]
    pub custom_entrance: Option<CustomEntrance>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_code"
    )]
    pub coupon_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stock_id")]
    pub stock_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "available_start_time"
    )]
    pub available_start_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "expire_time"
    )]
    pub expire_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "receive_time"
    )]
    pub receive_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "send_request_no"
    )]
    pub send_request_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "use_request_no"
    )]
    pub use_request_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "use_time")]
    pub use_time: Option<String>,
}
