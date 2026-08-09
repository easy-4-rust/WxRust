//! 对应 Java `me.chanjar.weixin.channel.bean.coupon.CouponInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CouponInfo {
    #[serde(rename = "coupon_id", default)]
    pub coupon_id: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "coupon_info", default)]
    pub detail: CouponDetailInfo,
    #[serde(rename = "stock_info", default)]
    pub stock_info: StockInfo,
}
