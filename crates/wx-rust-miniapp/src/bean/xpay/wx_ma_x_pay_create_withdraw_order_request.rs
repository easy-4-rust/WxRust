//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayCreateWithdrawOrderRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayCreateWithdrawOrderRequest {
    #[serde(rename = "env", default)]
    pub env: i32,
    #[serde(rename = "withdraw_no", default)]
    pub withdraw_no: String,
    #[serde(rename = "withdraw_amount", default)]
    pub withdraw_amount: String,
}

impl WxMaXPayCreateWithdrawOrderRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayCreateWithdrawOrderRequest 序列化失败: {e}"))
    }
}
