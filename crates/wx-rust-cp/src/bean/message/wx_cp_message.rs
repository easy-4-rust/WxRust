//! 消息（企业微信应用消息）。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpMessage`。Java 以
//! `toJson()` 手工组装 JsonObject（非 Gson 反射），线格式以
//! `WxCpMessageTest` golden 为准：
//!
//! - 顶层键序：`agentid`(有值) → `touser` → `msgtype`(恒有) → `toparty` →
//!   `totag` → `enable_id_trans` → `enable_duplicate_check` →
//!   `duplicate_check_interval` → 消息体子对象 → `safe`；
//! - 消息体按 msgtype 分派（text/textcard/image/file/voice/video/news/mpnews/
//!   markdown/taskcard/miniprogram_notice/template_card）；
//! - StringUtils.isNotBlank 判定（非 null 且非全空白），null 字段省略
//!   （Gson `addProperty` null 语义）。

use std::collections::HashMap;

use crate::bean::article::{MpnewsArticle, NewArticle};
use crate::bean::taskcard::TaskCardButton;
use crate::bean::templatecard::{
    ActionMenuItem, CheckboxOption, HorizontalContent, MultipleSelect, QuoteArea,
    TemplateCardButton, TemplateCardButtonSelection, TemplateCardImageTextArea, TemplateCardJump,
    VerticalContent,
};
use crate::message::messagebuilder::{
    FileBuilder, ImageBuilder, MarkdownMsgBuilder, MiniProgramNoticeMsgBuilder, MpnewsBuilder,
    NewsBuilder, TaskCardBuilder, TemplateCardBuilder, TextBuilder, TextCardBuilder, VideoBuilder,
    VoiceBuilder,
};

/// 是否非空白（对应 Java `StringUtils.isNotBlank`）。
pub(crate) fn nb(v: &Option<String>) -> bool {
    v.as_deref().map(|s| !s.trim().is_empty()).unwrap_or(false)
}

/// 字符串是否非空白（生成 bean 的 String 字段为非 Option 形态）。
pub(crate) fn nb_str(s: &str) -> bool {
    !s.trim().is_empty()
}

/// Option<String> -> JSON 值（None -> Null，对应 Gson `addProperty` null 语义）。
pub(crate) fn opt_json(v: &Option<String>) -> serde_json::Value {
    v.as_deref()
        .map(serde_json::Value::from)
        .unwrap_or(serde_json::Value::Null)
}

/// 消息。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WxCpMessage {
    /// 指定接收消息的成员（成员ID列表，多个接收者用‘|’分隔）。
    pub to_user: Option<String>,
    /// 指定接收消息的部门（部门ID列表，多个接收者用‘|’分隔）。
    pub to_party: Option<String>,
    /// 指定接收消息的标签（标签ID列表，多个接收者用‘|’分隔）。
    pub to_tag: Option<String>,
    /// 企业应用的 id。
    pub agent_id: Option<i32>,
    /// 消息类型（text/image/voice/video/file/textcard/news/mpnews/markdown/
    /// template_card）。
    pub msg_type: Option<String>,
    /// 消息内容。
    pub content: Option<String>,
    /// 媒体文件 id。
    pub media_id: Option<String>,
    /// 图文消息缩略图的 media_id。
    pub thumb_media_id: Option<String>,
    /// 标题。
    pub title: Option<String>,
    /// 描述。
    pub description: Option<String>,
    /// 音乐链接。
    pub music_url: Option<String>,
    /// 高质量音乐链接。
    pub hq_music_url: Option<String>,
    /// 是否保密消息（0 可对外分享 1 不能分享且内容显示水印 2 仅限在企业内分享）。
    pub safe: Option<String>,
    /// 点击后跳转的链接。
    pub url: Option<String>,
    /// 按钮文字。
    pub btn_txt: Option<String>,
    /// 图文消息（news），一个图文消息支持1到8条图文。
    pub articles: Vec<NewArticle>,
    /// 图文消息（mpnews），一个图文消息支持1到8条图文。
    pub mpnews_articles: Vec<MpnewsArticle>,
    /// 小程序 appid。
    pub app_id: Option<String>,
    /// 点击消息卡片后的小程序页面。
    pub page: Option<String>,
    /// 是否放大第一个 content_item。
    pub emphasis_first_item: Option<bool>,
    /// 消息内容键值对，最多允许10个item。
    pub content_items: HashMap<String, String>,
    /// 是否开启 id 转译。
    pub enable_id_trans: bool,
    /// 是否开启重复消息检查。
    pub enable_duplicate_check: bool,
    /// 重复消息检查的时间间隔（默认 1800s，最大不超过4小时）。
    pub duplicate_check_interval: Option<i32>,
    /// 任务卡片任务 id。
    pub task_id: Option<String>,
    /// 任务卡片按钮列表。
    pub task_buttons: Vec<TaskCardButton>,
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
    /// 操作列表（长度 [1, 3]）。
    pub action_menu_action_list: Vec<ActionMenuItem>,
    /// 一级标题。
    pub main_title_title: Option<String>,
    /// 标题辅助信息。
    pub main_title_desc: Option<String>,
    /// 左图右文样式。
    pub image_text_area: Option<TemplateCardImageTextArea>,
    /// 图片的 url。
    pub card_image_url: Option<String>,
    /// 图片的宽高比（<2.25 >1.3，默认1.3）。
    pub card_image_aspect_ratio: Option<f32>,
    /// 关键数据样式的数据内容。
    pub emphasis_content_title: Option<String>,
    /// 关键数据样式的数据描述内容。
    pub emphasis_content_desc: Option<String>,
    /// 二级普通文本。
    pub sub_title_text: Option<String>,
    /// 卡片二级垂直内容（长度不超过4）。
    pub vertical_contents: Vec<VerticalContent>,
    /// 二级标题+文本列表（长度不超过6）。
    pub horizontal_contents: Vec<HorizontalContent>,
    /// 跳转指引样式的列表（长度不超过3）。
    pub jumps: Vec<TemplateCardJump>,
    /// 整体卡片的点击跳转事件类型（1 跳转 url 2 打开小程序）。
    pub card_action_type: Option<i32>,
    /// 跳转事件的 url（type 1 时必填）。
    pub card_action_url: Option<String>,
    /// 跳转事件的小程序 appid（type 2 时必填）。
    pub card_action_appid: Option<String>,
    /// 跳转事件的小程序 pagepath（type 2 时选填）。
    pub card_action_pagepath: Option<String>,
    /// 按钮交互型卡片的 button_selection。
    pub button_selection: Option<TemplateCardButtonSelection>,
    /// 按钮列表（长度不超过6）。
    pub buttons: Vec<TemplateCardButton>,
    /// 选择题 key 值。
    pub checkbox_question_key: Option<String>,
    /// 选择题模式（单选 0 多选 1）。
    pub checkbox_mode: Option<i32>,
    /// 选项 list（1-20 个）。
    pub options: Vec<CheckboxOption>,
    /// 提交按钮文案。
    pub submit_button_text: Option<String>,
    /// 提交按钮的 key。
    pub submit_button_key: Option<String>,
    /// 下拉式的选择器列表（最多 3 个）。
    pub selects: Vec<MultipleSelect>,
    /// 引用文献样式。
    pub quote_area: Option<QuoteArea>,
}

impl WxCpMessage {
    /// 获得文本消息 builder（对应 Java `TEXT()`）。
    pub fn text() -> TextBuilder {
        TextBuilder::new()
    }

    /// 获得文本卡片消息 builder（对应 Java `TEXTCARD()`）。
    pub fn textcard() -> TextCardBuilder {
        TextCardBuilder::new()
    }

    /// 获得图片消息 builder（对应 Java `IMAGE()`）。
    pub fn image() -> ImageBuilder {
        ImageBuilder::new()
    }

    /// 获得语音消息 builder（对应 Java `VOICE()`）。
    pub fn voice() -> VoiceBuilder {
        VoiceBuilder::new()
    }

    /// 获得视频消息 builder（对应 Java `VIDEO()`）。
    pub fn video() -> VideoBuilder {
        VideoBuilder::new()
    }

    /// 获得图文消息 builder（对应 Java `NEWS()`）。
    pub fn news() -> NewsBuilder {
        NewsBuilder::new()
    }

    /// 获得 mpnews 图文消息 builder（对应 Java `MPNEWS()`）。
    pub fn mpnews() -> MpnewsBuilder {
        MpnewsBuilder::new()
    }

    /// 获得 markdown 消息 builder（对应 Java `MARKDOWN()`）。
    pub fn markdown() -> MarkdownMsgBuilder {
        MarkdownMsgBuilder::new()
    }

    /// 获得文件消息 builder（对应 Java `FILE()`）。
    pub fn file() -> FileBuilder {
        FileBuilder::new()
    }

    /// 获得任务卡片消息 builder（对应 Java `TASKCARD()`）。
    pub fn taskcard() -> TaskCardBuilder {
        TaskCardBuilder::new()
    }

    /// 获得模板卡片消息 builder（对应 Java `TEMPLATECARD()`）。
    pub fn templatecard() -> TemplateCardBuilder {
        TemplateCardBuilder::new()
    }

    /// 获得小程序通知消息 builder（对应 Java `MINIPROGRAM_NOTICE()`）。
    pub fn miniprogram_notice() -> MiniProgramNoticeMsgBuilder {
        MiniProgramNoticeMsgBuilder::new()
    }

    /// 序列化为 JSON（对应 Java `toJson()`，键序与 golden 一致）。
    pub fn to_json(&self) -> String {
        let mut message = serde_json::Map::new();
        if let Some(agent_id) = self.agent_id {
            message.insert("agentid".to_string(), serde_json::json!(agent_id));
        }
        if nb(&self.to_user) {
            message.insert("touser".to_string(), serde_json::json!(self.to_user));
        }
        message.insert("msgtype".to_string(), opt_json(&self.msg_type));
        if nb(&self.to_party) {
            message.insert("toparty".to_string(), serde_json::json!(self.to_party));
        }
        if nb(&self.to_tag) {
            message.insert("totag".to_string(), serde_json::json!(self.to_tag));
        }
        if self.enable_id_trans {
            message.insert("enable_id_trans".to_string(), serde_json::json!(1));
        }
        if self.enable_duplicate_check {
            message.insert("enable_duplicate_check".to_string(), serde_json::json!(1));
        }
        if let Some(interval) = self.duplicate_check_interval {
            message.insert(
                "duplicate_check_interval".to_string(),
                serde_json::json!(interval),
            );
        }
        self.handle_msg_type(&mut message);
        if nb(&self.safe) {
            message.insert("safe".to_string(), serde_json::json!(self.safe));
        }
        serde_json::Value::Object(message).to_string()
    }

    /// 消息体分派（对应 Java `handleMsgType(JsonObject)`）。
    fn handle_msg_type(&self, message: &mut serde_json::Map<String, serde_json::Value>) {
        match self.msg_type.as_deref() {
            Some("text") => {
                let mut text = serde_json::Map::new();
                text.insert("content".to_string(), opt_json(&self.content));
                message.insert("text".to_string(), serde_json::Value::Object(text));
            }
            Some("markdown") => {
                let mut text = serde_json::Map::new();
                text.insert("content".to_string(), opt_json(&self.content));
                message.insert("markdown".to_string(), serde_json::Value::Object(text));
            }
            Some("textcard") => {
                let mut text = serde_json::Map::new();
                text.insert("title".to_string(), opt_json(&self.title));
                text.insert("description".to_string(), opt_json(&self.description));
                text.insert("url".to_string(), opt_json(&self.url));
                text.insert("btntxt".to_string(), opt_json(&self.btn_txt));
                message.insert("textcard".to_string(), serde_json::Value::Object(text));
            }
            Some("image") | Some("file") | Some("voice") => {
                let mut media = serde_json::Map::new();
                media.insert("media_id".to_string(), opt_json(&self.media_id));
                message.insert(
                    self.msg_type.as_deref().unwrap_or_default().to_string(),
                    serde_json::Value::Object(media),
                );
            }
            Some("video") => {
                let mut video = serde_json::Map::new();
                video.insert("media_id".to_string(), opt_json(&self.media_id));
                video.insert("thumb_media_id".to_string(), opt_json(&self.thumb_media_id));
                video.insert("title".to_string(), opt_json(&self.title));
                video.insert("description".to_string(), opt_json(&self.description));
                message.insert("video".to_string(), serde_json::Value::Object(video));
            }
            Some("news") => {
                let mut news = serde_json::Map::new();
                let articles = self
                    .articles
                    .iter()
                    .map(|article| {
                        serde_json::json!({
                            "title": article.title,
                            "description": article.description,
                            "url": article.url,
                            "picurl": article.pic_url,
                            "appid": article.appid,
                            "pagepath": article.pagepath,
                        })
                    })
                    .collect::<Vec<_>>();
                news.insert("articles".to_string(), serde_json::Value::Array(articles));
                message.insert("news".to_string(), serde_json::Value::Object(news));
            }
            Some("mpnews") => {
                let mut news = serde_json::Map::new();
                if let Some(media_id) = self.media_id.as_deref() {
                    news.insert("media_id".to_string(), serde_json::json!(media_id));
                } else {
                    let articles = self
                        .mpnews_articles
                        .iter()
                        .map(|article| {
                            serde_json::json!({
                                "title": article.title,
                                "thumb_media_id": article.thumb_media_id,
                                "author": article.author,
                                "content_source_url": article.content_source_url,
                                "content": article.content,
                                "digest": article.digest,
                                "show_cover_pic": article.show_cover_pic,
                            })
                        })
                        .collect::<Vec<_>>();
                    news.insert("articles".to_string(), serde_json::Value::Array(articles));
                }
                message.insert("mpnews".to_string(), serde_json::Value::Object(news));
            }
            Some("taskcard") => {
                let mut text = serde_json::Map::new();
                text.insert("title".to_string(), opt_json(&self.title));
                text.insert("description".to_string(), opt_json(&self.description));
                if nb(&self.url) {
                    text.insert("url".to_string(), serde_json::json!(self.url));
                }
                text.insert("task_id".to_string(), opt_json(&self.task_id));
                let buttons = self
                    .task_buttons
                    .iter()
                    .map(btn_to_json)
                    .collect::<Vec<_>>();
                text.insert("btn".to_string(), serde_json::Value::Array(buttons));
                message.insert("taskcard".to_string(), serde_json::Value::Object(text));
            }
            Some("miniprogram_notice") => {
                let mut notice = serde_json::Map::new();
                notice.insert("appid".to_string(), opt_json(&self.app_id));
                notice.insert("page".to_string(), opt_json(&self.page));
                notice.insert("description".to_string(), opt_json(&self.description));
                notice.insert("title".to_string(), opt_json(&self.title));
                notice.insert(
                    "emphasis_first_item".to_string(),
                    self.emphasis_first_item
                        .map(|v| serde_json::json!(v))
                        .unwrap_or(serde_json::Value::Null),
                );
                let content = self
                    .content_items
                    .iter()
                    .map(|(k, v)| serde_json::json!({ "key": k, "value": v }))
                    .collect::<Vec<_>>();
                notice.insert(
                    "content_item".to_string(),
                    serde_json::Value::Array(content),
                );
                message.insert(
                    "miniprogram_notice".to_string(),
                    serde_json::Value::Object(notice),
                );
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

                if let Some(image_text_area) = &self.image_text_area {
                    template.insert(
                        "image_text_area".to_string(),
                        template_card_image_text_area_to_json(image_text_area),
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

                if nb(&self.task_id) {
                    template.insert("task_id".to_string(), serde_json::json!(self.task_id));
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

                if let Some(button_selection) = &self.button_selection {
                    template.insert(
                        "button_selection".to_string(),
                        button_selection_to_json(button_selection),
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

/// 任务卡片按钮 JSON（对应 Java `btn2Json`）。
pub(crate) fn btn_to_json(button: &TaskCardButton) -> serde_json::Value {
    let mut b = serde_json::Map::new();
    b.insert(
        "key".to_string(),
        button
            .key
            .as_deref()
            .map(serde_json::Value::from)
            .unwrap_or(serde_json::Value::Null),
    );
    b.insert(
        "name".to_string(),
        button
            .name
            .as_deref()
            .map(serde_json::Value::from)
            .unwrap_or(serde_json::Value::Null),
    );
    if nb(&button.replace_name) {
        b.insert(
            "replace_name".to_string(),
            serde_json::json!(button.replace_name),
        );
    }
    if nb(&button.color) {
        b.insert("color".to_string(), serde_json::json!(button.color));
    }
    if let Some(bold) = button.bold {
        b.insert("is_bold".to_string(), serde_json::json!(bold));
    }
    serde_json::Value::Object(b)
}

/// 操作项 JSON（对应 Java `ActionMenuItem.toJson`：text/key 恒输出）。
pub(crate) fn action_menu_item_to_json(item: &ActionMenuItem) -> serde_json::Value {
    serde_json::json!({
        "text": item.text,
        "key": item.key,
    })
}

/// 左图右文样式 JSON（对应 Java `TemplateCardImageTextArea.toJson`）。
pub(crate) fn template_card_image_text_area_to_json(
    area: &TemplateCardImageTextArea,
) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    if area.r#type != 0 {
        o.insert("type".to_string(), serde_json::json!(area.r#type));
    }
    if nb_str(&area.url) {
        o.insert("url".to_string(), serde_json::json!(area.url));
    }
    if nb_str(&area.title) {
        o.insert("title".to_string(), serde_json::json!(area.title));
    }
    if nb_str(&area.desc) {
        o.insert("desc".to_string(), serde_json::json!(area.desc));
    }
    if nb_str(&area.image_url) {
        o.insert("image_url".to_string(), serde_json::json!(area.image_url));
    }
    serde_json::Value::Object(o)
}

/// 垂直内容 JSON（对应 Java `VerticalContent.toJson`：title 恒输出）。
pub(crate) fn vertical_content_to_json(content: &VerticalContent) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    o.insert("title".to_string(), serde_json::json!(content.title));
    if nb_str(&content.desc) {
        o.insert("desc".to_string(), serde_json::json!(content.desc));
    }
    serde_json::Value::Object(o)
}

/// 二级标题+文本 JSON（对应 Java `HorizontalContent.toJson`）。
pub(crate) fn horizontal_content_to_json(content: &HorizontalContent) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    if content.r#type != 0 {
        o.insert("type".to_string(), serde_json::json!(content.r#type));
    }
    o.insert("keyname".to_string(), serde_json::json!(content.keyname));
    if nb_str(&content.value) {
        o.insert("value".to_string(), serde_json::json!(content.value));
    }
    if nb_str(&content.url) {
        o.insert("url".to_string(), serde_json::json!(content.url));
    }
    if nb_str(&content.media_id) {
        o.insert("media_id".to_string(), serde_json::json!(content.media_id));
    }
    if nb_str(&content.userid) {
        o.insert("userid".to_string(), serde_json::json!(content.userid));
    }
    serde_json::Value::Object(o)
}

/// 跳转指引 JSON（对应 Java `TemplateCardJump.toJson`）。
pub(crate) fn jump_to_json(jump: &TemplateCardJump) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    if jump.r#type != 0 {
        o.insert("type".to_string(), serde_json::json!(jump.r#type));
    }
    o.insert("title".to_string(), serde_json::json!(jump.title));
    if nb_str(&jump.url) {
        o.insert("url".to_string(), serde_json::json!(jump.url));
    }
    if nb_str(&jump.appid) {
        o.insert("appid".to_string(), serde_json::json!(jump.appid));
    }
    if nb_str(&jump.pagepath) {
        o.insert("pagepath".to_string(), serde_json::json!(jump.pagepath));
    }
    serde_json::Value::Object(o)
}

/// 按钮选择 JSON（对应 Java `TemplateCardButtonSelection.toJson`）。
pub(crate) fn button_selection_to_json(
    selection: &TemplateCardButtonSelection,
) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    if nb_str(&selection.question_key) {
        o.insert(
            "question_key".to_string(),
            serde_json::json!(selection.question_key),
        );
    }
    if nb_str(&selection.title) {
        o.insert("title".to_string(), serde_json::json!(selection.title));
    }
    if nb_str(&selection.selected_id) {
        o.insert(
            "selected_id".to_string(),
            serde_json::json!(selection.selected_id),
        );
    }
    if !selection.option_list.is_empty() {
        let list = selection
            .option_list
            .iter()
            .map(|option| {
                let mut op = serde_json::Map::new();
                if nb_str(&option.id) {
                    op.insert("id".to_string(), serde_json::json!(option.id));
                }
                if nb_str(&option.text) {
                    op.insert("text".to_string(), serde_json::json!(option.text));
                }
                serde_json::Value::Object(op)
            })
            .collect::<Vec<_>>();
        o.insert("option_list".to_string(), serde_json::Value::Array(list));
    }
    serde_json::Value::Object(o)
}

/// 按钮 JSON（对应 Java `TemplateCardButton.toJson`）。
pub(crate) fn template_button_to_json(button: &TemplateCardButton) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    o.insert("text".to_string(), serde_json::json!(button.text));
    if button.style != 0 {
        o.insert("style".to_string(), serde_json::json!(button.style));
    }
    o.insert("key".to_string(), serde_json::json!(button.key));
    o.insert("type".to_string(), serde_json::json!(button.r#type));
    if !button.url.is_empty() {
        o.insert("url".to_string(), serde_json::json!(button.url));
    }
    serde_json::Value::Object(o)
}

/// 选项 JSON（对应 Java `CheckboxOption.toJson`）。
pub(crate) fn checkbox_option_to_json(option: &CheckboxOption) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    o.insert(
        "id".to_string(),
        option
            .id
            .as_deref()
            .map(serde_json::Value::from)
            .unwrap_or(serde_json::Value::Null),
    );
    o.insert(
        "text".to_string(),
        option
            .text
            .as_deref()
            .map(serde_json::Value::from)
            .unwrap_or(serde_json::Value::Null),
    );
    if let Some(is_checked) = option.is_checked {
        o.insert("is_checked".to_string(), serde_json::json!(is_checked));
    }
    serde_json::Value::Object(o)
}

/// 下拉选择器 JSON（对应 Java `MultipleSelect.toJson`）。
pub(crate) fn multiple_select_to_json(select: &MultipleSelect) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    o.insert(
        "question_key".to_string(),
        serde_json::json!(select.question_key),
    );
    if nb_str(&select.title) {
        o.insert("title".to_string(), serde_json::json!(select.title));
    }
    if nb_str(&select.selected_id) {
        o.insert(
            "selected_id".to_string(),
            serde_json::json!(select.selected_id),
        );
    }
    if !select.options.is_empty() {
        let list = select
            .options
            .iter()
            .map(checkbox_option_to_json)
            .collect::<Vec<_>>();
        o.insert("option_list".to_string(), serde_json::Value::Array(list));
    }
    serde_json::Value::Object(o)
}

/// 引用文献样式 JSON（对应 Java `QuoteArea.toJson`）。
pub(crate) fn quote_area_to_json(quote: &QuoteArea) -> serde_json::Value {
    let mut o = serde_json::Map::new();
    if quote.r#type != 0 {
        o.insert("type".to_string(), serde_json::json!(quote.r#type));
    }
    if nb_str(&quote.url) {
        o.insert("url".to_string(), serde_json::json!(quote.url));
    }
    if nb_str(&quote.appid) {
        o.insert("appid".to_string(), serde_json::json!(quote.appid));
    }
    if nb_str(&quote.pagepath) {
        o.insert("pagepath".to_string(), serde_json::json!(quote.pagepath));
    }
    if nb_str(&quote.title) {
        o.insert("title".to_string(), serde_json::json!(quote.title));
    }
    if nb_str(&quote.quote_text) {
        o.insert(
            "quote_text".to_string(),
            serde_json::json!(quote.quote_text),
        );
    }
    serde_json::Value::Object(o)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Java `WxCpMessageTest.testTextBuild` 线格式 golden。
    #[test]
    fn text_build_golden() {
        let reply = WxCpMessage::text()
            .to_user("OPENID")
            .content("sfsfdsdf")
            .build();
        assert_eq!(
            reply.to_json(),
            "{\"touser\":\"OPENID\",\"msgtype\":\"text\",\"text\":{\"content\":\"sfsfdsdf\"},\"safe\":\"0\"}"
        );
    }

    /// Java `WxCpMessageTest.testImageBuild` 线格式 golden。
    #[test]
    fn image_build_golden() {
        let reply = WxCpMessage::image()
            .to_user("OPENID")
            .media_id("MEDIA_ID")
            .build();
        assert_eq!(
            reply.to_json(),
            "{\"touser\":\"OPENID\",\"msgtype\":\"image\",\"image\":{\"media_id\":\"MEDIA_ID\"},\"safe\":\"0\"}"
        );
    }

    /// Java `WxCpMessageTest.testTextCardBuild` 线格式 golden。
    #[test]
    fn text_card_build_golden() {
        let reply = WxCpMessage::textcard()
            .to_user("OPENID")
            .title("领奖通知")
            .description("<div class=\"gray\">2016年9月26日</div> <div class=\"normal\">恭喜你抽中iPhone 7一台，领奖码：xxxx</div><div class=\"highlight\">请于2016年10月10日前联系行政同事领取</div>")
            .url("http://www.qq.com")
            .btn_txt("更多")
            .build();
        assert_eq!(
            reply.to_json(),
            "{\"touser\":\"OPENID\",\"msgtype\":\"textcard\",\"textcard\":{\"title\":\"领奖通知\",\"description\":\"<div class=\\\"gray\\\">2016年9月26日</div> <div class=\\\"normal\\\">恭喜你抽中iPhone 7一台，领奖码：xxxx</div><div class=\\\"highlight\\\">请于2016年10月10日前联系行政同事领取</div>\",\"url\":\"http://www.qq.com\",\"btntxt\":\"更多\"},\"safe\":\"0\"}"
        );
    }
}
