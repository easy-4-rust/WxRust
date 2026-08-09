//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.busifavor.ExchangeCoupon.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeCoupon {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "exchange_price"
    )]
    pub exchange_price: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_minimum"
    )]
    pub transaction_minimum: Option<i32>,
}
