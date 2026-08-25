//! 用户抬头填写小程序跳转信息。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.TitleUrlResult`。

use serde::{Deserialize, Serialize};

/// 用户抬头填写小程序跳转信息。
///
/// 对应 Java: `TitleUrlResult`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TitleUrlResult {
    /// 抬头填写链接。
    #[serde(rename = "title_url", skip_serializing_if = "Option::is_none")]
    pub title_url: Option<String>,

    /// 小程序 AppID。
    #[serde(rename = "miniprogram_appid", skip_serializing_if = "Option::is_none")]
    pub miniprogram_appid: Option<String>,

    /// 小程序路径。
    #[serde(rename = "miniprogram_path", skip_serializing_if = "Option::is_none")]
    pub miniprogram_path: Option<String>,

    /// 小程序用户名。
    #[serde(
        rename = "miniprogram_user_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub miniprogram_user_name: Option<String>,
}
