//! 对应 Java `me.chanjar.weixin.channel.bean.compass.finder.FieldData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FieldData {
    #[serde(rename = "dim_key", default)]
    pub dim_key: String,
    #[serde(rename = "dim_value", default)]
    pub dim_value: String,
}
