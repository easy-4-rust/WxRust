//! 对应 Java `me.chanjar.weixin.cp.bean.license.WxCpTpLicenseActiveCodeInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpLicenseActiveCodeInfo {
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "active_code", default)]
    pub active_code: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "active_time", default)]
    pub active_time: i64,
    #[serde(rename = "expire_time", default)]
    pub expire_time: i64,
}
