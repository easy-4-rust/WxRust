//! 对应 Java `me.chanjar.weixin.channel.bean.cooperation.CooperationSharerParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CooperationSharerParam {
    #[serde(rename = "sharer_id", default)]
    pub sharer_id: String,
    #[serde(rename = "sharer_type", default)]
    pub sharer_type: i32,
}
