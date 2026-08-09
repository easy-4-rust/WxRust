//! 对应 Java `com.github.binarywang.wxpay.bean.result.WxSignStatusNotifyResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "xml")]
pub struct WxSignStatusNotifyResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "return_code"
    )]
    pub return_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "return_msg"
    )]
    pub return_msg: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "result_code"
    )]
    pub result_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "err_code")]
    pub err_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "err_code_des"
    )]
    pub err_code_des: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "error_code"
    )]
    pub error_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "error_message"
    )]
    pub error_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mch_id")]
    pub mch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_app_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_mch_id"
    )]
    pub sub_mch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonce_str")]
    pub nonce_str: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sign")]
    pub sign: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "xmlString")]
    pub xml_string: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contract_code"
    )]
    pub contract_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "plan_id")]
    pub plan_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub open_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "change_type"
    )]
    pub change_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "operate_time"
    )]
    pub operate_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contract_id"
    )]
    pub contract_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contract_expired_time"
    )]
    pub contract_expired_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contract_termination_mode"
    )]
    pub contract_termination_mode: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "request_serial"
    )]
    pub request_serial: Option<i64>,
}

/// 签约状态变更通知（XML，对应 Java `WxSignStatusNotifyResult`）。
impl WxSignStatusNotifyResult {
    /// 从 XML 解析（对应 Java `fromXML`）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self = quick_xml::de::from_str(xml)
            .map_err(|e| format!("WxSignStatusNotifyResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        Ok(v)
    }
}
