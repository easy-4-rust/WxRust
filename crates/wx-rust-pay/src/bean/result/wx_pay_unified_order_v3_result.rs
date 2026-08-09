//! 对应 Java `com.github.binarywang.wxpay.bean.result.WxPayUnifiedOrderV3Result.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayUnifiedOrderV3Result {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prepay_id")]
    pub prepay_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "h5_url")]
    pub h5_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "code_url")]
    pub code_url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JsapiResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appId")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeStamp")]
    pub time_stamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonceStr")]
    pub nonce_str: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "package")]
    pub package_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signType")]
    pub sign_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "paySign")]
    pub pay_sign: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prepayId")]
    pub prepay_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AppResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "partnerid")]
    pub partner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prepayid")]
    pub prepay_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "package")]
    pub package_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noncestr")]
    pub noncestr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sign")]
    pub sign: Option<String>,
}
