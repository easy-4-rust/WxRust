//! 对应 Java `me.chanjar.weixin.channel.bean.compass.finder.Field.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Field {
    #[serde(rename = "field_name", default)]
    pub field_name: String,
    #[serde(rename = "data_list", default)]
    pub data_list: Vec<FieldData>,
}
