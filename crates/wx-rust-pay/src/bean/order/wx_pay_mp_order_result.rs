//! 对应 Java `com.github.binarywang.wxpay.bean.order.WxPayMpOrderResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "xml")]
pub struct WxPayMpOrderResult {
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
}

impl WxPayMpOrderResult {
    /// 从 XML 解析（对应 Java `fromXML`，XStream 语义：未知元素忽略、缺失字段默认）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        quick_xml::de::from_str(xml).map_err(|e| format!("WxPayMpOrderResult 解析失败: {e}"))
    }

    /// 序列化为 XML（根元素 `<xml>`，对应 Java `toXML`）。
    ///
    /// 注意：quick-xml 以转义文本代替 Java 的 CDATA、空元素输出 `<x/>`
    /// （`expand_empty_elements` 归一为 `<x></x>`）——解析语义等价，
    /// 逐字节格式化差异记录于 Wave 2。
    pub fn to_xml(&self) -> Result<String, String> {
        let out = quick_xml::se::to_string(self)
            .map_err(|e| format!("WxPayMpOrderResult 序列化失败: {e}"))?;
        Ok(crate::bean::xml::expand_empty_elements(&out))
    }
}
