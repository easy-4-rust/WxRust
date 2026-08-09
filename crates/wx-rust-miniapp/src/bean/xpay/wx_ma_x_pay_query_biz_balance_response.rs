//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayQueryBizBalanceResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryBizBalanceResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "balance_available", default)]
    pub balance_available: BalanceAvailable,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BalanceAvailable {
    #[serde(rename = "amount", default)]
    pub amount: String,
    #[serde(rename = "currency_code", default)]
    pub currency_code: String,
}

impl WxMaXPayQueryBizBalanceResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryBizBalanceResponse 序列化失败: {e}"))
    }
}
