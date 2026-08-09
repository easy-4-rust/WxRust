//! 对应 Java `me.chanjar.weixin.channel.bean.address.AddressIdParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddressIdParam {
    #[serde(rename = "address_id", default)]
    pub address_id: String,
}
