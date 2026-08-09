//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayQueryOrderRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryOrderRequest {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "env", default)]
    pub env: i32,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "wx_order_id", default)]
    pub wx_order_id: String,
}

impl WxMaXPayQueryOrderRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryOrderRequest 序列化失败: {e}"))
    }
}
