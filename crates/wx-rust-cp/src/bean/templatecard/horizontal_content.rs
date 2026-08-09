//! 对应 Java `me.chanjar.weixin.cp.bean.templatecard.HorizontalContent.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HorizontalContent {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "keyname", default)]
    pub keyname: String,
    #[serde(rename = "value", default)]
    pub value: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "media_id", default)]
    pub media_id: String,
    #[serde(rename = "userid", default)]
    pub userid: String,
}
