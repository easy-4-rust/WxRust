//! 对应 Java `com.binarywang.wxjava.store.bean.freight.ConditionFreeDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

/// 微信小店 ConditionFreeDetail 数据类型；对应 Java com.binarywang.wxjava.store.bean.freight.ConditionFreeDetail.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConditionFreeDetail {
    #[serde(rename = "address_infos", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_infos: Option<Vec<AddressInfo>>,
    #[serde(rename = "min_piece", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_piece: Option<i32>,
    #[serde(rename = "min_weight", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_weight: Option<f64>,
    #[serde(rename = "min_amount", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_amount: Option<i32>,
    #[serde(rename = "valuation_flag", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valuation_flag: Option<i32>,
    #[serde(rename = "amount_flag", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_flag: Option<i32>,
}
