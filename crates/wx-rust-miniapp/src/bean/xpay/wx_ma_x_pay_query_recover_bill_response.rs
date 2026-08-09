//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayQueryRecoverBillResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryRecoverBillResponse {
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
    #[serde(rename = "recover_time", default)]
    pub recover_time: i64,
    #[serde(rename = "settle_begin", default)]
    pub settle_begin: i64,
    #[serde(rename = "settle_end", default)]
    pub settle_end: i64,
    #[serde(rename = "fund_id", default)]
    pub fund_id: String,
    #[serde(rename = "recover_account_name", default)]
    pub recover_account_name: String,
    #[serde(rename = "recover_amount", default)]
    pub recover_amount: i32,
    #[serde(rename = "refund_order_list", default)]
    pub refund_order_list: Vec<String>,
}

impl WxMaXPayQueryRecoverBillResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryRecoverBillResponse 序列化失败: {e}"))
    }
}
