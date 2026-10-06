//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderProductExtraService.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderProductExtraService；对应 Java com.binarywang.wxjava.store.bean.order.OrderProductExtraService.java。
pub struct OrderProductExtraService {
    #[serde(rename = "seven_day_return", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seven_day_return: Option<i32>,
    #[serde(rename = "freight_insurance", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_insurance: Option<i32>,
}
