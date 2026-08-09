//! 对应 Java `me.chanjar.weixin.channel.bean.address.AddressDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddressDetail {
    #[serde(rename = "address_id", default)]
    pub address_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "address_info", default)]
    pub address_info: AddressInfo,
    #[serde(rename = "landline", default)]
    pub landline: String,
    #[serde(rename = "send_addr", default)]
    pub send_addr: bool,
    #[serde(rename = "recv_addr", default)]
    pub recv_addr: bool,
    #[serde(rename = "default_send", default)]
    pub default_send: bool,
    #[serde(rename = "default_recv", default)]
    pub default_recv: bool,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "address_type", default)]
    pub address_type: OfflineAddressType,
}
