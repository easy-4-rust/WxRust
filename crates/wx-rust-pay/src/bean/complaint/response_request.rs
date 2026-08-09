//! 对应 Java `com.github.binarywang.wxpay.bean.complaint.ResponseRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResponseRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complaint_id"
    )]
    pub complaint_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complainted_mchid"
    )]
    pub complainted_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "response_content"
    )]
    pub response_content: Option<String>,
    #[serde(default, rename = "response_images")]
    pub response_images: Vec<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jump_url")]
    pub jump_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "jump_url_text"
    )]
    pub jump_url_text: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mini_program_jump_info"
    )]
    pub mini_program_jump_info: Option<MiniProgramJumpInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MiniProgramJumpInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "path")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "text")]
    pub text: Option<String>,
}
