//! 对应 Java `me.chanjar.weixin.channel.bean.warehouse.Warehouse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Warehouse {
    #[serde(rename = "out_warehouse_id", default)]
    pub out_warehouse_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "intro", default)]
    pub intro: String,
    #[serde(rename = "cover_locations", default)]
    pub cover_locations: Vec<WarehouseLocation>,
}
