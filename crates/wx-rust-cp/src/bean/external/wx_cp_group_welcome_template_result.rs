//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpGroupWelcomeTemplateResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGroupWelcomeTemplateResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "text", default)]
    pub text: crate::bean::wx_cp_user_external_contact_info::Text,
    #[serde(rename = "image", default)]
    pub image: crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::Image,
    #[serde(rename = "link", default)]
    pub link: crate::bean::oa::doc::wx_cp_doc_sheet_data::Link,
    #[serde(rename = "miniprogram", default)]
    pub miniprogram: crate::bean::wx_cp_user_external_contact_info::MiniProgram,
    #[serde(rename = "file", default)]
    pub file: crate::bean::oa::wedrive::wx_cp_file_rename::File,
    #[serde(rename = "video", default)]
    pub video: crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::Video,
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "notify", default)]
    pub notify: i32,
}

impl WxCpGroupWelcomeTemplateResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpGroupWelcomeTemplateResult 解析失败: {e}"))
    }
}

impl WxCpGroupWelcomeTemplateResult {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpGroupWelcomeTemplateResult 序列化失败: {e}"))
    }
}
