//! 对应 Java `me.chanjar.weixin.channel.bean.coupon.CouponDetailInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CouponDetailInfo {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "valid_info", default)]
    pub valid_info: ValidInfo,
    #[serde(rename = "promote_info", default)]
    pub promote_info: PromoteInfo,
    #[serde(rename = "discount_info", default)]
    pub discount_info: DiscountInfo,
    #[serde(rename = "ext_info", default)]
    pub ext_info: ExtInfo,
    #[serde(rename = "receive_info", default)]
    pub receive_info: ReceiveInfo,
}
