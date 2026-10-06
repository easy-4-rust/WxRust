//! 对应 Java `com.binarywang.wxjava.store.bean.warehouse.WarehouseStockParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::product::SkuStockParam;

/// 微信小店 WarehouseStockParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.warehouse.WarehouseStockParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WarehouseStockParam {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "diff_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_type: Option<i32>,
    #[serde(rename = "num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<i32>,
    #[serde(rename = "out_warehouse_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_warehouse_id: Option<String>,
}
