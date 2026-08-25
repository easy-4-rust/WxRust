//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.PartnerTransactionsNotifyResult.java`。
//!
//! 电商平台（子商户）支付通知结果（v3 电商收付通-支付-支付通知回调解密后的数据体）。

#[allow(unused_imports)]
use super::*;

/// 电商平台（子商户）支付通知结果。
///
/// 对应 Java `PartnerTransactionsNotifyResult`。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartnerTransactionsNotifyResult {
    /// 服务商商户号。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sp_mchid")]
    pub sp_mchid: Option<String>,
    /// 子商户号。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    /// 交易状态。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trade_state"
    )]
    pub trade_state: Option<String>,
    /// 交易类型。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trade_type"
    )]
    pub trade_type: Option<String>,
    /// 付款银行。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bank_type")]
    pub bank_type: Option<String>,
    /// 附加数据。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
    /// 支付完成时间。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "success_time"
    )]
    pub success_time: Option<String>,
    /// 微信支付订单号。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    /// 商户订单号。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    /// 订单金额。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<NotifyAmount>,
    /// 场景信息。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "scene_info"
    )]
    pub scene_info: Option<NotifySceneInfo>,
    /// 支付者信息。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "payer")]
    pub payer: Option<NotifyPayer>,
    /// 优惠功能。
    #[serde(default, rename = "promotion_detail")]
    pub promotion_detail: Vec<NotifyPromotionDetail>,
}

/// 通知金额信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyAmount {
    /// 总金额。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total")]
    pub total: Option<i32>,
    /// 付款金额。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_total"
    )]
    pub payer_total: Option<i32>,
    /// 货币类型。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currency")]
    pub currency: Option<String>,
    /// 付款币种。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_currency"
    )]
    pub payer_currency: Option<String>,
}

/// 通知场景信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifySceneInfo {
    /// 设备 ID。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "device_id")]
    pub device_id: Option<String>,
}

/// 通知支付者信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyPayer {
    /// 用户标识。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sp_openid")]
    pub sp_openid: Option<String>,
    /// 子商户用户标识。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_openid"
    )]
    pub sub_openid: Option<String>,
}

/// 通知优惠信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyPromotionDetail {
    /// 券 ID。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "coupon_id")]
    pub coupon_id: Option<String>,
    /// 优惠名称。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "name")]
    pub name: Option<String>,
    /// 优惠范围。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scope")]
    pub scope: Option<String>,
    /// 优惠类型。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// 优惠券面额。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<i32>,
    /// 活动 ID。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stock_id")]
    pub stock_id: Option<String>,
    /// 微信出资。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "wechatpay_contribute"
    )]
    pub wechatpay_contribute: Option<i32>,
    /// 商户出资。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_contribute"
    )]
    pub merchant_contribute: Option<i32>,
    /// 其他出资。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "other_contribute"
    )]
    pub other_contribute: Option<i32>,
    /// 优惠币种。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currency")]
    pub currency: Option<String>,
}
