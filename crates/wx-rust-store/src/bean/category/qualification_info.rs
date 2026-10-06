//! 对应 Java `com.binarywang.wxjava.store.bean.category.QualificationInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 QualificationInfo；对应 Java com.binarywang.wxjava.store.bean.category.QualificationInfo.java。
pub struct QualificationInfo {
    #[serde(rename = "qua_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "need_to_apply", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_to_apply: Option<bool>,
    #[serde(rename = "tips", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tips: Option<String>,
    #[serde(rename = "mandatory", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(rename = "name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
