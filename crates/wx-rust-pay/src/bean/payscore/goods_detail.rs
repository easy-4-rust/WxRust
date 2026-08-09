//! 对应 Java `com.github.binarywang.wxpay.bean.payscore.GoodsDetail.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoodsDetail {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "goods_id")]
    pub goods_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "quantity")]
    pub quantity: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "unit_price"
    )]
    pub unit_price: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "discount_amount"
    )]
    pub discount_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "goods_remark"
    )]
    pub goods_remark: Option<String>,
}
