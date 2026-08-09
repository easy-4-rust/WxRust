//! 对应 Java `com.github.binarywang.wxpay.bean.customs.DeclarationQueryRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeclarationQueryRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "order_type"
    )]
    pub order_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "order_no")]
    pub order_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customs")]
    pub customs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offset")]
    pub offset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limit")]
    pub limit: Option<String>,
}
