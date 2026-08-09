//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderProductExtraService.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderProductExtraService {
    #[serde(rename = "seven_day_return", default)]
    pub seven_day_return: i32,
    #[serde(rename = "freight_insurance", default)]
    pub freight_insurance: i32,
}
