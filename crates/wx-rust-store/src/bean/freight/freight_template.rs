//! 对应 Java `com.binarywang.wxjava.store.bean.freight.FreightTemplate.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{AllConditionFreeDetail, AllFreightCalcMethod, NotSendArea};

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

/// 微信小店 FreightTemplate 数据类型；对应 Java com.binarywang.wxjava.store.bean.freight.FreightTemplate.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FreightTemplate {
    #[serde(rename = "template_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "valuation_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valuation_type: Option<String>,
    #[serde(rename = "send_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_time: Option<String>,
    #[serde(rename = "address_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_info: Option<AddressInfo>,
    #[serde(rename = "delivery_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_type: Option<String>,
    #[serde(rename = "shipping_method", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_method: Option<String>,
    #[serde(rename = "all_condition_free_detail", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_condition_free_detail: Option<AllConditionFreeDetail>,
    #[serde(rename = "all_freight_calc_method", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_freight_calc_method: Option<AllFreightCalcMethod>,
    #[serde(rename = "create_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "update_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(rename = "is_default", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "not_send_area", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_send_area: Option<NotSendArea>,
}
