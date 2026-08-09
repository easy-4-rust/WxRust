//! 对应 Java `com.github.binarywang.wxpay.bean.result.WxPayRefundPromotionDetail.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayRefundPromotionDetail {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "promotion_id"
    )]
    pub promotion_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scope")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_amount"
    )]
    pub refund_amount: Option<i32>,
    #[serde(default, rename = "goods_detail")]
    pub goods_details: Vec<GoodDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoodDetail {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "goods_id")]
    pub goods_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_amount"
    )]
    pub refund_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_quantity"
    )]
    pub refund_quantity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "price")]
    pub price: Option<i32>,
}
