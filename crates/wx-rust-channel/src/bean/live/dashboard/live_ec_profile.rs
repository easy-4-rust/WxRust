//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveEcProfile.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveEcProfile {
    #[serde(rename = "profiles", default)]
    pub profiles: Vec<DataNodeThirdList>,
    #[serde(rename = "totals", default)]
    pub totals: Vec<DataNodeList>,
}
