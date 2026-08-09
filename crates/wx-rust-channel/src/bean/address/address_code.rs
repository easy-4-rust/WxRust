//! 对应 Java `me.chanjar.weixin.channel.bean.address.AddressCode.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddressCode {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "code", default)]
    pub code: i32,
    #[serde(rename = "level", default)]
    pub level: i32,
}
