//! 对应 Java `me.chanjar.weixin.channel.bean.freight.ConditionFreeDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConditionFreeDetail {
    #[serde(rename = "address_infos", default)]
    pub address_infos: Vec<AddressInfo>,
    #[serde(rename = "min_piece", default)]
    pub min_piece: i32,
    #[serde(rename = "min_weight", default)]
    pub min_weight: f64,
    #[serde(rename = "min_amount", default)]
    pub min_amount: i32,
    #[serde(rename = "valuation_flag", default)]
    pub valuation_flag: i32,
    #[serde(rename = "amount_flag", default)]
    pub amount_flag: i32,
}
