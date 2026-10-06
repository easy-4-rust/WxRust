//! 对应 Java `com.binarywang.wxjava.store.bean.product.SkuDeliverInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 SkuDeliverInfo；对应 Java com.binarywang.wxjava.store.bean.product.SkuDeliverInfo.java。
pub struct SkuDeliverInfo {
    #[serde(rename = "stock_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_type: Option<i32>,
    #[serde(rename = "full_payment_presale_delivery_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_payment_presale_delivery_type: Option<i32>,
    #[serde(rename = "presale_begin_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presale_begin_time: Option<i64>,
    #[serde(rename = "presale_end_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presale_end_time: Option<i64>,
    #[serde(rename = "full_payment_presale_delivery_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_payment_presale_delivery_time: Option<i32>,
}
