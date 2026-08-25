//! 服务商开具通用行业电子发票请求。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.GeneralInvoiceRequest`。

use serde::{Deserialize, Serialize};

/// 服务商开具通用行业电子发票请求。
///
/// 对应 Java: `GeneralInvoiceRequest`
///
/// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4015792574>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GeneralInvoiceRequest {
    /// 子商户号。
    #[serde(rename = "sub_mchid", skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,

    /// 开票申请单号。
    #[serde(rename = "fapiao_apply_id", skip_serializing_if = "Option::is_none")]
    pub fapiao_apply_id: Option<String>,

    /// 购买方信息。
    #[serde(rename = "buyer_information", skip_serializing_if = "Option::is_none")]
    pub buyer_information: Option<super::buyer_information::BuyerInformation>,

    /// 发票信息。
    #[serde(rename = "fapiao_information", skip_serializing_if = "Option::is_none")]
    pub fapiao_information: Option<FapiaoInformation>,
}

/// 发票信息。
///
/// 对应 Java: `GeneralInvoiceRequest.FapiaoInformation`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FapiaoInformation {
    /// 发票 ID。
    #[serde(rename = "fapiao_id", skip_serializing_if = "Option::is_none")]
    pub fapiao_id: Option<String>,

    /// 总金额。
    #[serde(rename = "total_amount", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<i64>,

    /// 发票行明细。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<InvoiceItem>>,

    /// 出口业务政策代码。
    #[serde(
        rename = "export_business_policy_code",
        skip_serializing_if = "Option::is_none"
    )]
    pub export_business_policy_code: Option<i64>,

    /// 增值税退税征收代码。
    #[serde(
        rename = "vat_refund_levy_code",
        skip_serializing_if = "Option::is_none"
    )]
    pub vat_refund_levy_code: Option<i64>,

    /// 开票人员 ID。
    #[serde(rename = "billing_person_id", skip_serializing_if = "Option::is_none")]
    pub billing_person_id: Option<String>,

    /// 开票人员。
    #[serde(rename = "billing_person", skip_serializing_if = "Option::is_none")]
    pub billing_person: Option<String>,

    /// 发票账单类型。
    #[serde(rename = "fapiao_bill_type", skip_serializing_if = "Option::is_none")]
    pub fapiao_bill_type: Option<String>,

    /// 交易信息。
    #[serde(
        rename = "transaction_information",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_information: Option<Vec<TransactionInformation>>,

    /// 备注。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

/// 发票行明细。
///
/// 对应 Java: `GeneralInvoiceRequest.InvoiceItem`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoiceItem {
    /// 税收分类编码。
    #[serde(rename = "tax_code", skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// 商品名称。
    #[serde(rename = "goods_name", skip_serializing_if = "Option::is_none")]
    pub goods_name: Option<String>,

    /// 规格型号。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<String>,

    /// 单位。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    /// 数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,

    /// 总金额。
    #[serde(rename = "total_amount", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<i64>,

    /// 税率。
    #[serde(rename = "tax_rate", skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<i64>,

    /// 是否折扣。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<bool>,

    /// 优惠政策代码。
    #[serde(
        rename = "preferential_policy_code",
        skip_serializing_if = "Option::is_none"
    )]
    pub preferential_policy_code: Option<i64>,
}

/// 交易信息。
///
/// 对应 Java: `GeneralInvoiceRequest.TransactionInformation`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransactionInformation {
    /// 支付渠道。
    #[serde(rename = "pay_channel", skip_serializing_if = "Option::is_none")]
    pub pay_channel: Option<String>,

    /// 微信支付交易号。
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,

    /// 商户订单号。
    #[serde(rename = "out_trade_no", skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,

    /// 金额。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
