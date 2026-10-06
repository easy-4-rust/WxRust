//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.ShopField.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ShopField；对应 Java com.binarywang.wxjava.store.bean.compass.shop.ShopField.java。
pub struct ShopField {
    #[serde(rename = "field_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "data_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_list: Option<Vec<FieldDetail>>,
}

/// 微信小店 FieldDetail 数据类型；对应 Java com.binarywang.wxjava.store.bean.compass.shop.ShopField.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FieldDetail {
    #[serde(rename = "dim_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dim_key: Option<String>,
    #[serde(rename = "dim_value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dim_value: Option<String>,
}
