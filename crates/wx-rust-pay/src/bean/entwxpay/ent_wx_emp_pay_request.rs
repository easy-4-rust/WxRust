//! 对应 Java `com.github.binarywang.wxpay.bean.entwxpay.EntWxEmpPayRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "xml")]
pub struct EntWxEmpPayRequest {
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sign_type")]
    pub sign_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "workwx_sign"
    )]
    pub work_wx_sign: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "partner_trade_no"
    )]
    pub partner_trade_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "device_info"
    )]
    pub device_info: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "check_name"
    )]
    pub check_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "re_user_name"
    )]
    pub re_user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desc")]
    pub description: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "spbill_create_ip"
    )]
    pub spbill_create_ip: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ww_msg_type"
    )]
    pub ww_msg_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "approval_number"
    )]
    pub approval_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "approval_type"
    )]
    pub approval_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "act_name")]
    pub act_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "agentid")]
    pub agent_id: Option<i32>,
}

impl EntWxEmpPayRequest {
    /// 从 XML 解析（对应 Java `fromXML`，XStream 语义：未知元素忽略、缺失字段默认）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        quick_xml::de::from_str(xml).map_err(|e| format!("EntWxEmpPayRequest 解析失败: {e}"))
    }

    /// 序列化为 XML（根元素 `<xml>`，对应 Java `toXML`）。
    ///
    /// 注意：quick-xml 以转义文本代替 Java 的 CDATA、空元素输出 `<x/>`
    /// （`expand_empty_elements` 归一为 `<x></x>`）——解析语义等价，
    /// 逐字节格式化差异记录于 Wave 2。
    pub fn to_xml(&self) -> Result<String, String> {
        let out = quick_xml::se::to_string(self)
            .map_err(|e| format!("EntWxEmpPayRequest 序列化失败: {e}"))?;
        Ok(crate::bean::xml::expand_empty_elements(&out))
    }
}
