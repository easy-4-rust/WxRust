//! 对应 Java `me.chanjar.weixin.cp.bean.kf.WxCpKfMsgSendRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfMsgSendRequest {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "touser", default)]
    pub to_user: String,
    #[serde(rename = "open_kfid", default)]
    pub open_kfid: String,
    #[serde(rename = "msgid", default)]
    pub msg_id: String,
    #[serde(rename = "msgtype", default)]
    pub msg_type: String,
    #[serde(rename = "text", default)]
    pub text: crate::bean::kf::msg::wx_cp_kf_text_msg::WxCpKfTextMsg,
    #[serde(rename = "image", default)]
    pub image: crate::bean::kf::msg::wx_cp_kf_resource_msg::WxCpKfResourceMsg,
    #[serde(rename = "voice", default)]
    pub voice: crate::bean::kf::msg::wx_cp_kf_resource_msg::WxCpKfResourceMsg,
    #[serde(rename = "video", default)]
    pub video: crate::bean::kf::msg::wx_cp_kf_resource_msg::WxCpKfResourceMsg,
    #[serde(rename = "file", default)]
    pub file: crate::bean::kf::msg::wx_cp_kf_resource_msg::WxCpKfResourceMsg,
    #[serde(rename = "link", default)]
    pub link: crate::bean::kf::msg::wx_cp_kf_link_msg::WxCpKfLinkMsg,
    #[serde(rename = "miniprogram", default)]
    pub mini_program: crate::bean::kf::msg::wx_cp_kf_mini_program_msg::WxCpKfMiniProgramMsg,
    #[serde(rename = "msgmenu", default)]
    pub msg_menu: crate::bean::kf::msg::wx_cp_kf_menu_msg::WxCpKfMenuMsg,
    #[serde(rename = "location", default)]
    pub location: crate::bean::kf::msg::wx_cp_kf_location_msg::WxCpKfLocationMsg,
}
