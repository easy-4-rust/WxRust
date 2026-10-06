//! 对应 Java `com.binarywang.wxjava.store.bean.address.AddressCode.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AddressCode；对应 Java com.binarywang.wxjava.store.bean.address.AddressCode.java。
pub struct AddressCode {
    #[serde(rename = "name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "level", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
}
