//! 对应 Java `me.chanjar.weixin.open.bean.shoppingOrders.ShoppingInfoVerifyUpload.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShoppingInfoVerifyUpload {
    #[serde(rename = "order_key", default)]
    pub order_key: OrderKeyBean,
    #[serde(rename = "payer", default)]
    pub payer: PayerBean,
}
