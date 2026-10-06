//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleExchangeDeliveryInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

/// 微信小店 AfterSaleExchangeDeliveryInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleExchangeDeliveryInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleExchangeDeliveryInfo {
    #[serde(rename = "waybill_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waybill_id: Option<String>,
    #[serde(rename = "delivery_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    #[serde(rename = "delivery_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_name: Option<String>,
    #[serde(rename = "address_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_info: Option<AddressInfo>,
}
