//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.payroll.PreOrderResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreOrderResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authenticate_number"
    )]
    pub authenticate_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "token")]
    pub token: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "expires_in"
    )]
    pub expires_in: Option<i32>,
}
