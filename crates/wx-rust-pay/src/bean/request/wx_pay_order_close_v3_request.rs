//! 对应 Java `com.github.binarywang.wxpay.bean.request.WxPayOrderCloseV3Request.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayOrderCloseV3Request {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    /// Java `transient` 字段：类字段保留，但 Gson/XStream 线格式跳过（不含此键）。
    #[serde(skip)]
    pub out_trade_no: Option<String>,
}
