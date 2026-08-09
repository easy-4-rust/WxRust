//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayRefundOrderResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayRefundOrderResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "refund_order_id", default)]
    pub refund_order_id: String,
    #[serde(rename = "refund_wx_order_id", default)]
    pub refund_wx_order_id: String,
    #[serde(rename = "pay_order_id", default)]
    pub pay_order_id: String,
    #[serde(rename = "pay_wx_order_id", default)]
    pub pay_wx_order_id: String,
}

impl WxMaXPayRefundOrderResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayRefundOrderResponse 序列化失败: {e}"))
    }
}
