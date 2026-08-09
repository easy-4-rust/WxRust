//! 对应 Java `me.chanjar.weixin.channel.bean.compass.shop.FinderGmvItem.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinderGmvItem {
    #[serde(rename = "finder_id", default)]
    pub finder_id: String,
    #[serde(rename = "finder_nickname", default)]
    pub finder_nickname: String,
    #[serde(rename = "data", default)]
    pub data: FinderGmvData,
}
