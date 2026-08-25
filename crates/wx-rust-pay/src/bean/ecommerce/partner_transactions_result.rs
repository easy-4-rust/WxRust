//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.PartnerTransactionsResult.java`。
//!
//! 电商平台（子商户）下单返回结果。

#[allow(unused_imports)]
use super::*;

/// 电商平台（子商户）下单返回结果。
///
/// 对应 Java `PartnerTransactionsResult`。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartnerTransactionsResult {
    /// 预支付交易会话标识。
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prepay_id")]
    pub prepay_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let result = PartnerTransactionsResult {
            prepay_id: Some("wx201410272009395522657a690389285100".to_string()),
        };
        let json = serde_json::to_string(&result).unwrap();
        let parsed: PartnerTransactionsResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result, parsed);
    }
}
