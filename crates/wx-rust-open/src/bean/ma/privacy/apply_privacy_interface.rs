//! 对应 Java `me.chanjar.weixin.open.bean.ma.privacy.ApplyPrivacyInterface.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::ma::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplyPrivacyInterface {
    #[serde(rename = "api_name", default)]
    pub api_name: String,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "url_list", default)]
    pub url_list: Vec<String>,
    #[serde(rename = "pic_list", default)]
    pub pic_list: Vec<String>,
    #[serde(rename = "video_list", default)]
    pub video_list: Vec<String>,
}
