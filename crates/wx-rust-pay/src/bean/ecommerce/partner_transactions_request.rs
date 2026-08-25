//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.PartnerTransactionsRequest.java`。
//!
//! 电商平台（子商户）下单请求（v3 电商收付通-支付-合单下单）。

#[allow(unused_imports)]
use super::*;

/// 电商平台（子商户）下单请求。
///
/// 对应 Java `PartnerTransactionsRequest`。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartnerTransactionsRequest {
    /// 公众号 ID。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sp_appid")]
    pub sp_appid: Option<String>,
    /// 服务商户号。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sp_mchid")]
    pub sp_mchid: Option<String>,
    /// 子商户公众号 ID。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_appid: Option<String>,
    /// 子商户号。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    /// 商户订单号。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    /// 交易类型。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trade_type"
    )]
    pub trade_type: Option<String>,
    /// 交易描述。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "description"
    )]
    pub description: Option<String>,
    /// 交易结束时间。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "time_expire"
    )]
    pub time_expire: Option<String>,
    /// 附加数据。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
    /// 通知地址。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "notify_url"
    )]
    pub notify_url: Option<String>,
    /// 订单金额。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<PartnerAmount>,
    /// 场景信息。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "scene_info"
    )]
    pub scene_info: Option<PartnerSceneInfo>,
    /// 结算信息。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settle_info"
    )]
    pub settle_info: Option<PartnerSettleInfo>,
}

/// 订单金额。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartnerAmount {
    /// 总金额。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total")]
    pub total: Option<i32>,
    /// 货币类型。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currency")]
    pub currency: Option<String>,
}

/// 场景信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartnerSceneInfo {
    /// 终端 IP。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_client_ip"
    )]
    pub payer_client_ip: Option<String>,
    /// 设备 ID。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "device_id")]
    pub device_id: Option<String>,
}

/// 结算信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartnerSettleInfo {
    /// 是否指定分账。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "profit_sharing"
    )]
    pub profit_sharing: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let req = PartnerTransactionsRequest {
            sp_appid: Some("wx8888888888888888".to_string()),
            sp_mchid: Some("1230000109".to_string()),
            sub_appid: Some("wxd678efh567hg6787".to_string()),
            sub_mchid: Some("1900000109".to_string()),
            out_trade_no: Some("P20150806125346".to_string()),
            trade_type: Some("JSAPI".to_string()),
            description: Some("Image形象店-深圳腾大-QQ公仔".to_string()),
            time_expire: None,
            attach: None,
            notify_url: Some("https://www.weixin.qq.com/wxpay/pay.php".to_string()),
            amount: Some(PartnerAmount {
                total: Some(100),
                currency: Some("CNY".to_string()),
            }),
            scene_info: Some(PartnerSceneInfo {
                payer_client_ip: Some("14.23.150.211".to_string()),
                device_id: None,
            }),
            settle_info: Some(PartnerSettleInfo {
                profit_sharing: Some(false),
            }),
        };
        let json = serde_json::to_string(&req).unwrap();
        let parsed: PartnerTransactionsRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, parsed);
    }
}
