//! 对应 Java `com.binarywang.wxjava.store.bean.warehouse.StockGetParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 StockGetParam；对应 Java com.binarywang.wxjava.store.bean.warehouse.StockGetParam.java。
pub struct StockGetParam {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "out_warehouse_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_warehouse_id: Option<String>,
}
