//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayQueryOrderResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryOrderResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "order", default)]
    pub order: OrderInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderInfo {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "biz_type", default)]
    pub biz_type: i32,
    #[serde(rename = "order_fee", default)]
    pub order_fee: i64,
    #[serde(rename = "coupon_fee", default)]
    pub coupon_fee: i64,
    #[serde(rename = "paid_fee", default)]
    pub paid_fee: i64,
    #[serde(rename = "order_type", default)]
    pub order_type: i32,
    #[serde(rename = "refund_fee", default)]
    pub refund_fee: i64,
    #[serde(rename = "paid_time", default)]
    pub paid_time: i64,
    #[serde(rename = "provide_time", default)]
    pub provide_time: i64,
    #[serde(rename = "env_type", default)]
    pub env_type: i64,
    #[serde(rename = "biz_meta", default)]
    pub biz_meta: String,
    #[serde(rename = "token", default)]
    pub token: String,
    #[serde(rename = "left_fee", default)]
    pub left_fee: i64,
    #[serde(rename = "wx_order_id", default)]
    pub wx_order_id: String,
    #[serde(rename = "channel_order_id", default)]
    pub channel_order_id: String,
    #[serde(rename = "wxpay_order_id", default)]
    pub wxpay_order_id: String,
    #[serde(rename = "sett_time", default)]
    pub sett_time: i64,
    #[serde(rename = "sett_state", default)]
    pub sett_state: i32,
    #[serde(rename = "platform_fee_fen", default)]
    pub platform_fee_fen: i64,
    #[serde(rename = "cps_fee_fen", default)]
    pub cps_fee_fen: i64,
}

impl WxMaXPayQueryOrderResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryOrderResponse 序列化失败: {e}"))
    }
}
