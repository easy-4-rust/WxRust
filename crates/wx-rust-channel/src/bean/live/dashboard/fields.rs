//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.Fields.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fields {
    #[serde(rename = "dim_key", default)]
    pub dim_key: String,
    #[serde(rename = "dim_val", default)]
    pub dim_val: String,
    #[serde(rename = "dim_val_ratio", default)]
    pub dim_val_ratio: String,
}
