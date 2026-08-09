//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayQueryFundsBillResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryFundsBillResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "bill_list", default)]
    pub bill_list: Vec<Bill>,
    #[serde(rename = "total_page", default)]
    pub total_page: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Bill {
    #[serde(rename = "bill_id", default)]
    pub bill_id: String,
    #[serde(rename = "oper_time", default)]
    pub oper_time: i64,
    #[serde(rename = "settle_begin", default)]
    pub settle_begin: i64,
    #[serde(rename = "settle_end", default)]
    pub settle_end: i64,
    #[serde(rename = "fund_id", default)]
    pub fund_id: String,
    #[serde(rename = "transfer_account_name", default)]
    pub transfer_account_name: String,
    #[serde(rename = "transfer_account_uid", default)]
    pub transfer_account_uid: i32,
    #[serde(rename = "transfer_amount", default)]
    pub transfer_amount: i32,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "request_id", default)]
    pub request_id: String,
}

impl WxMaXPayQueryFundsBillResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryFundsBillResponse 序列化失败: {e}"))
    }
}
