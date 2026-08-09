//! 对应 Java `com.github.binarywang.wxpay.bean.request.WxPayApplyTradeBillV3Request.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayApplyTradeBillV3Request {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bill_date")]
    pub bill_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bill_type")]
    pub bill_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tar_type")]
    pub tar_type: Option<String>,
}
