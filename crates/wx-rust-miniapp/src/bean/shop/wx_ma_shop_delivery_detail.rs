//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopDeliveryDetail.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopDeliveryDetail {
    #[serde(rename = "delivery_type", default)]
    pub delivery_type: i32,
    #[serde(rename = "finish_all_delivery", default)]
    pub finish_all_delivery: i32,
    #[serde(rename = "delivery_list", default)]
    pub delivery_list: Vec<WxMaShopDeliveryItem>,
}
