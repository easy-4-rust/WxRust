//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.SpuBaseData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpuBaseData {
    #[serde(rename = "src_spu_id", default)]
    pub src_spu_id: String,
    #[serde(rename = "src", default)]
    pub src: i64,
    #[serde(rename = "spu_name", default)]
    pub spu_name: String,
    #[serde(rename = "spu_id", default)]
    pub spu_id: i64,
    #[serde(rename = "thumb_url", default)]
    pub thumb_url: String,
    #[serde(rename = "price", default)]
    pub price: i64,
    #[serde(rename = "src_name", default)]
    pub src_name: String,
    #[serde(rename = "stock", default)]
    pub stock: i64,
}
