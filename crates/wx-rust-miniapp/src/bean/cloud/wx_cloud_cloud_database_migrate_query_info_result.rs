//! 对应 Java `cn.binarywang.wx.miniapp.bean.cloud.WxCloudCloudDatabaseMigrateQueryInfoResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCloudCloudDatabaseMigrateQueryInfoResult {
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "record_success", default)]
    pub record_success: i32,
    #[serde(rename = "record_fail", default)]
    pub record_fail: i32,
    #[serde(rename = "err_msg", default)]
    pub err_msg: String,
    #[serde(rename = "file_url", default)]
    pub file_url: String,
}
