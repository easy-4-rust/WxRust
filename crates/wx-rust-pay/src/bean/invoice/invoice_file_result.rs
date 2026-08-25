//! 电子发票文件下载信息。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.InvoiceFileResult`。

use serde::{Deserialize, Serialize};

/// 电子发票文件下载信息。
///
/// 对应 Java: `InvoiceFileResult`
///
/// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4015792576>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoiceFileResult {
    /// 发票下载信息列表。
    #[serde(
        rename = "fapiao_download_info_list",
        skip_serializing_if = "Option::is_none"
    )]
    pub fapiao_download_info_list: Option<Vec<DownloadInfo>>,
}

/// 发票下载信息。
///
/// 对应 Java: `InvoiceFileResult.DownloadInfo`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DownloadInfo {
    /// 发票 ID。
    #[serde(rename = "fapiao_id", skip_serializing_if = "Option::is_none")]
    pub fapiao_id: Option<String>,

    /// 下载链接。
    #[serde(rename = "download_url", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,

    /// 状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
