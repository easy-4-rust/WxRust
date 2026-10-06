//! 对应 Java `com.binarywang.wxjava.store.bean.warehouse.UpdateLocationParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::WarehouseLocation;

/// 微信小店 UpdateLocationParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.warehouse.UpdateLocationParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateLocationParam {
    #[serde(rename = "out_warehouse_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_warehouse_id: Option<String>,
    #[serde(rename = "cover_locations", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_locations: Option<Vec<WarehouseLocation>>,
}
