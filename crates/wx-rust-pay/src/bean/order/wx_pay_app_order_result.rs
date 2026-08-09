//! 对应 Java `com.github.binarywang.wxpay.bean.order.WxPayAppOrderResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayAppOrderResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sign")]
    pub sign: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prepayId")]
    pub prepay_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "partnerId")]
    pub partner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appId")]
    pub app_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "packageValue"
    )]
    pub package_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeStamp")]
    pub time_stamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonceStr")]
    pub nonce_str: Option<String>,
}
