//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.PartnerTransactionsCloseRequest.java`。
//!
//! 电商平台（子商户）关单请求。

#[allow(unused_imports)]
use super::*;

/// 电商平台（子商户）关单请求。
///
/// 对应 Java `PartnerTransactionsCloseRequest`。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartnerTransactionsCloseRequest {
    /// 服务商户号。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sp_mchid")]
    pub sp_mchid: Option<String>,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let req = PartnerTransactionsCloseRequest {
            sp_mchid: Some("1230000109".to_string()),
            sub_mchid: Some("1900000109".to_string()),
            out_trade_no: Some("P20150806125346".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        let parsed: PartnerTransactionsCloseRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, parsed);
    }
}
