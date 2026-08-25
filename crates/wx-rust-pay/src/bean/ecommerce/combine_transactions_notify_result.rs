//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.CombineTransactionsNotifyResult.java`。
//!
//! 合单支付通知结果（v3 合单支付回调解密后的数据体）。

#[allow(unused_imports)]
use super::*;

/// 合单支付通知结果。
///
/// 对应 Java `CombineTransactionsNotifyResult`。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CombineTransactionsNotifyResult {
    /// 合单商户号。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "combine_mchid"
    )]
    pub combine_mchid: Option<String>,
    /// 合单商户订单号。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "combine_out_trade_no"
    )]
    pub combine_out_trade_no: Option<String>,
    /// 场景信息。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "scene_info"
    )]
    pub scene_info: Option<SceneInfo>,
    /// 子单信息列表。
    #[serde(default, rename = "sub_orders")]
    pub sub_orders: Vec<SubOrder>,
    /// 支付者信息。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "combine_payer_info"
    )]
    pub combine_payer_info: Option<CombinePayerInfo>,
}

/// 合单支付通知中的场景信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SceneInfo {
    /// 设备 ID。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "device_id")]
    pub device_id: Option<String>,
}

/// 合单支付通知中的子单信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubOrder {
    /// 子单商户号。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
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
    /// 子单商户订单号。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    /// 金额信息。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<SubOrderAmount>,
    /// 场景信息。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "scene_info"
    )]
    pub scene_info: Option<SceneInfo>,
    /// 优惠功能。
    #[serde(default, rename = "promotion_detail")]
    pub promotion_detail: Vec<PromotionDetail>,
}

/// 子单金额信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubOrderAmount {
    /// 订单总金额。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_amount"
    )]
    pub total_amount: Option<i32>,
    /// 付款金额。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_amount"
    )]
    pub payer_amount: Option<i32>,
    /// 货币类型。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currency")]
    pub currency: Option<String>,
}

/// 合单支付通知中的支付者信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CombinePayerInfo {
    /// 用户标识。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    /// 子商户用户标识。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_openid"
    )]
    pub sub_openid: Option<String>,
}

/// 优惠功能。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromotionDetail {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let result = CombineTransactionsNotifyResult {
            combine_mchid: Some("1230000109".to_string()),
            combine_out_trade_no: Some("P20150806125346".to_string()),
            scene_info: Some(SceneInfo {
                device_id: Some("POS12345".to_string()),
            }),
            sub_orders: vec![SubOrder {
                mchid: Some("1230000109".to_string()),
                trade_state: Some("SUCCESS".to_string()),
                trade_type: Some("MICROPAY".to_string()),
                bank_type: Some("CMC".to_string()),
                attach: Some("深圳分店".to_string()),
                success_time: Some("2015-08-06T12:53:46+08:00".to_string()),
                transaction_id: Some("1008450740201411110052686910".to_string()),
                out_trade_no: Some("20150806125346".to_string()),
                amount: Some(SubOrderAmount {
                    total_amount: Some(100),
                    payer_amount: Some(100),
                    currency: Some("CNY".to_string()),
                }),
                scene_info: None,
                promotion_detail: vec![],
            }],
            combine_payer_info: Some(CombinePayerInfo {
                openid: Some("oUpF8uMuAJO_M2pxb1Q9zNjWeS6o".to_string()),
                sub_openid: None,
            }),
        };
        let json = serde_json::to_string(&result).unwrap();
        let parsed: CombineTransactionsNotifyResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result, parsed);
    }
}
