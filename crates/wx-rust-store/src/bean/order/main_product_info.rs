//! 对应 Java `com.binarywang.wxjava.store.bean.order.MainProductInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 MainProductInfo；对应 Java com.binarywang.wxjava.store.bean.order.MainProductInfo.java。
pub struct MainProductInfo {
    #[serde(rename = "gift_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_cnt: Option<i32>,
    #[serde(rename = "task_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i32>,
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<i32>,
}
