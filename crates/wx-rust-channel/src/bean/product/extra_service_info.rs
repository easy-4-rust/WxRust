//! 对应 Java `me.chanjar.weixin.channel.bean.product.ExtraServiceInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExtraServiceInfo {
    #[serde(rename = "seven_day_return", default)]
    pub seven_day_return: i32,
    #[serde(rename = "pay_after_use", default)]
    pub pay_after_use: i32,
    #[serde(rename = "freight_insurance", default)]
    pub freight_insurance: i32,
    #[serde(rename = "fake_one_pay_three", default)]
    pub fake_one_pay_three: i32,
    #[serde(rename = "damage_guarantee", default)]
    pub damage_guarantee: i32,
}
