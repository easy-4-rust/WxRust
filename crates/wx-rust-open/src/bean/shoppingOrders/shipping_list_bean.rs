//! 对应 Java `me.chanjar.weixin.open.bean.shoppingOrders.ShippingListBean.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShippingListBean {
    #[serde(rename = "tracking_no", default)]
    pub tracking_no: String,
    #[serde(rename = "express_company", default)]
    pub express_company: String,
    #[serde(rename = "item_list", default)]
    pub item_list: Vec<ShippingItemListBean>,
    #[serde(rename = "contact", default)]
    pub contact: ContactBean,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShippingItemListBean {
    #[serde(rename = "merchant_item_id", default)]
    pub merchant_item_id: String,
}
