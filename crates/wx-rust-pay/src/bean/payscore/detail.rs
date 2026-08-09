//! 对应 Java `com.github.binarywang.wxpay.bean.payscore.Detail.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Detail {
    #[serde(default, rename = "seq")]
    pub seq: i32,
    #[serde(default, rename = "amount")]
    pub amount: i32,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "paid_type")]
    pub paid_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "paid_time")]
    pub paid_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(default, rename = "promotion_detail")]
    pub promotion_detail: Vec<PromotionDetail>,
}
