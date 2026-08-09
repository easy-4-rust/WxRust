//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxMaOpenNetworkTimeout.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOpenNetworkTimeout {
    #[serde(rename = "request", default)]
    pub request: i32,
    #[serde(rename = "connectSocket", default)]
    pub connect_socket: i32,
    #[serde(rename = "uploadFile", default)]
    pub upload_file: i32,
    #[serde(rename = "downloadFile", default)]
    pub download_file: i32,
}
