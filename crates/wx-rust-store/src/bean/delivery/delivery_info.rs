//! 对应 Java `com.binarywang.wxjava.store.bean.delivery.DeliveryInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::FreightProductInfo;

/// 微信小店 DeliveryInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.delivery.DeliveryInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryInfo {
    #[serde(rename = "waybill_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waybill_id: Option<String>,
    #[serde(rename = "delivery_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    #[serde(rename = "deliver_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_type: Option<i32>,
    #[serde(rename = "product_infos", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_infos: Option<Vec<FreightProductInfo>>,
}
