//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaGetUserNotifyResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaGetUserNotifyResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "notify_info", default)]
    pub notify_info: NotifyInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyInfo {
    #[serde(rename = "notify_type", default)]
    pub notify_type: i32,
    #[serde(rename = "content_json", default)]
    pub content_json: String,
    #[serde(rename = "code_state", default)]
    pub code_state: i32,
    #[serde(rename = "code_expire_time", default)]
    pub code_expire_time: i64,
}
