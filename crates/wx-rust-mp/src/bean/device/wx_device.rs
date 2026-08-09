//! 对应 Java `bean.device.WxDevice`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxDevice {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "mac", default)]
    pub mac: String,
    #[serde(rename = "connect_protocol", default)]
    pub connect_protocol: String,
    #[serde(rename = "auth_key", default)]
    pub auth_key: String,
    #[serde(rename = "close_strategy", default)]
    pub close_strategy: String,
    #[serde(rename = "conn_strategy", default)]
    pub conn_strategy: String,
    #[serde(rename = "crypt_method", default)]
    pub crypt_method: String,
    #[serde(rename = "auth_ver", default)]
    pub auth_ver: String,
    #[serde(rename = "manu_mac_pos", default)]
    pub manu_mac_pos: String,
    #[serde(rename = "ser_mac_pos", default)]
    pub ser_mac_pos: String,
    #[serde(rename = "ble_simple_protocol", default)]
    pub ble_simple_protocol: String,
}
