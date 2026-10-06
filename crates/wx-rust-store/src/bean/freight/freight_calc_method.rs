//! 对应 Java `com.binarywang.wxjava.store.bean.freight.FreightCalcMethod.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

/// 微信小店 FreightCalcMethod 数据类型；对应 Java com.binarywang.wxjava.store.bean.freight.FreightCalcMethod.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FreightCalcMethod {
    #[serde(rename = "address_infos", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_infos: Option<Vec<AddressInfo>>,
    #[serde(rename = "is_default", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "delivery_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    #[serde(rename = "first_val_amount", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_val_amount: Option<i32>,
    #[serde(rename = "first_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_price: Option<i32>,
    #[serde(rename = "second_val_amount", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_val_amount: Option<i32>,
    #[serde(rename = "second_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_price: Option<i32>,
}
