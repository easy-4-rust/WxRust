//! 上传电子发票文件结果。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.InvoiceFileUploadResult`。

use serde::{Deserialize, Serialize};

/// 上传电子发票文件结果。
///
/// 对应 Java: `InvoiceFileUploadResult`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoiceFileUploadResult {
    /// 发票媒体 ID。
    #[serde(rename = "fapiao_media_id", skip_serializing_if = "Option::is_none")]
    pub fapiao_media_id: Option<String>,
}
