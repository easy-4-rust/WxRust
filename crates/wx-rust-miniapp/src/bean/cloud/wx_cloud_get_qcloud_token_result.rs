//! 对应 Java `cn.binarywang.wx.miniapp.bean.cloud.WxCloudGetQcloudTokenResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCloudGetQcloudTokenResult {
    #[serde(rename = "secretid", default)]
    pub secret_id: String,
    #[serde(rename = "secretkey", default)]
    pub secret_key: String,
    #[serde(rename = "token", default)]
    pub token: String,
    #[serde(rename = "expired_time", default)]
    pub expired_time: i64,
}
