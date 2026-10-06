//! 对应 Java `com.binarywang.wxjava.store.bean.delivery.DeliverySendParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::DeliveryInfo;

/// 微信小店 DeliverySendParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.delivery.DeliverySendParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliverySendParam {
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "delivery_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_list: Option<Vec<DeliveryInfo>>,
}
