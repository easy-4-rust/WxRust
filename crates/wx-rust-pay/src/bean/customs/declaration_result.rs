//! 对应 Java `com.github.binarywang.wxpay.bean.customs.DeclarationResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeclarationResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "20150806125346"
    )]
    pub out_trade_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "state")]
    pub state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_order_no"
    )]
    pub sub_order_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_order_id"
    )]
    pub sub_order_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "verify_department"
    )]
    pub verify_department: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "verify_department_trade_id"
    )]
    pub verify_department_trade_id: Option<String>,
}
