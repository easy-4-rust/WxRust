//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxFastMaCategory.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxFastMaCategory {
    #[serde(rename = "first", default)]
    pub first: i32,
    #[serde(rename = "second", default)]
    pub second: i32,
    #[serde(rename = "certicates", default)]
    pub certicates: Vec<Certificate>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Certificate {
    #[serde(rename = "key", default)]
    pub key: String,
    #[serde(rename = "value", default)]
    pub value: String,
}
