//! 对应 Java `me.chanjar.weixin.common.bean.ocr.WxOcrIdCardResult`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOcrIdCardResult {
    /// type
    #[serde(rename = "type", default)]
    pub r#type: String,
    /// name
    #[serde(rename = "name", default)]
    pub name: String,
    /// id
    #[serde(rename = "id", default)]
    pub id: String,
    /// addr
    #[serde(rename = "addr", default)]
    pub addr: String,
    /// gender
    #[serde(rename = "gender", default)]
    pub gender: String,
    /// nationality
    #[serde(rename = "nationality", default)]
    pub nationality: String,
    /// validDate
    #[serde(rename = "valid_date", default)]
    pub valid_date: String,
}
