//! 对应 Java `com.binarywang.wxjava.store.bean.warehouse.WarehouseLocationParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 WarehouseLocationParam；对应 Java com.binarywang.wxjava.store.bean.warehouse.WarehouseLocationParam.java。
pub struct WarehouseLocationParam {
    #[serde(rename = "address_id1", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id1: Option<i32>,
    #[serde(rename = "address_id2", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id2: Option<i32>,
    #[serde(rename = "address_id3", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id3: Option<i32>,
    #[serde(rename = "address_id4", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id4: Option<i32>,
}
