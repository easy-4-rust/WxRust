//! 对应 Java `com.github.binarywang.wxpay.bean.notify.WxPayRefundNotifyResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "xml")]
pub struct WxPayRefundNotifyResult {
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "req_info")]
    pub req_info_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reqInfo")]
    pub req_info: Option<ReqInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReqInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refund_id")]
    pub refund_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_refund_no"
    )]
    pub out_refund_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total_fee")]
    pub total_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settlement_total_fee"
    )]
    pub settlement_total_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_fee"
    )]
    pub refund_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settlement_refund_fee"
    )]
    pub settlement_refund_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_status"
    )]
    pub refund_status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "success_time"
    )]
    pub success_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_recv_accout"
    )]
    pub refund_recv_account: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_account"
    )]
    pub refund_account: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_request_source"
    )]
    pub refund_request_source: Option<String>,
}

/// 退款结果通知（对应 Java `WxPayRefundNotifyResult`）。
impl WxPayRefundNotifyResult {
    /// 从 XML 解析（对应 Java `fromXML`）：仅结构解析，不涉及 req_info 解密。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self = quick_xml::de::from_str(xml)
            .map_err(|e| format!("WxPayRefundNotifyResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        Ok(v)
    }

    /// 解密并解析 `req_info`（对应 Java `decryptReqInfo(String mchKey)`：
    /// `md5(mchKey)` 作为 AES-256-ECB 密钥解密 `req_info` 的 Base64 内容）。
    ///
    /// **Wave 2 实现**：AES-256-ECB 解密 + `ReqInfo::from_xml`；当前返回未实现错误。
    pub fn decrypt_req_info(&mut self, mch_key: &str) -> Result<(), String> {
        let _ = mch_key;
        Err(
            "WxPayRefundNotifyResult::decrypt_req_info 未实现（Wave 2：AES-256-ECB 解密 req_info）"
                .to_string(),
        )
    }
}
