//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaTemplateData.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaTemplateData {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "value", default)]
    pub value: String,
    #[serde(rename = "color", default)]
    pub color: String,
}
