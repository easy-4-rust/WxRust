//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.ReturnOrdersQueryRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReturnOrdersQueryRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "order_id")]
    pub order_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_order_no"
    )]
    pub out_order_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_return_no"
    )]
    pub out_return_no: Option<String>,
}
