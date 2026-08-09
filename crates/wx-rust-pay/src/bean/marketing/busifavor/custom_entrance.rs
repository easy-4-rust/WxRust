//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.busifavor.CustomEntrance.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CustomEntrance {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mini_programs_info"
    )]
    pub mini_programs_info: Option<MiniProgramsInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hall_id")]
    pub hall_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "store_id")]
    pub store_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "code_display_mode"
    )]
    pub code_display_mode: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MiniProgramsInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mini_programs_appid"
    )]
    pub mini_programs_appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mini_programs_path"
    )]
    pub mini_programs_path: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "entrance_words"
    )]
    pub entrance_words: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "guiding_words"
    )]
    pub guiding_words: Option<String>,
}
