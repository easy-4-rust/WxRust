//! 对应 Java `com.binarywang.wxjava.store.bean.address.AddressDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::OfflineAddressType;

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

/// 微信小店 AddressDetail 数据类型；对应 Java com.binarywang.wxjava.store.bean.address.AddressDetail.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddressDetail {
    #[serde(rename = "address_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    #[serde(rename = "name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "address_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_info: Option<AddressInfo>,
    #[serde(rename = "landline", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landline: Option<String>,
    #[serde(rename = "send_addr", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_addr: Option<bool>,
    #[serde(rename = "recv_addr", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_addr: Option<bool>,
    #[serde(rename = "default_send", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_send: Option<bool>,
    #[serde(rename = "default_recv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_recv: Option<bool>,
    #[serde(rename = "create_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "update_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(rename = "address_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<OfflineAddressType>,
}
