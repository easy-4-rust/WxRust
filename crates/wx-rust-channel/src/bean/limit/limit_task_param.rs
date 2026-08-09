//! 对应 Java `me.chanjar.weixin.channel.bean.limit.LimitTaskParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitTaskParam {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "start_time", default)]
    pub start_time: String,
    #[serde(rename = "end_time", default)]
    pub end_time: String,
    #[serde(rename = "limited_discount_skus", default)]
    pub skus: Vec<LimitSku>,
}
