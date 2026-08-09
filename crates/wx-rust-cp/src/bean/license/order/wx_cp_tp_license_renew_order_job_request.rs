//! 对应 Java `me.chanjar.weixin.cp.bean.license.order.WxCpTpLicenseRenewOrderJobRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::license::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpLicenseRenewOrderJobRequest {
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
    #[serde(rename = "account_list", default)]
    pub account_list:
        Vec<crate::bean::license::wx_cp_tp_license_base_account::WxCpTpLicenseBaseAccount>,
    #[serde(rename = "jobid", default)]
    pub job_id: String,
}

impl WxCpTpLicenseRenewOrderJobRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpTpLicenseRenewOrderJobRequest 序列化失败: {e}"))
    }
}
