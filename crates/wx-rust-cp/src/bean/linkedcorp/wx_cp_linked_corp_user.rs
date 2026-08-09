//! 对应 Java `me.chanjar.weixin.cp.bean.linkedcorp.WxCpLinkedCorpUser.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpLinkedCorpUser {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "department", default)]
    pub department: Vec<String>,
    #[serde(rename = "mobile", default)]
    pub mobile: String,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "position", default)]
    pub position: String,
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
    #[serde(rename = "extAttrs", default)]
    pub ext_attrs: Vec<Attr>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attr {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "textValue", default)]
    pub text_value: String,
    #[serde(rename = "webUrl", default)]
    pub web_url: String,
    #[serde(rename = "webTitle", default)]
    pub web_title: String,
}
