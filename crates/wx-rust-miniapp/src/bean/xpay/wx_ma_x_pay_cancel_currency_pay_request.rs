//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayCancelCurrencyPayRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayCancelCurrencyPayRequest {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "env", default)]
    pub env: i32,
    #[serde(rename = "user_ip", default)]
    pub user_ip: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "pay_order_id", default)]
    pub pay_order_id: String,
    #[serde(rename = "amount", default)]
    pub amount: i64,
    #[serde(rename = "device_type", default)]
    pub device_type: i32,
}

impl WxMaXPayCancelCurrencyPayRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayCancelCurrencyPayRequest 序列化失败: {e}"))
    }
}
