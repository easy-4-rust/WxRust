//! 对应 Java `com.github.binarywang.wxpay.bean.entpay.EntPayRedpackRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "xml")]
pub struct EntPayRedpackRequest {
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
        rename = "mch_billno"
    )]
    pub mch_bill_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "wxappid")]
    pub wx_app_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sender_name"
    )]
    pub sender_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "agentid")]
    pub agent_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sender_header_media_id"
    )]
    pub sender_header_media_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "re_openid")]
    pub re_openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_amount"
    )]
    pub total_amount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "wishing")]
    pub wishing: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "act_name")]
    pub act_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remark")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scene_id")]
    pub scene_id: Option<String>,
}

impl EntPayRedpackRequest {
    /// 从 XML 解析（对应 Java `fromXML`，XStream 语义：未知元素忽略、缺失字段默认）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        quick_xml::de::from_str(xml).map_err(|e| format!("EntPayRedpackRequest 解析失败: {e}"))
    }

    /// 序列化为 XML（根元素 `<xml>`，对应 Java `toXML`）。
    ///
    /// 注意：quick-xml 以转义文本代替 Java 的 CDATA、空元素输出 `<x/>`
    /// （`expand_empty_elements` 归一为 `<x></x>`）——解析语义等价，
    /// 逐字节格式化差异记录于 Wave 2。
    pub fn to_xml(&self) -> Result<String, String> {
        let out = quick_xml::se::to_string(self)
            .map_err(|e| format!("EntPayRedpackRequest 序列化失败: {e}"))?;
        Ok(crate::bean::xml::expand_empty_elements(&out))
    }
}
