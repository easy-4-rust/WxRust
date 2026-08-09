//! 对应 Java `me.chanjar.weixin.channel.bean.category.QualificationInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QualificationInfo {
    #[serde(rename = "qua_id", default)]
    pub id: String,
    #[serde(rename = "need_to_apply", default)]
    pub need_to_apply: bool,
    #[serde(rename = "tips", default)]
    pub tips: String,
    #[serde(rename = "mandatory", default)]
    pub mandatory: bool,
    #[serde(rename = "name", default)]
    pub name: String,
}
