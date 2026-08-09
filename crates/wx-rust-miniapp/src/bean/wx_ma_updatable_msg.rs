//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaUpdatableMsg.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaUpdatableMsg {
    #[serde(rename = "activity_id", default)]
    pub activity_id: String,
    #[serde(rename = "target_state", default)]
    pub target_state: i32,
    #[serde(rename = "template_info", default)]
    pub template_info: TemplateInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateInfo {
    #[serde(rename = "parameter_list", default)]
    pub parameter_list: Vec<Parameter>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Parameter {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "value", default)]
    pub value: String,
}
