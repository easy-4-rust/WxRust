//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.AddOrderResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddOrderResponse {
    #[serde(rename = "resultcode", default)]
    pub result_code: i32,
    #[serde(rename = "resultmsg", default)]
    pub result_msg: String,
    #[serde(
        rename = "fee",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub fee: String,
    #[serde(
        rename = "deliverfee",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub deliver_fee: String,
    #[serde(
        rename = "couponfee",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub coupon_fee: String,
    #[serde(
        rename = "tips",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub tips: String,
    #[serde(
        rename = "insurancfee",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub insuranc_fee: String,
    #[serde(
        rename = "distance",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub distance: String,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "order_status", default)]
    pub order_status: i32,
    #[serde(rename = "finish_code", default)]
    pub finish_code: i32,
    #[serde(rename = "pickup_code", default)]
    pub pickup_code: i32,
    #[serde(
        rename = "dispatch_duration",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub dispatch_duration: String,
}
