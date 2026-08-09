//! 对应 Java `me.chanjar.weixin.common.bean.ocr.WxOcrDrivingLicenseResult`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOcrDrivingLicenseResult {
    /// idNum
    #[serde(rename = "id_num", default)]
    pub id_num: String,
    /// name
    #[serde(rename = "name", default)]
    pub name: String,
    /// sex
    #[serde(rename = "sex", default)]
    pub sex: String,
    /// nationality
    #[serde(rename = "nationality", default)]
    pub nationality: String,
    /// address
    #[serde(rename = "address", default)]
    pub address: String,
    /// birthDate
    #[serde(rename = "birth_date", default)]
    pub birth_date: String,
    /// issueDate
    #[serde(rename = "issue_date", default)]
    pub issue_date: String,
    /// carClass
    #[serde(rename = "car_class", default)]
    pub car_class: String,
    /// validFrom
    #[serde(rename = "valid_from", default)]
    pub valid_from: String,
    /// validTo
    #[serde(rename = "valid_to", default)]
    pub valid_to: String,
    /// officialSeal
    #[serde(rename = "official_seal", default)]
    pub official_seal: String,
}
