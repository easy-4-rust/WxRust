//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderSettleInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderSettleInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderSettleInfo.java。
pub struct OrderSettleInfo {
    #[serde(rename = "predict_commission_fee", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predict_commission_fee: Option<i32>,
    #[serde(rename = "commission_fee", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_fee: Option<i32>,
    #[serde(rename = "predict_wecoin_commission", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predict_wecoin_commission: Option<i32>,
    #[serde(rename = "wecoin_commission", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wecoin_commission: Option<i32>,
    #[serde(rename = "settle_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_time: Option<i64>,
}
