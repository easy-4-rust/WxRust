//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopOrderInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopOrderInfo {
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "out_order_id", default)]
    pub out_order_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "out_user_id", default)]
    pub out_user_id: String,
    #[serde(rename = "order_detail", default)]
    pub order_detail: WxMaShopOrderDetail,
    #[serde(rename = "delivery_detail", default)]
    pub delivery_detail: WxMaShopDeliveryDetail,
    #[serde(rename = "address_info", default)]
    pub address_info: WxMaShopAddressInfo,
    #[serde(rename = "fund_type", default)]
    pub fund_type: i32,
    #[serde(rename = "expire_time", default)]
    pub expire_time: i64,
    #[serde(rename = "aftersale_duration", default)]
    pub aftersale_duration: i32,
    #[serde(rename = "trace_id", default)]
    pub trace_id: String,
    #[serde(rename = "default_receiving_address", default)]
    pub default_receiving_address: WxMaShopAddressInfo,
    #[serde(rename = "stringify_64bits_number", default)]
    pub stringify64bits_number: bool,
}
