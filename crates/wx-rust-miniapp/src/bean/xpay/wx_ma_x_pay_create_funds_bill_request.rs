//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayCreateFundsBillRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayCreateFundsBillRequest {
    #[serde(rename = "transfer_amount", default)]
    pub transfer_amount: i32,
    #[serde(rename = "transfer_account_uid", default)]
    pub transfer_account_uid: i64,
    #[serde(rename = "transfer_account_name", default)]
    pub transfer_account_name: String,
    #[serde(rename = "transfer_account_agency_id", default)]
    pub transfer_account_agency_id: i32,
    #[serde(rename = "request_id", default)]
    pub request_id: String,
    #[serde(rename = "settle_begin", default)]
    pub settle_begin: i64,
    #[serde(rename = "settle_end", default)]
    pub settle_end: i64,
    #[serde(rename = "env", default)]
    pub env: i32,
    #[serde(rename = "authorize_advertise", default)]
    pub authorize_advertise: i32,
    #[serde(rename = "fund_type", default)]
    pub fund_type: i32,
}

impl WxMaXPayCreateFundsBillRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayCreateFundsBillRequest 序列化失败: {e}"))
    }
}
