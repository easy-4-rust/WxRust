//! 发起订阅扣款请求。
//!
//! 对应官方文档 `developers.weixin.qq.com` 虚拟支付
//! `submit_subscribe_pay_order`（2026-09 更新，超出 WxJava 4.8.6 覆盖范围的新增接口）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPaySubmitSubscribePayOrderRequest {
    /// 用户的openid
    #[serde(rename = "openid", default)]
    pub openid: String,

    /// 在米大师侧申请的应用 id
    #[serde(rename = "offer_id", default)]
    pub offer_id: String,

    /// 购买数量，填：1
    #[serde(rename = "buy_quantity", default)]
    pub buy_quantity: i32,

    /// 环境配置，0 正式环境
    #[serde(rename = "env", default)]
    pub env: i32,

    /// 币种，填：CNY
    #[serde(rename = "currency_type", default)]
    pub currency_type: String,

    /// 订阅道具ID
    #[serde(rename = "product_id", default)]
    pub product_id: String,

    /// 扣款金额(分)，属于 [1，道具价格]
    #[serde(rename = "deduct_price", default)]
    pub deduct_price: i64,

    /// 业务订单号，8-32 字符，数字/大小写字母/_-
    #[serde(rename = "order_id", default)]
    pub order_id: String,

    /// 透传数据，发货通知时透传给开发者
    #[serde(rename = "attach", default)]
    pub attach: String,
}

impl WxMaXPaySubmitSubscribePayOrderRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPaySubmitSubscribePayOrderRequest 序列化失败: {e}"))
    }
}
