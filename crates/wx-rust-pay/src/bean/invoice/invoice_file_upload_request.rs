//! 上传电子发票 PDF 文件请求。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.InvoiceFileUploadRequest`。

use serde::{Deserialize, Serialize};

/// 上传电子发票 PDF 文件请求。
///
/// `digest` 为官方要求的 SM3 十六进制摘要。
///
/// 对应 Java: `InvoiceFileUploadRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoiceFileUploadRequest {
    /// 子商户号。
    #[serde(rename = "sub_mchid", skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,

    /// 文件类型，默认 PDF。
    #[serde(rename = "file_type", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,

    /// 摘要算法，默认 SM3。
    ///
    /// 微信支付官方接口字段即为 `digest_alogrithm`（文档中的既定拼写）。
    #[serde(rename = "digest_alogrithm", skip_serializing_if = "Option::is_none")]
    pub digest_alogrithm: Option<String>,

    /// 文件摘要。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}
