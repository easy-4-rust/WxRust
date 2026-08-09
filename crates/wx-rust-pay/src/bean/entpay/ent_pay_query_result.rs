//! 对应 Java `com.github.binarywang.wxpay.bean.entpay.EntPayQueryResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "xml")]
pub struct EntPayQueryResult {
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
        rename = "partner_trade_no"
    )]
    pub partner_trade_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "detail_id")]
    pub detail_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "status")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reason")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_name"
    )]
    pub transfer_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payment_amount"
    )]
    pub payment_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transfer_time"
    )]
    pub transfer_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payment_time"
    )]
    pub payment_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desc")]
    pub desc: Option<String>,
}

impl EntPayQueryResult {
    /// 从 XML 解析（对应 Java `fromXML`，XStream 语义：未知元素忽略、缺失字段默认）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self =
            quick_xml::de::from_str(xml).map_err(|e| format!("EntPayQueryResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        Ok(v)
    }

    /// 序列化为 XML（根元素 `<xml>`，对应 Java `toXML`）。
    ///
    /// 注意：quick-xml 以转义文本代替 Java 的 CDATA、空元素输出 `<x/>`
    /// （`expand_empty_elements` 归一为 `<x></x>`）——解析语义等价，
    /// 逐字节格式化差异记录于 Wave 2。
    pub fn to_xml(&self) -> Result<String, String> {
        let out = quick_xml::se::to_string(self)
            .map_err(|e| format!("EntPayQueryResult 序列化失败: {e}"))?;
        Ok(crate::bean::xml::expand_empty_elements(&out))
    }
}
