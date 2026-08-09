//! 对应 Java `me.chanjar.weixin.cp.bean.message.TemplateCardMessage.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateCardMessage {
    #[serde(rename = "userids", default)]
    pub userids: Vec<String>,
    #[serde(rename = "partyids", default)]
    pub partyids: Vec<i32>,
    #[serde(rename = "tagids", default)]
    pub tagids: Vec<i32>,
    #[serde(rename = "atall", default)]
    pub atall: i32,
    #[serde(rename = "agentid", default)]
    pub agentid: i32,
    #[serde(rename = "response_code", default)]
    pub response_code: String,
    #[serde(rename = "enable_id_trans", default)]
    pub enable_id_trans: i32,
    #[serde(rename = "template_card", default)]
    pub template_card: TemplateCardDTO,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateCardDTO {
    #[serde(rename = "card_type", default)]
    pub card_type: String,
    #[serde(rename = "source", default)]
    pub source: SourceDTO,
    #[serde(rename = "main_title", default)]
    pub main_title: MainTitleDTO,
    #[serde(rename = "select_list", default)]
    pub select_list: Vec<SelectListDTO>,
    #[serde(rename = "submit_button", default)]
    pub submit_button: SubmitButtonDTO,
    #[serde(rename = "replace_text", default)]
    pub replace_text: String,
    #[serde(rename = "checkbox", default)]
    pub checkbox: CheckboxDTO,
    #[serde(rename = "action_menu", default)]
    pub action_menu: ActionMenuDTO,
    #[serde(rename = "quote_area", default)]
    pub quote_area: QuoteAreaDTO,
    #[serde(rename = "sub_title_text", default)]
    pub sub_title_text: String,
    #[serde(rename = "horizontal_content_list", default)]
    pub horizontal_content_list: Vec<HorizontalContentListDTO>,
    #[serde(rename = "card_action", default)]
    pub card_action: CardActionDTO,
    #[serde(rename = "button_selection", default)]
    pub button_selection: ButtonSelectionDTO,
    #[serde(rename = "button_list", default)]
    pub button_list: Vec<ButtonListDTO>,
    #[serde(rename = "image_text_area", default)]
    pub image_text_area: ImageTextAreaDTO,
    #[serde(rename = "card_image", default)]
    pub card_image: CardImageDTO,
    #[serde(rename = "vertical_content_list", default)]
    pub vertical_content_list: Vec<MainTitleDTO>,
    #[serde(rename = "jump_list", default)]
    pub jump_list: Vec<JumpListDTO>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SourceDTO {
    #[serde(rename = "icon_url", default)]
    pub icon_url: String,
    #[serde(rename = "desc", default)]
    pub desc: String,
    #[serde(rename = "desc_color", default)]
    pub desc_color: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ActionMenuDTO {
    #[serde(rename = "desc", default)]
    pub desc: String,
    #[serde(rename = "action_list", default)]
    pub action_list: Vec<crate::bean::message::template_card_message::SubmitButtonDTO>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuoteAreaDTO {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "quote_text", default)]
    pub quote_text: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CardActionDTO {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "pagepath", default)]
    pub pagepath: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ButtonSelectionDTO {
    #[serde(rename = "question_key", default)]
    pub question_key: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "option_list", default)]
    pub option_list: Vec<crate::bean::message::template_card_message::OptionListDTO>,
    #[serde(rename = "selected_id", default)]
    pub selected_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HorizontalContentListDTO {
    #[serde(rename = "keyname", default)]
    pub keyname: String,
    #[serde(rename = "value", default)]
    pub value: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "media_id", default)]
    pub media_id: String,
    #[serde(rename = "userid", default)]
    pub userid: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ButtonListDTO {
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "style", default)]
    pub style: i32,
    #[serde(rename = "key", default)]
    pub key: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CheckboxDTO {
    #[serde(rename = "question_key", default)]
    pub question_key: String,
    #[serde(rename = "option_list", default)]
    pub option_list: Vec<OptionListDTO>,
    #[serde(rename = "disable", default)]
    pub disable: bool,
    #[serde(rename = "mode", default)]
    pub mode: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OptionListDTO {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "is_checked", default)]
    pub is_checked: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MainTitleDTO {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "desc", default)]
    pub desc: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubmitButtonDTO {
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "key", default)]
    pub key: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SelectListDTO {
    #[serde(rename = "question_key", default)]
    pub question_key: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "selected_id", default)]
    pub selected_id: String,
    #[serde(rename = "disable", default)]
    pub disable: bool,
    #[serde(rename = "option_list", default)]
    pub option_list: Vec<SelectListDTOOptionListDTO>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SelectListDTOOptionListDTO {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "text", default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ImageTextAreaDTO {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "desc", default)]
    pub desc: String,
    #[serde(rename = "image_url", default)]
    pub image_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CardImageDTO {
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "aspect_ratio", default)]
    pub aspect_ratio: f64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JumpListDTO {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "pagepath", default)]
    pub pagepath: String,
}

impl TemplateCardMessage {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("TemplateCardMessage 序列化失败: {e}"))
    }
}
