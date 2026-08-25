//! 电子发票查询结果。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.InvoiceResult`。

use serde::{Deserialize, Serialize};

/// 电子发票查询结果。
///
/// 对应 Java: `InvoiceResult`
///
/// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4015792567>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoiceResult {
    /// 总数。
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,

    /// 发票信息列表。
    #[serde(rename = "fapiao_information", skip_serializing_if = "Option::is_none")]
    pub fapiao_information: Option<Vec<InvoiceInformation>>,
}

/// 发票信息。
///
/// 对应 Java: `InvoiceResult.InvoiceInformation`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoiceInformation {
    /// 发票 ID。
    #[serde(rename = "fapiao_id", skip_serializing_if = "Option::is_none")]
    pub fapiao_id: Option<String>,

    /// 状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// 蓝票信息。
    #[serde(rename = "blue_fapiao", skip_serializing_if = "Option::is_none")]
    pub blue_fapiao: Option<Fapiao>,

    /// 红票信息。
    #[serde(rename = "red_fapiao", skip_serializing_if = "Option::is_none")]
    pub red_fapiao: Option<Fapiao>,

    /// 总金额。
    #[serde(rename = "total_amount", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<i64>,

    /// 税额。
    #[serde(rename = "tax_amount", skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<i64>,

    /// 金额。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// 发票错误代码。
    #[serde(rename = "fapiao_error_code", skip_serializing_if = "Option::is_none")]
    pub fapiao_error_code: Option<String>,

    /// 发票错误信息。
    #[serde(
        rename = "fapiao_error_message",
        skip_serializing_if = "Option::is_none"
    )]
    pub fapiao_error_message: Option<String>,
}

/// 发票详情。
///
/// 对应 Java: `InvoiceResult.Fapiao`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Fapiao {
    /// 发票代码。
    #[serde(rename = "fapiao_code", skip_serializing_if = "Option::is_none")]
    pub fapiao_code: Option<String>,

    /// 发票号码。
    #[serde(rename = "fapiao_number", skip_serializing_if = "Option::is_none")]
    pub fapiao_number: Option<String>,

    /// 校验码。
    #[serde(rename = "check_code", skip_serializing_if = "Option::is_none")]
    pub check_code: Option<String>,

    /// 密码。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// 开票时间。
    #[serde(rename = "fapiao_time", skip_serializing_if = "Option::is_none")]
    pub fapiao_time: Option<String>,
}
