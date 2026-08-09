//! 对应 Java `me.chanjar.weixin.channel.bean.freight.FreightCalcMethod.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FreightCalcMethod {
    #[serde(rename = "address_infos", default)]
    pub address_infos: Vec<AddressInfo>,
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "first_val_amount", default)]
    pub first_val_amount: i32,
    #[serde(rename = "first_price", default)]
    pub first_price: i32,
    #[serde(rename = "second_val_amount", default)]
    pub second_val_amount: i32,
    #[serde(rename = "second_price", default)]
    pub second_price: i32,
}
