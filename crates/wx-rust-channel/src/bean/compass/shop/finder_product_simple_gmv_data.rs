//! 对应 Java `me.chanjar.weixin.channel.bean.compass.shop.FinderProductSimpleGmvData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinderProductSimpleGmvData {
    #[serde(rename = "commission_ratio", default)]
    pub commission_ratio: f64,
    #[serde(rename = "pay_gmv", default)]
    pub pay_gmv: String,
}
