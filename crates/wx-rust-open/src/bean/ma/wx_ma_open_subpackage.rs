//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxMaOpenSubpackage.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOpenSubpackage {
    #[serde(rename = "root", default)]
    pub root: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "pages", default)]
    pub pages: String,
    #[serde(rename = "independent", default)]
    pub independent: bool,
}
