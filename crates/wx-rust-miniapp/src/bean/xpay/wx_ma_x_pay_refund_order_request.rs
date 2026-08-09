//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayRefundOrderRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayRefundOrderRequest {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "env", default)]
    pub env: i32,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "wx_order_id", default)]
    pub wx_order_id: String,
    #[serde(rename = "refund_order_id", default)]
    pub refund_order_id: String,
    #[serde(rename = "left_fee", default)]
    pub left_fee: i64,
    #[serde(rename = "refund_fee", default)]
    pub refund_fee: i64,
    #[serde(rename = "biz_meta", default)]
    pub biz_meta: String,
    #[serde(rename = "refund_reason", default)]
    pub refund_reason: String,
    #[serde(rename = "req_from", default)]
    pub req_from: String,
}

impl WxMaXPayRefundOrderRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayRefundOrderRequest 序列化失败: {e}"))
    }
}
