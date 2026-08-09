//! 对应 Java `me.chanjar.weixin.channel.bean.freight.FreightTemplate.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FreightTemplate {
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "valuation_type", default)]
    pub valuation_type: String,
    #[serde(rename = "send_time", default)]
    pub send_time: String,
    #[serde(rename = "address_info", default)]
    pub address_info: AddressInfo,
    #[serde(rename = "delivery_type", default)]
    pub delivery_type: String,
    #[serde(rename = "shipping_method", default)]
    pub shipping_method: String,
    #[serde(rename = "all_condition_free_detail", default)]
    pub all_condition_free_detail: AllConditionFreeDetail,
    #[serde(rename = "all_freight_calc_method", default)]
    pub all_freight_calc_method: AllFreightCalcMethod,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    #[serde(rename = "not_send_area", default)]
    pub not_send_area: NotSendArea,
}
