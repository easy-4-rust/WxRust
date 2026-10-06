//! 对应 Java `com.binarywang.wxjava.store.bean.product.WarehouseStockInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 WarehouseStockInfo；对应 Java com.binarywang.wxjava.store.bean.product.WarehouseStockInfo.java。
pub struct WarehouseStockInfo {
    #[serde(rename = "out_warehouse_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_warehouse_id: Option<String>,
    #[serde(rename = "num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<i32>,
    #[serde(rename = "lock_stock", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_stock: Option<i32>,
}
