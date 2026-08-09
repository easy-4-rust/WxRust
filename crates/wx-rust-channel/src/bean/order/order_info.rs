//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderInfo {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "unionid", default)]
    pub unionid: String,
    #[serde(rename = "order_detail", default)]
    pub order_detail: OrderDetailInfo,
    #[serde(rename = "aftersale_detail", default)]
    pub after_sale_detail: AfterSaleDetail,
    #[serde(rename = "is_present", default)]
    pub present: bool,
    #[serde(rename = "present_order_id_str", default)]
    pub present_order_id: String,
    #[serde(rename = "present_note", default)]
    pub present_note: String,
    #[serde(rename = "present_giver_openid", default)]
    pub present_giver_openid: String,
    #[serde(rename = "present_giver_unionid", default)]
    pub present_giver_unionid: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i32,
    #[serde(rename = "update_time", default)]
    pub update_time: i32,
}
