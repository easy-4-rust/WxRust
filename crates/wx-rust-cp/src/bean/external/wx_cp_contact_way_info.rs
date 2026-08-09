//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpContactWayInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpContactWayInfo {
    #[serde(rename = "contact_way", default)]
    pub contact_way: ContactWay,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContactWay {
    #[serde(rename = "config_id", default)]
    pub config_id: String,
    #[serde(rename = "type", default)]
    pub r#type: Type,
    #[serde(rename = "scene", default)]
    pub scene: Scene,
    #[serde(rename = "style", default)]
    pub style: i32,
    #[serde(rename = "remark", default)]
    pub remark: String,
    #[serde(rename = "skip_verify", default)]
    pub skip_verify: bool,
    #[serde(rename = "state", default)]
    pub state: String,
    #[serde(rename = "qr_code", default)]
    pub qr_code: String,
    #[serde(rename = "user", default)]
    pub users: Vec<String>,
    #[serde(rename = "party", default)]
    pub parties: Vec<String>,
    #[serde(rename = "is_temp", default)]
    pub is_temp: bool,
    #[serde(rename = "expires_in", default)]
    pub expires_in: i32,
    #[serde(rename = "chat_expires_in", default)]
    pub chat_expires_in: i32,
    #[serde(rename = "unionid", default)]
    pub union_id: String,
    #[serde(rename = "is_exclusive", default)]
    pub is_exclusive: bool,
    #[serde(rename = "conclusions", default)]
    pub conclusions: Conclusion,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Conclusion {
    #[serde(rename = "textContent", default)]
    pub text_content: String,
    #[serde(rename = "imgMediaId", default)]
    pub img_media_id: String,
    #[serde(rename = "imgPicUrl", default)]
    pub img_pic_url: String,
    #[serde(rename = "linkTitle", default)]
    pub link_title: String,
    #[serde(rename = "linkPicUrl", default)]
    pub link_pic_url: String,
    #[serde(rename = "linkDesc", default)]
    pub link_desc: String,
    #[serde(rename = "linkUrl", default)]
    pub link_url: String,
    #[serde(rename = "miniProgramTitle", default)]
    pub mini_program_title: String,
    #[serde(rename = "miniProgramPicMediaId", default)]
    pub mini_program_pic_media_id: String,
    #[serde(rename = "miniProgramAppId", default)]
    pub mini_program_app_id: String,
    #[serde(rename = "miniProgramPage", default)]
    pub mini_program_page: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum Type {
    #[serde(rename = "1")]
    #[default]
    Single,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum Scene {
    #[serde(rename = "1")]
    #[default]
    Miniprogram,
}

impl WxCpContactWayInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpContactWayInfo 解析失败: {e}"))
    }
}

impl WxCpContactWayInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpContactWayInfo 序列化失败: {e}"))
    }
}
