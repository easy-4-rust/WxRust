//! 对应 Java `com.binarywang.wxjava.store.bean.address.OfflineAddressType.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OfflineAddressType；对应 Java com.binarywang.wxjava.store.bean.address.OfflineAddressType.java。
pub struct OfflineAddressType {
    #[serde(rename = "same_city", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_city: Option<i32>,
    #[serde(rename = "pickup", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup: Option<i32>,
}
