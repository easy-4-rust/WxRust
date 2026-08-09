//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.GetOrderRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetOrderRequest {
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "shop_order_id", default)]
    pub shop_order_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "shop_no", default)]
    pub shop_no: String,
    #[serde(rename = "shopid", default)]
    pub shop_id: String,
    #[serde(rename = "delivery_sign", default)]
    pub delivery_sign: String,
    #[serde(rename = "appSecret", default)]
    pub app_secret: String,
}
