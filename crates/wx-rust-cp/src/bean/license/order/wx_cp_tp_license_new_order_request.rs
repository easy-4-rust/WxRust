//! 对应 Java `me.chanjar.weixin.cp.bean.license.order.WxCpTpLicenseNewOrderRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::license::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpLicenseNewOrderRequest {
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
    #[serde(rename = "buyer_userid", default)]
    pub buyer_user_id: String,
    #[serde(rename = "account_count", default)]
    pub account_count:
        crate::bean::license::wx_cp_tp_license_account_count::WxCpTpLicenseAccountCount,
    #[serde(rename = "account_duration", default)]
    pub account_duration:
        crate::bean::license::wx_cp_tp_license_account_duration::WxCpTpLicenseAccountDuration,
}

impl WxCpTpLicenseNewOrderRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpTpLicenseNewOrderRequest 序列化失败: {e}"))
    }
}
