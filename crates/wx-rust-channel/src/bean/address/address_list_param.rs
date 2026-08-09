//! 对应 Java `me.chanjar.weixin.channel.bean.address.AddressListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::OffsetParam;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddressListParam {
    #[serde(rename = "offset", default)]
    pub offset: i32,
    #[serde(rename = "limit", default)]
    pub limit: i32,
}
