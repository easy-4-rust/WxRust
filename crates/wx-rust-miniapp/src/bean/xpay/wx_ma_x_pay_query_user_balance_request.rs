//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayQueryUserBalanceRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryUserBalanceRequest {
    #[serde(rename = "user_ip", default)]
    pub user_ip: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "env", default)]
    pub env: i32,
}

impl WxMaXPayQueryUserBalanceRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryUserBalanceRequest 序列化失败: {e}"))
    }
}
