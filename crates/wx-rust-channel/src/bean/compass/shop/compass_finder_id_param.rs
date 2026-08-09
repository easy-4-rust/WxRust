//! 对应 Java `me.chanjar.weixin.channel.bean.compass.shop.CompassFinderIdParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[allow(unused_imports)]
use crate::bean::compass::CompassFinderBaseParam;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompassFinderIdParam {
    #[serde(rename = "ds", default)]
    pub ds: String,
    #[serde(rename = "finder_id", default)]
    pub finder_id: String,
}
