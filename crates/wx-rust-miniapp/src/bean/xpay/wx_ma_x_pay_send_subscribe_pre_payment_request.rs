//! 预通知扣款请求。
//!
//! 对应官方文档 `developers.weixin.qq.com` 虚拟支付
//! `send_subscribe_pre_payment`（2026-09 更新，超出 WxJava 4.8.6 覆盖范围的新增接口）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPaySendSubscribePrePaymentRequest {
    /// 用户的openid
    #[serde(rename = "openid", default)]
    pub openid: String,

    /// 金额/分，属于 [100，道具价格]
    #[serde(rename = "deduct_price", default)]
    pub deduct_price: i64,

    /// 道具 id，需为订阅制道具
    #[serde(rename = "product_id", default)]
    pub product_id: String,

    /// 签约时传入的协议号
    #[serde(rename = "out_contract_code", default)]
    pub out_contract_code: String,
}

impl WxMaXPaySendSubscribePrePaymentRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPaySendSubscribePrePaymentRequest 序列化失败: {e}"))
    }
}
