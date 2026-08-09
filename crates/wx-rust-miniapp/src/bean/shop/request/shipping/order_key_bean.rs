//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.shipping.OrderKeyBean.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::request::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderKeyBean {
    #[serde(rename = "order_number_type", default)]
    pub order_number_type: i32,
    #[serde(rename = "transaction_id", default)]
    pub transaction_id: String,
    #[serde(rename = "mchid", default)]
    pub mch_id: String,
    #[serde(rename = "out_trade_no", default)]
    pub out_trade_no: String,
}
