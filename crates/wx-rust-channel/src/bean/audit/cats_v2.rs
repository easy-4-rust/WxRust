//! 对应 Java `me.chanjar.weixin.channel.bean.audit.CatsV2.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CatsV2 {
    #[serde(rename = "cat_id", default)]
    pub cat_id: String,
}
