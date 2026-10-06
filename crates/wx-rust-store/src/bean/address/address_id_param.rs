//! 对应 Java `com.binarywang.wxjava.store.bean.address.AddressIdParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AddressIdParam；对应 Java com.binarywang.wxjava.store.bean.address.AddressIdParam.java。
pub struct AddressIdParam {
    #[serde(rename = "address_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
}
