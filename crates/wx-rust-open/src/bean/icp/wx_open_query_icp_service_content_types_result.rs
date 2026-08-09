//! 对应 Java `me.chanjar.weixin.open.bean.icp.WxOpenQueryIcpServiceContentTypesResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenQueryIcpServiceContentTypesResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "items", default)]
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "parent_type", default)]
    pub parent_type: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "remark", default)]
    pub remark: String,
}
