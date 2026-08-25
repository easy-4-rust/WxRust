//! 电子发票购买方抬头信息。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.BuyerInformation`。

use serde::{Deserialize, Serialize};

/// 电子发票购买方抬头信息。
///
/// 敏感字段 phone、email 由调用方按支付文档加密。
///
/// 对应 Java: `BuyerInformation`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuyerInformation {
    /// 抬头类型。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// 抬头名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 纳税人识别号。
    #[serde(rename = "taxpayer_id", skip_serializing_if = "Option::is_none")]
    pub taxpayer_id: Option<String>,

    /// 地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// 电话。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,

    /// 开户银行。
    #[serde(rename = "bank_name", skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    /// 银行账号。
    #[serde(rename = "bank_account", skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String>,

    /// 手机号（敏感字段，需加密）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// 邮箱（敏感字段，需加密）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// 金额。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// 商户订单号。
    #[serde(rename = "out_trade_no", skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,

    /// 发票账单类型。
    #[serde(rename = "fapiao_bill_type", skip_serializing_if = "Option::is_none")]
    pub fapiao_bill_type: Option<String>,

    /// 用户申请备注。
    #[serde(rename = "user_apply_message", skip_serializing_if = "Option::is_none")]
    pub user_apply_message: Option<String>,
}
