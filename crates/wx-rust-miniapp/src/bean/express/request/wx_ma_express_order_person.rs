//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.request.WxMaExpressOrderPerson.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressOrderPerson {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "tel", default)]
    pub tel: String,
    #[serde(rename = "mobile", default)]
    pub mobile: String,
    #[serde(rename = "company", default)]
    pub company: String,
    #[serde(rename = "post_code", default)]
    pub post_code: String,
    #[serde(rename = "country", default)]
    pub country: String,
    #[serde(rename = "province", default)]
    pub province: String,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "area", default)]
    pub area: String,
    #[serde(rename = "address", default)]
    pub address: String,
}
