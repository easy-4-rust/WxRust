//! 对应 Java `me.chanjar.weixin.channel.bean.compass.finder.Overall.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Overall {
    #[serde(rename = "pay_gmv", default)]
    pub pay_gmv: String,
    #[serde(rename = "live_pay_gmv", default)]
    pub live_pay_gmv: String,
    #[serde(rename = "feed_pay_gmv", default)]
    pub feed_pay_gmv: String,
    #[serde(rename = "window_pay_gmv", default)]
    pub window_pay_gmv: String,
    #[serde(rename = "product_pay_gmv", default)]
    pub product_pay_gmv: String,
    #[serde(rename = "other_pay_gmv", default)]
    pub other_pay_gmv: String,
}
