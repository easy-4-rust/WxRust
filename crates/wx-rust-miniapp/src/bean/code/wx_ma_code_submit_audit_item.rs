//! 对应 Java `cn.binarywang.wx.miniapp.bean.code.WxMaCodeSubmitAuditItem.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCodeSubmitAuditItem {
    #[serde(rename = "address", default)]
    pub address: String,
    #[serde(rename = "tag", default)]
    pub tag: String,
    #[serde(rename = "first_class", default)]
    pub first_class: String,
    #[serde(rename = "second_class", default)]
    pub second_class: String,
    #[serde(rename = "third_class", default)]
    pub third_class: String,
    #[serde(rename = "first_id", default)]
    pub first_id: i64,
    #[serde(rename = "second_id", default)]
    pub second_id: i64,
    #[serde(rename = "third_id", default)]
    pub third_id: i64,
    #[serde(rename = "title", default)]
    pub title: String,
}
