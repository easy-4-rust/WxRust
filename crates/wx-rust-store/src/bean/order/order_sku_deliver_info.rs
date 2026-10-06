//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderSkuDeliverInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderSkuDeliverInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderSkuDeliverInfo.java。
pub struct OrderSkuDeliverInfo {
    #[serde(rename = "stock_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_type: Option<i32>,
    #[serde(rename = "predict_delivery_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predict_delivery_time: Option<String>,
}
