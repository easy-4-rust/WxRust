//! 对应 Java `me.chanjar.weixin.channel.bean.coupon.ExtInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExtInfo {
    #[serde(rename = "jump_product_id", default)]
    pub jump_product_id: String,
    #[serde(rename = "notes", default)]
    pub notes: String,
    #[serde(rename = "valid_time", default)]
    pub valid_time: i64,
    #[serde(rename = "invalid_time", default)]
    pub invalid_time: i64,
}
