//! 冲红电子发票请求。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.ReverseInvoiceRequest`。

use serde::{Deserialize, Serialize};

/// 冲红电子发票请求。
///
/// 对应 Java: `ReverseInvoiceRequest`
///
/// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4015792575>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReverseInvoiceRequest {
    /// 子商户号。
    #[serde(rename = "sub_mchid", skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,

    /// 开票申请单号（仅用于构造 URL，不参与 body 序列化）。
    #[serde(rename = "fapiao_apply_id", skip_serializing_if = "Option::is_none")]
    pub fapiao_apply_id: Option<String>,

    /// 冲红原因。
    #[serde(rename = "reverse_reason", skip_serializing_if = "Option::is_none")]
    pub reverse_reason: Option<String>,

    /// 发票信息列表。
    #[serde(rename = "fapiao_information", skip_serializing_if = "Option::is_none")]
    pub fapiao_information: Option<Vec<InvoiceInfo>>,
}

/// 冲红发票信息。
///
/// 对应 Java: `ReverseInvoiceRequest.InvoiceInfo`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoiceInfo {
    /// 发票 ID。
    #[serde(rename = "fapiao_id", skip_serializing_if = "Option::is_none")]
    pub fapiao_id: Option<String>,

    /// 发票代码。
    #[serde(rename = "fapiao_code", skip_serializing_if = "Option::is_none")]
    pub fapiao_code: Option<String>,

    /// 发票号码。
    #[serde(rename = "fapiao_number", skip_serializing_if = "Option::is_none")]
    pub fapiao_number: Option<String>,
}
