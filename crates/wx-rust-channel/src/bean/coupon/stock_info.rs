//! 对应 Java `me.chanjar.weixin.channel.bean.coupon.StockInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StockInfo {
    #[serde(rename = "issued_num", default)]
    pub issued_num: i32,
    #[serde(rename = "receive_num", default)]
    pub receive_num: i32,
    #[serde(rename = "used_num", default)]
    pub used_num: i32,
}
