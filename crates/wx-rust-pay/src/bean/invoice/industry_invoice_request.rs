//! 行业电子发票请求。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.IndustryInvoiceRequest`。

use serde::{Deserialize, Serialize};

/// 行业电子发票请求。
///
/// `fapiao_information` 对应不动产租赁或成品油官方请求对象，
/// 此处使用 `serde_json::Value` 以保持灵活性。
///
/// 对应 Java: `IndustryInvoiceRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndustryInvoiceRequest {
    /// 子商户号。
    #[serde(rename = "sub_mchid", skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,

    /// 开票申请单号。
    #[serde(rename = "fapiao_apply_id", skip_serializing_if = "Option::is_none")]
    pub fapiao_apply_id: Option<String>,

    /// 购买方信息。
    #[serde(rename = "buyer_information", skip_serializing_if = "Option::is_none")]
    pub buyer_information: Option<super::buyer_information::BuyerInformation>,

    /// 发票信息（不动产租赁或成品油对象）。
    #[serde(rename = "fapiao_information", skip_serializing_if = "Option::is_none")]
    pub fapiao_information: Option<serde_json::Value>,
}
