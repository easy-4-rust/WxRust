//! 对应 Java `com.github.binarywang.wxpay.bean.payscore.PromotionDetail.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromotionDetail {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "coupon_id")]
    pub coupon_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "name")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scope")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stock_id")]
    pub stock_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "wechatpay_contribute"
    )]
    pub wechatpay_contribute: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_contribute"
    )]
    pub merchant_contribute: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "other_contribute"
    )]
    pub other_contribute: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currency")]
    pub currency: Option<String>,
    #[serde(default, rename = "goods_detail")]
    pub goods_detail: Vec<GoodsDetail>,
}
