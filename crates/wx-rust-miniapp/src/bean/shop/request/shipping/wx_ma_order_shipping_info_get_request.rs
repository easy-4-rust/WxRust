//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.shipping.WxMaOrderShippingInfoGetRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::request::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOrderShippingInfoGetRequest {
    #[serde(rename = "transaction_id", default)]
    pub transaction_id: String,
    #[serde(rename = "merchant_id", default)]
    pub merchant_id: String,
    #[serde(rename = "sub_merchant_id", default)]
    pub sub_merchant_id: String,
    #[serde(rename = "merchant_trade_no", default)]
    pub merchant_trade_no: String,
}
