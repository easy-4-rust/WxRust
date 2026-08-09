//! 对应 Java `me.chanjar.weixin.cp.bean.license.WxCpTpLicenseTransfer.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpLicenseTransfer {
    #[serde(rename = "handover_userid", default)]
    pub handover_user_id: String,
    #[serde(rename = "takeover_userid", default)]
    pub takeover_user_id: String,
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
}
