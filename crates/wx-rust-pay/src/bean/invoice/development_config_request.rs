//! 服务商电子发票开发配置请求。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.DevelopmentConfigRequest`。

use serde::{Deserialize, Serialize};

/// 服务商电子发票开发配置请求。
///
/// 对应 Java: `DevelopmentConfigRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DevelopmentConfigRequest {
    /// 回调 URL。
    #[serde(rename = "callback_url", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,

    /// 子商户代码。
    #[serde(rename = "sub_mch_code", skip_serializing_if = "Option::is_none")]
    pub sub_mch_code: Option<String>,

    /// 是否展示发票入口。
    #[serde(rename = "show_fapiao_cell", skip_serializing_if = "Option::is_none")]
    pub show_fapiao_cell: Option<bool>,

    /// 是否支持增值税发票。
    #[serde(rename = "support_vat_fapiao", skip_serializing_if = "Option::is_none")]
    pub support_vat_fapiao: Option<bool>,
}
