//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopOrderPayRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopOrderPayRequest {
    #[serde(rename = "order_id", default)]
    pub order_id: i64,
    #[serde(rename = "out_order_id", default)]
    pub out_order_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "action_type", default)]
    pub action_type: i32,
    #[serde(rename = "action_remark", default)]
    pub action_remark: String,
    #[serde(rename = "transaction_id", default)]
    pub transaction_id: String,
    #[serde(rename = "pay_time", default)]
    pub pay_time: String,
}
