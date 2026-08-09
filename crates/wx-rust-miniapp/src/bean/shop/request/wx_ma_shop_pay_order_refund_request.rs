//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopPayOrderRefundRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopPayOrderRefundRequest {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "mchid", default)]
    pub mchid: String,
    #[serde(rename = "trade_no", default)]
    pub trade_no: String,
    #[serde(rename = "transaction_id", default)]
    pub transaction_id: String,
    #[serde(rename = "refund_no", default)]
    pub refund_no: String,
    #[serde(rename = "total_amount", default)]
    pub total_amount: i32,
    #[serde(rename = "refund_amount", default)]
    pub refund_amount: i32,
}
