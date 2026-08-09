//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.payroll.PreOrderWithAuthRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreOrderWithAuthRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authenticate_number"
    )]
    pub authenticate_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "project_name"
    )]
    pub project_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "employer_name"
    )]
    pub employer_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "user_name")]
    pub user_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_card_number"
    )]
    pub id_card_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "employment_type"
    )]
    pub employment_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authenticate_type"
    )]
    pub authenticate_type: Option<String>,
}
