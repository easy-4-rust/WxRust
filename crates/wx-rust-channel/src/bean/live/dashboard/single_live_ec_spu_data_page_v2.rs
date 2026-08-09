//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.SingleLiveEcSpuDataPageV2.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SingleLiveEcSpuDataPageV2 {
    #[serde(rename = "spu_data_list", default)]
    pub spu_data_list: Vec<SpuData>,
    #[serde(rename = "total_cnt", default)]
    pub total_cnt: i32,
    #[serde(rename = "data_key", default)]
    pub data_key: Vec<String>,
}
