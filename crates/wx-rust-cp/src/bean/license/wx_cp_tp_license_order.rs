//! 对应 Java `me.chanjar.weixin.cp.bean.license.WxCpTpLicenseOrder.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpLicenseOrder {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "order_type", default)]
    pub order_type: i32,
    #[serde(rename = "order_status", default)]
    pub order_status: i32,
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
    #[serde(rename = "price", default)]
    pub price: i64,
    #[serde(rename = "account_count", default)]
    pub account_count:
        crate::bean::license::wx_cp_tp_license_account_count::WxCpTpLicenseAccountCount,
    #[serde(rename = "account_duration", default)]
    pub account_duration:
        crate::bean::license::wx_cp_tp_license_account_duration::WxCpTpLicenseAccountDuration,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "pay_time", default)]
    pub pay_time: i64,
}

impl WxCpTpLicenseOrder {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTpLicenseOrder 序列化失败: {e}"))
    }
}
