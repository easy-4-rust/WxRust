//! 对应 Java `com.binarywang.wxjava.store.bean.product.TimingOnSaleInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TimingOnSaleInfo；对应 Java com.binarywang.wxjava.store.bean.product.TimingOnSaleInfo.java。
pub struct TimingOnSaleInfo {
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "onsale_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_sale_time: Option<i64>,
    #[serde(rename = "is_hide_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hide_price: Option<i32>,
    #[serde(rename = "task_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i32>,
}
