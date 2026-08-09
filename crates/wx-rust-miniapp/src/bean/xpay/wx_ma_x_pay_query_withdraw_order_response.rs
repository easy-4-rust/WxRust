//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayQueryWithdrawOrderResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryWithdrawOrderResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "withdraw_no", default)]
    pub withdraw_no: String,
    #[serde(rename = "withdraw_amount", default)]
    pub withdraw_amount: String,
    #[serde(rename = "wx_withdraw_no", default)]
    pub wx_withdraw_no: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "withdraw_success_timestamp", default)]
    pub withdraw_success_timestamp: String,
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "fail_reason", default)]
    pub fail_reason: String,
}

impl WxMaXPayQueryWithdrawOrderResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryWithdrawOrderResponse 序列化失败: {e}"))
    }
}
