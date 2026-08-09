//! 对应 Java `me.chanjar.weixin.cp.bean.license.WxCpTpLicenseActiveAccount.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpLicenseActiveAccount {
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "active_code", default)]
    pub active_code: String,
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
}
