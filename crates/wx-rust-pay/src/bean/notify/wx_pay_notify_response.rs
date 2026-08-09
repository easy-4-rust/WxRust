//! 对应 Java `com.github.binarywang.wxpay.bean.notify.WxPayNotifyResponse.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "xml")]
pub struct WxPayNotifyResponse {
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
}
/// 通知响应构造辅助（对应 Java `WxPayNotifyResponse` 静态方法）。
impl WxPayNotifyResponse {
    /// 成功响应（对应 Java `WxPayNotifyResponse.successResp(String msg)`，
    /// 无参时 msg 为 "OK"）。
    pub fn success() -> String {
        Self::success_resp("OK")
    }

    /// 失败响应（对应 Java `WxPayNotifyResponse.failResp(String msg)`）。
    pub fn fail(msg: &str) -> String {
        Self::fail_resp(msg)
    }

    /// 成功响应（对应 Java `WxPayNotifyResponse.successResp`）。
    pub fn success_resp(msg: &str) -> String {
        Self::generate_xml("SUCCESS", msg)
    }

    /// 失败响应（对应 Java `WxPayNotifyResponse.failResp`）。
    pub fn fail_resp(msg: &str) -> String {
        Self::generate_xml("FAIL", msg)
    }

    /// 生成响应 XML（对应 Java `generateXml`，单行 CDATA 格式）。
    pub fn generate_xml(code: &str, msg: &str) -> String {
        format!(
            "<xml><return_code><![CDATA[{code}]]></return_code><return_msg><![CDATA[{msg}]]></return_msg></xml>"
        )
    }
}
