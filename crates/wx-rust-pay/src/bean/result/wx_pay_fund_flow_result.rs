//! 对应 Java `com.github.binarywang.wxpay.bean.result.WxPayFundFlowResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayFundFlowResult {
    #[serde(default, rename = "wxPayFundFlowBaseResultList")]
    pub wx_pay_fund_flow_base_result_list: Vec<WxPayFundFlowBaseResult>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "totalRecord"
    )]
    pub total_record: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "incomeRecord"
    )]
    pub income_record: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "incomeAmount"
    )]
    pub income_amount: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "expenditureRecord"
    )]
    pub expenditure_record: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "expenditureAmount"
    )]
    pub expenditure_amount: Option<String>,
}
