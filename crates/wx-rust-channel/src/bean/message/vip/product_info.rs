//! 商品信息（会员积分兑换）。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.vip.ProductInfo.java`。

use serde::{Deserialize, Serialize};

/// 商品信息（对应 Java `ProductInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProductInfo {
    /// 兑换的商品ID（对应 Java `relatedProductId`）。
    #[serde(
        rename = "related_product_id",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub related_product_id: Option<i64>,
}
