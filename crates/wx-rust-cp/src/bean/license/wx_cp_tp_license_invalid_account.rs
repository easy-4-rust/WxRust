//! 对应 Java `me.chanjar.weixin.cp.bean.license.WxCpTpLicenseInvalidAccount.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpLicenseInvalidAccount {
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "errcode", default)]
    pub error_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
}
