//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.ShopOverall.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ShopOverall；对应 Java com.binarywang.wxjava.store.bean.compass.shop.ShopOverall.java。
pub struct ShopOverall {
    #[serde(rename = "pay_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_gmv: Option<String>,
    #[serde(rename = "pay_uv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_uv: Option<String>,
    #[serde(rename = "pay_refund_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_refund_gmv: Option<String>,
    #[serde(rename = "pay_order_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_order_cnt: Option<String>,
    #[serde(rename = "live_pay_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_pay_gmv: Option<String>,
    #[serde(rename = "feed_pay_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_pay_gmv: Option<String>,
}
