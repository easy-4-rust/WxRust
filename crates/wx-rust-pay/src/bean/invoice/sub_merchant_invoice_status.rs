//! 服务商子商户电子发票能力状态。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.SubMerchantInvoiceStatus`。

use serde::{Deserialize, Serialize};

/// 服务商子商户电子发票能力状态。
///
/// 对应 Java: `SubMerchantInvoiceStatus`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubMerchantInvoiceStatus {
    /// 子商户号。
    #[serde(rename = "sub_mchid", skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,

    /// 第三方开票模式。
    #[serde(rename = "third_mode", skip_serializing_if = "Option::is_none")]
    pub third_mode: Option<Mode>,

    /// 数字税务模式。
    #[serde(rename = "digital_tax_mode", skip_serializing_if = "Option::is_none")]
    pub digital_tax_mode: Option<DigitalTaxMode>,
}

/// 开票模式。
///
/// 对应 Java: `SubMerchantInvoiceStatus.Mode`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mode {
    /// 状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 数字税务模式。
///
/// 对应 Java: `SubMerchantInvoiceStatus.DigitalTaxMode`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DigitalTaxMode {
    /// 状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// 开票人员信息。
    #[serde(
        rename = "billing_person_info",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_person_info: Option<Vec<BillingPerson>>,

    /// 接入时间。
    #[serde(rename = "access_time", skip_serializing_if = "Option::is_none")]
    pub access_time: Option<String>,

    /// 过期时间。
    #[serde(rename = "expired_time", skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,

    /// 接入失败原因。
    #[serde(rename = "access_fail_reason", skip_serializing_if = "Option::is_none")]
    pub access_fail_reason: Option<String>,

    /// 能力信息。
    #[serde(rename = "ability_info", skip_serializing_if = "Option::is_none")]
    pub ability_info: Option<Vec<Ability>>,
}

/// 开票人员。
///
/// 对应 Java: `SubMerchantInvoiceStatus.BillingPerson`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BillingPerson {
    /// ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// 名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 能力信息。
///
/// 对应 Java: `SubMerchantInvoiceStatus.Ability`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ability {
    /// 类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// 状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
