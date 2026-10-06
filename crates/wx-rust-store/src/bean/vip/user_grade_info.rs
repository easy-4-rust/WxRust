//! 对应 Java `com.binarywang.wxjava.store.bean.vip.UserGradeInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 UserGradeInfo；对应 Java com.binarywang.wxjava.store.bean.vip.UserGradeInfo.java。
pub struct UserGradeInfo {
    #[serde(rename = "grade", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<i32>,
    #[serde(rename = "experience_value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_value: Option<String>,
}
