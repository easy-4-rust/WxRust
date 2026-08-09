//! 对应 Java `me.chanjar.weixin.cp.bean.license.order.WxCpTpLicenseRenewOrderJobResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::license::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpLicenseRenewOrderJobResp {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "jobid", default)]
    pub job_id: String,
    #[serde(rename = "invalid_account_list", default)]
    pub invalid_account_list:
        Vec<crate::bean::license::wx_cp_tp_license_invalid_account::WxCpTpLicenseInvalidAccount>,
}

impl WxCpTpLicenseRenewOrderJobResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpTpLicenseRenewOrderJobResp 解析失败: {e}"))
    }
}
