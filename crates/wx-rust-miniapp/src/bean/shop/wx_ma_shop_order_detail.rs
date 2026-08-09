//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopOrderDetail.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopOrderDetail {
    #[serde(rename = "promotion_info", default)]
    pub promotion_info: WxMaShopPromotionInfo,
    #[serde(rename = "product_infos", default)]
    pub product_infos: Vec<WxMaShopProductInfo>,
    #[serde(rename = "pay_info", default)]
    pub pay_info: WxMaShopPayInfo,
    #[serde(rename = "price_info", default)]
    pub price_info: WxMaShopPriceInfo,
    #[serde(rename = "multi_pay_info", default)]
    pub multi_pay_info: Vec<WxMaShopPayInfo>,
    #[serde(rename = "delivery_detail", default)]
    pub delivery_detail: WxMaShopDeliveryDetail,
}
