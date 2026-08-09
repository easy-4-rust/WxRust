//! 对应 Java `me.chanjar.weixin.cp.bean.kf.msg.WxCpKfMenuMsg.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::kf::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfMenuMsg {
    #[serde(rename = "head_content", default)]
    pub head_content: String,
    #[serde(rename = "list", default)]
    pub list: Vec<WxCpKfMenuItem>,
    #[serde(rename = "tail_content", default)]
    pub tail_content: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfMenuItem {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "click", default)]
    pub click: crate::bean::kf::msg::wx_cp_kf_menu_msg::MenuClick,
    #[serde(rename = "view", default)]
    pub view: crate::bean::kf::msg::wx_cp_kf_menu_msg::MenuView,
    #[serde(rename = "miniprogram", default)]
    pub mini_program: crate::bean::kf::msg::wx_cp_kf_menu_msg::MiniProgram,
    #[serde(rename = "text", default)]
    pub text: crate::bean::kf::msg::wx_cp_kf_menu_msg::MenuText,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MenuClick {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "content", default)]
    pub content: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MenuView {
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "content", default)]
    pub content: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MiniProgram {
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "pagepath", default)]
    pub page_path: String,
    #[serde(rename = "content", default)]
    pub content: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MenuText {
    #[serde(rename = "content", default)]
    pub content: String,
}
