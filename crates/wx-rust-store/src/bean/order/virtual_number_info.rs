//! 对应 Java `com.binarywang.wxjava.store.bean.order.VirtualNumberInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 VirtualNumberInfo；对应 Java com.binarywang.wxjava.store.bean.order.VirtualNumberInfo.java。
pub struct VirtualNumberInfo {
    #[serde(rename = "virtual_number", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_number: Option<String>,
    #[serde(rename = "extension", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(rename = "expiration", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
}
