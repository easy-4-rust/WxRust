//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderPriceParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::ChangeOrderInfo;

/// 微信小店 OrderPriceParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.OrderPriceParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderPriceParam {
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "change_express", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_express: Option<bool>,
    #[serde(rename = "express_fee", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub express_fee: Option<i32>,
    #[serde(rename = "change_order_infos", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_order_infos: Option<Vec<ChangeOrderInfo>>,
}
