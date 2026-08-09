//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.BusiFavorQueryUserCouponsRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BusiFavorQueryUserCouponsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stock_id")]
    pub stock_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_state"
    )]
    pub coupon_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "creator_merchant"
    )]
    pub creator_merchant: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "belong_merchant"
    )]
    pub belong_merchant: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sender_merchant"
    )]
    pub sender_merchant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offset")]
    pub offset: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limit")]
    pub limit: Option<i32>,
}
