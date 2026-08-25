//! 创建电子发票卡券模板请求。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.CardTemplateRequest`。

use serde::{Deserialize, Serialize};

/// 创建电子发票卡券模板请求。
///
/// 对应 Java: `CardTemplateRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardTemplateRequest {
    /// 子商户号。
    #[serde(rename = "sub_mchid", skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,

    /// 卡券 AppID。
    #[serde(rename = "card_appid", skip_serializing_if = "Option::is_none")]
    pub card_appid: Option<String>,

    /// 卡券模板信息。
    #[serde(
        rename = "card_template_information",
        skip_serializing_if = "Option::is_none"
    )]
    pub card_template_information: Option<TemplateInformation>,
}

/// 卡券模板信息。
///
/// 对应 Java: `CardTemplateRequest.TemplateInformation`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateInformation {
    /// 收款方名称。
    #[serde(rename = "payee_name", skip_serializing_if = "Option::is_none")]
    pub payee_name: Option<String>,

    /// Logo URL。
    #[serde(rename = "logo_url", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    /// 自定义栏。
    #[serde(rename = "custom_cell", skip_serializing_if = "Option::is_none")]
    pub custom_cell: Option<CustomCell>,
}

/// 自定义栏。
///
/// 对应 Java: `CardTemplateRequest.CustomCell`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomCell {
    /// 入口名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<String>,

    /// 入口描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 跳转链接。
    #[serde(rename = "jump_url", skip_serializing_if = "Option::is_none")]
    pub jump_url: Option<String>,

    /// 小程序用户名。
    #[serde(
        rename = "miniprogram_user_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub miniprogram_user_name: Option<String>,

    /// 小程序路径。
    #[serde(rename = "miniprogram_path", skip_serializing_if = "Option::is_none")]
    pub miniprogram_path: Option<String>,
}
