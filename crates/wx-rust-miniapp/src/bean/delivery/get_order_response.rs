//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.GetOrderResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetOrderResponse {
    #[serde(rename = "resultcode", default)]
    pub result_code: i32,
    #[serde(rename = "resultmsg", default)]
    pub result_msg: String,
    #[serde(rename = "order_status", default)]
    pub order_status: i32,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "rider_name", default)]
    pub rider_name: String,
    #[serde(rename = "rider_phone", default)]
    pub rider_phone: String,
    #[serde(
        rename = "rider_lng",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub rider_lng: String,
    #[serde(
        rename = "rider_lat",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub rider_lat: String,
    #[serde(
        rename = "reach_time",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub reach_time: String,
}
