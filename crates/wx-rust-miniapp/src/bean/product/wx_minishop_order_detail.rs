//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopOrderDetail.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopOrderDetail {
    #[serde(rename = "product_infos", default)]
    pub product_infos: Vec<WxMinishopProductInfo>,
    #[serde(rename = "pay_info", default)]
    pub pay_info: WxMinishopPayInfo,
    #[serde(rename = "price_info", default)]
    pub price_info: WxMinishopPriceInfo,
    #[serde(rename = "delivery_info", default)]
    pub delivery_info: WxMinishopDeliveryInfo,
}
