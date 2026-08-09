//! 对应 Java `com.github.binarywang.wxpay.bean.request.CombineCloseRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CombineCloseRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "combine_appid"
    )]
    pub combine_appid: Option<String>,
    /// Java `transient` 字段：类字段保留，但 Gson/XStream 线格式跳过（不含此键）。
    #[serde(skip)]
    pub combine_out_trade_no: Option<String>,
    #[serde(default, rename = "sub_orders")]
    pub sub_orders: Vec<SubOrders>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubOrders {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_appid: Option<String>,
}
