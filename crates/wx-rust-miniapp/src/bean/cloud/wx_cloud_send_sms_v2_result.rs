//! 对应 Java `cn.binarywang.wx.miniapp.bean.cloud.WxCloudSendSmsV2Result.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCloudSendSmsV2Result {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "send_status_list", default)]
    pub send_status_list: Vec<SendStatus>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SendStatus {
    #[serde(rename = "serial_no", default)]
    pub serial_no: String,
    #[serde(rename = "phone_number", default)]
    pub phone_number: String,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "message", default)]
    pub message: String,
    #[serde(rename = "iso_code", default)]
    pub iso_code: String,
}
