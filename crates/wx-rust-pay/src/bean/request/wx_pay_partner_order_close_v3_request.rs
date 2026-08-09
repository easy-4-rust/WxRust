//! 对应 Java `com.github.binarywang.wxpay.bean.request.WxPayPartnerOrderCloseV3Request.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayPartnerOrderCloseV3Request {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sp_mchid")]
    pub sp_mch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mch_id: Option<String>,
    /// Java `transient` 字段：类字段保留，但 Gson/XStream 线格式跳过（不含此键）。
    #[serde(skip)]
    pub out_trade_no: Option<String>,
}
