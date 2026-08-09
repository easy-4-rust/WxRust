//! 群机器人消息（企业微信群机器人）。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpGroupRobotMessage`。Java
//! 以 `toJson()` 手工组装 JsonObject：顶层键序 `msgtype` → `agentid`(有值) →
//! 消息体子对象（text/markdown/markdown_v2/image/news/file/template_card）。

use crate::bean::article::NewArticle;
use crate::bean::message::wx_cp_message::{
    action_menu_item_to_json, checkbox_option_to_json, horizontal_content_to_json, jump_to_json,
    multiple_select_to_json, nb, opt_json, quote_area_to_json, template_button_to_json,
    vertical_content_to_json,
};
use crate::bean::templatecard::{
    ActionMenuItem, CheckboxOption, HorizontalContent, MultipleSelect, QuoteArea,
    TemplateCardButton, TemplateCardJump, VerticalContent,
};

/// 群机器人消息。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpGroupRobotMessage {
    /// 消息类型（text/markdown/markdown_v2/image/news/file/template_card）。
    pub msg_type: Option<String>,
    /// 消息内容。
    pub content: Option<String>,
    /// 被 @ 的成员 userid 列表。
    pub mentioned_list: Vec<String>,
    /// 被 @ 的成员手机号列表。
    pub mentioned_mobile_list: Vec<String>,
    /// 图片的 base64 编码。
    pub base64: Option<String>,
    /// 图片的 md5 值。
    pub md5: Option<String>,
    /// 图文消息（news）。
    pub articles: Vec<NewArticle>,
    /// 媒体文件 id。
    pub media_id: Option<String>,
    /// 企业应用的 id。
    pub agent_id: Option<i32>,
    /// 模板卡片类型（text_notice/news_notice/button_interaction/
    /// vote_interaction/multiple_interaction）。
    pub card_type: Option<String>,
    /// 来源图片的 url。
    pub source_icon_url: Option<String>,
    /// 来源图片的描述。
    pub source_desc: Option<String>,
    /// 来源文字的颜色（0 默认灰色 1 黑色 2 红色 3 绿色）。
    pub source_desc_color: Option<i32>,
    /// 更多操作界面的描述。
    pub action_menu_desc: Option<String>,
    /// 操作列表。
    pub action_menu_action_list: Vec<ActionMenuItem>,
    /// 一级标题。
    pub main_title_title: Option<String>,
    /// 标题辅助信息。
    pub main_title_desc: Option<String>,
    /// 图片的 url。
    pub card_image_url: Option<String>,
    /// 图片的宽高比。
    pub card_image_aspect_ratio: Option<f32>,
    /// 关键数据样式的数据内容。
    pub emphasis_content_title: Option<String>,
    /// 关键数据样式的数据描述内容。
    pub emphasis_content_desc: Option<String>,
    /// 二级普通文本。
    pub sub_title_text: Option<String>,
    /// 卡片二级垂直内容。
    pub vertical_contents: Vec<VerticalContent>,
    /// 二级标题+文本列表。
    pub horizontal_contents: Vec<HorizontalContent>,
    /// 跳转指引样式的列表。
    pub jumps: Vec<TemplateCardJump>,
    /// 整体卡片的点击跳转事件类型。
    pub card_action_type: Option<i32>,
    /// 跳转事件的 url。
    pub card_action_url: Option<String>,
    /// 跳转事件的小程序 appid。
    pub card_action_appid: Option<String>,
    /// 跳转事件的小程序 pagepath。
    pub card_action_pagepath: Option<String>,
    /// 按钮列表。
    pub buttons: Vec<TemplateCardButton>,
    /// 选择题 key 值。
    pub checkbox_question_key: Option<String>,
    /// 选择题模式（单选 0 多选 1）。
    pub checkbox_mode: Option<i32>,
    /// 选项 list。
    pub options: Vec<CheckboxOption>,
    /// 提交按钮文案。
    pub submit_button_text: Option<String>,
    /// 提交按钮的 key。
    pub submit_button_key: Option<String>,
    /// 下拉式的选择器列表。
    pub selects: Vec<MultipleSelect>,
    /// 引用文献样式。
    pub quote_area: Option<QuoteArea>,
}

impl WxCpGroupRobotMessage {
    /// 序列化为 JSON（对应 Java `toJson()`）。
    pub fn to_json(&self) -> String {
        let mut message = serde_json::Map::new();
        message.insert("msgtype".to_string(), opt_json(&self.msg_type));
        if let Some(agent_id) = self.agent_id {
            message.insert("agentid".to_string(), serde_json::json!(agent_id));
        }
        self.handle_msg_type(&mut message);
        serde_json::Value::Object(message).to_string()
    }

    /// 消息体分派（对应 Java `handleMsgType`）。
    fn handle_msg_type(&self, message: &mut serde_json::Map<String, serde_json::Value>) {
        match self.msg_type.as_deref() {
            Some("text") => {
                let mut text = serde_json::Map::new();
                text.insert("content".to_string(), opt_json(&self.content));
                text.insert(
                    "mentioned_list".to_string(),
                    serde_json::Value::Array(
                        self.mentioned_list
                            .iter()
                            .cloned()
                            .map(serde_json::Value::from)
                            .collect(),
                    ),
                );
                text.insert(
                    "mentioned_mobile_list".to_string(),
                    serde_json::Value::Array(
                        self.mentioned_mobile_list
                            .iter()
                            .cloned()
                            .map(serde_json::Value::from)
                            .collect(),
                    ),
                );
                message.insert("text".to_string(), serde_json::Value::Object(text));
            }
            Some("markdown") => {
                let mut text = serde_json::Map::new();
                text.insert("content".to_string(), opt_json(&self.content));
                message.insert("markdown".to_string(), serde_json::Value::Object(text));
            }
            Some("markdown_v2") => {
                let mut text = serde_json::Map::new();
                text.insert("content".to_string(), opt_json(&self.content));
                message.insert("markdown_v2".to_string(), serde_json::Value::Object(text));
            }
            Some("image") => {
                let mut text = serde_json::Map::new();
                text.insert("base64".to_string(), opt_json(&self.base64));
                text.insert("md5".to_string(), opt_json(&self.md5));
                message.insert("image".to_string(), serde_json::Value::Object(text));
            }
            Some("news") => {
                let mut text = serde_json::Map::new();
                let articles = self
                    .articles
                    .iter()
                    .map(|article| {
                        serde_json::json!({
                            "title": article.title,
                            "description": article.description,
                            "url": article.url,
                            "picurl": article.pic_url,
                        })
                    })
                    .collect::<Vec<_>>();
                text.insert("articles".to_string(), serde_json::Value::Array(articles));
                message.insert("news".to_string(), serde_json::Value::Object(text));
            }
            Some("file") => {
                let mut file = serde_json::Map::new();
                file.insert("media_id".to_string(), opt_json(&self.media_id));
                message.insert("file".to_string(), serde_json::Value::Object(file));
            }
            Some("template_card") => {
                let mut template = serde_json::Map::new();
                template.insert("card_type".to_string(), opt_json(&self.card_type));

                if nb(&self.source_icon_url) || nb(&self.source_desc) {
                    let mut source = serde_json::Map::new();
                    if nb(&self.source_icon_url) {
                        source.insert(
                            "icon_url".to_string(),
                            serde_json::json!(self.source_icon_url),
                        );
                    }
                    if nb(&self.source_desc) {
                        source.insert("desc".to_string(), serde_json::json!(self.source_desc));
                    }
                    source.insert(
                        "desc_color".to_string(),
                        self.source_desc_color
                            .map(|v| serde_json::json!(v))
                            .unwrap_or(serde_json::Value::Null),
                    );
                    template.insert("source".to_string(), serde_json::Value::Object(source));
                }

                if nb(&self.action_menu_desc) {
                    let mut action_menu = serde_json::Map::new();
                    action_menu
                        .insert("desc".to_string(), serde_json::json!(self.action_menu_desc));
                    let action_list = self
                        .action_menu_action_list
                        .iter()
                        .map(action_menu_item_to_json)
                        .collect::<Vec<_>>();
                    action_menu.insert(
                        "action_list".to_string(),
                        serde_json::Value::Array(action_list),
                    );
                    template.insert(
                        "action_menu".to_string(),
                        serde_json::Value::Object(action_menu),
                    );
                }

                if nb(&self.main_title_title) || nb(&self.main_title_desc) {
                    let mut main_title = serde_json::Map::new();
                    if nb(&self.main_title_title) {
                        main_title.insert(
                            "title".to_string(),
                            serde_json::json!(self.main_title_title),
                        );
                    }
                    if nb(&self.main_title_desc) {
                        main_title
                            .insert("desc".to_string(), serde_json::json!(self.main_title_desc));
                    }
                    template.insert(
                        "main_title".to_string(),
                        serde_json::Value::Object(main_title),
                    );
                }

                if nb(&self.card_image_url) || self.card_image_aspect_ratio.is_some() {
                    let mut card_image = serde_json::Map::new();
                    if nb(&self.card_image_url) {
                        card_image
                            .insert("url".to_string(), serde_json::json!(self.card_image_url));
                    }
                    if let Some(ratio) = self.card_image_aspect_ratio {
                        card_image.insert("aspect_ratio".to_string(), serde_json::json!(ratio));
                    }
                    template.insert(
                        "card_image".to_string(),
                        serde_json::Value::Object(card_image),
                    );
                }

                if nb(&self.emphasis_content_title) || nb(&self.emphasis_content_desc) {
                    let mut emphasis = serde_json::Map::new();
                    if nb(&self.emphasis_content_title) {
                        emphasis.insert(
                            "title".to_string(),
                            serde_json::json!(self.emphasis_content_title),
                        );
                    }
                    if nb(&self.emphasis_content_desc) {
                        emphasis.insert(
                            "desc".to_string(),
                            serde_json::json!(self.emphasis_content_desc),
                        );
                    }
                    template.insert(
                        "emphasis_content".to_string(),
                        serde_json::Value::Object(emphasis),
                    );
                }

                if nb(&self.sub_title_text) {
                    template.insert(
                        "sub_title_text".to_string(),
                        serde_json::json!(self.sub_title_text),
                    );
                }

                if !self.vertical_contents.is_empty() {
                    let list = self
                        .vertical_contents
                        .iter()
                        .map(vertical_content_to_json)
                        .collect::<Vec<_>>();
                    template.insert(
                        "vertical_content_list".to_string(),
                        serde_json::Value::Array(list),
                    );
                }

                if !self.horizontal_contents.is_empty() {
                    let list = self
                        .horizontal_contents
                        .iter()
                        .map(horizontal_content_to_json)
                        .collect::<Vec<_>>();
                    template.insert(
                        "horizontal_content_list".to_string(),
                        serde_json::Value::Array(list),
                    );
                }

                if !self.jumps.is_empty() {
                    let list = self.jumps.iter().map(jump_to_json).collect::<Vec<_>>();
                    template.insert("jump_list".to_string(), serde_json::Value::Array(list));
                }

                if let Some(action_type) = self.card_action_type {
                    let mut card_action = serde_json::Map::new();
                    card_action.insert("type".to_string(), serde_json::json!(action_type));
                    if nb(&self.card_action_url) {
                        card_action
                            .insert("url".to_string(), serde_json::json!(self.card_action_url));
                    }
                    if nb(&self.card_action_appid) {
                        card_action.insert(
                            "appid".to_string(),
                            serde_json::json!(self.card_action_appid),
                        );
                    }
                    if nb(&self.card_action_pagepath) {
                        card_action.insert(
                            "pagepath".to_string(),
                            serde_json::json!(self.card_action_pagepath),
                        );
                    }
                    template.insert(
                        "card_action".to_string(),
                        serde_json::Value::Object(card_action),
                    );
                }

                if !self.buttons.is_empty() {
                    let list = self
                        .buttons
                        .iter()
                        .map(template_button_to_json)
                        .collect::<Vec<_>>();
                    template.insert("button_list".to_string(), serde_json::Value::Array(list));
                }

                if nb(&self.checkbox_question_key) {
                    let mut checkbox = serde_json::Map::new();
                    checkbox.insert(
                        "question_key".to_string(),
                        serde_json::json!(self.checkbox_question_key),
                    );
                    if let Some(mode) = self.checkbox_mode {
                        checkbox.insert("mode".to_string(), serde_json::json!(mode));
                    }
                    let options = self
                        .options
                        .iter()
                        .map(checkbox_option_to_json)
                        .collect::<Vec<_>>();
                    checkbox.insert("option_list".to_string(), serde_json::Value::Array(options));
                    template.insert("checkbox".to_string(), serde_json::Value::Object(checkbox));
                }

                if nb(&self.submit_button_text) || nb(&self.submit_button_key) {
                    let mut submit_button = serde_json::Map::new();
                    if nb(&self.submit_button_text) {
                        submit_button.insert(
                            "text".to_string(),
                            serde_json::json!(self.submit_button_text),
                        );
                    }
                    if nb(&self.submit_button_key) {
                        submit_button
                            .insert("key".to_string(), serde_json::json!(self.submit_button_key));
                    }
                    template.insert(
                        "submit_button".to_string(),
                        serde_json::Value::Object(submit_button),
                    );
                }

                if !self.selects.is_empty() {
                    let list = self
                        .selects
                        .iter()
                        .map(multiple_select_to_json)
                        .collect::<Vec<_>>();
                    template.insert("select_list".to_string(), serde_json::Value::Array(list));
                }

                if let Some(quote_area) = &self.quote_area {
                    template.insert("quote_area".to_string(), quote_area_to_json(quote_area));
                }

                message.insert(
                    "template_card".to_string(),
                    serde_json::Value::Object(template),
                );
            }
            _ => {
                // 未知类型不做任何处理（对应 Java default 分支）
            }
        }
    }
}
