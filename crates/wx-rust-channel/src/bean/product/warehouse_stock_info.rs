//! 对应 Java `me.chanjar.weixin.channel.bean.product.WarehouseStockInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WarehouseStockInfo {
    #[serde(rename = "out_warehouse_id", default)]
    pub out_warehouse_id: String,
    #[serde(rename = "num", default)]
    pub num: i32,
    #[serde(rename = "lock_stock", default)]
    pub lock_stock: i32,
}
