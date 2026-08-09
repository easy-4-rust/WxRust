//! 对应 Java `me.chanjar.weixin.channel.bean.after.AfterSaleVirtualNumberInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleVirtualNumberInfo {
    #[serde(rename = "virtual_tel_number", default)]
    pub virtual_tel_number: String,
    #[serde(rename = "virtual_tel_expire_time", default)]
    pub virtual_tel_expire_time: i64,
}
