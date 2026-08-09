//! 模板卡片消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.messagebuilder.TemplateCardBuilder`
//! （msgType 固定为 `template_card`；safe 置 null，对应 Java
//! `m.setSafe(null)`）。

use crate::bean::message::WxCpMessage;
use crate::bean::templatecard::{
    ActionMenuItem, CheckboxOption, HorizontalContent, MultipleSelect, QuoteArea,
    TemplateCardButton, TemplateCardButtonSelection, TemplateCardImageTextArea, TemplateCardJump,
    VerticalContent,
};
use crate::message::messagebuilder::BaseBuilder;

/// 模板卡片消息 builder。
#[derive(Debug, Clone, Default)]
pub struct TemplateCardBuilder {
    base: BaseBuilder,
    card_type: Option<String>,
    source_icon_url: Option<String>,
    source_desc: Option<String>,
    source_desc_color: Option<i32>,
    action_menu_desc: Option<String>,
    action_menu_action_list: Vec<ActionMenuItem>,
    main_title_title: Option<String>,
    main_title_desc: Option<String>,
    card_image_url: Option<String>,
    card_image_aspect_ratio: Option<f32>,
    emphasis_content_title: Option<String>,
    emphasis_content_desc: Option<String>,
    sub_title_text: Option<String>,
    vertical_contents: Vec<VerticalContent>,
    horizontal_contents: Vec<HorizontalContent>,
    jumps: Vec<TemplateCardJump>,
    image_text_area: Option<TemplateCardImageTextArea>,
    card_action_type: Option<i32>,
    card_action_url: Option<String>,
    card_action_appid: Option<String>,
    card_action_pagepath: Option<String>,
    task_id: Option<String>,
    button_selection: Option<TemplateCardButtonSelection>,
    buttons: Vec<TemplateCardButton>,
    checkbox_question_key: Option<String>,
    checkbox_mode: Option<i32>,
    options: Vec<CheckboxOption>,
    submit_button_text: Option<String>,
    submit_button_key: Option<String>,
    selects: Vec<MultipleSelect>,
    quote_area: Option<QuoteArea>,
}

impl TemplateCardBuilder {
    /// 构建空 builder（msgType 固定为 template_card）。
    pub fn new() -> Self {
        Self {
            base: BaseBuilder {
                msg_type: Some("template_card".to_string()),
                ..Default::default()
            },
            card_type: None,
            source_icon_url: None,
            source_desc: None,
            source_desc_color: None,
            action_menu_desc: None,
            action_menu_action_list: Vec::new(),
            main_title_title: None,
            main_title_desc: None,
            card_image_url: None,
            card_image_aspect_ratio: None,
            emphasis_content_title: None,
            emphasis_content_desc: None,
            sub_title_text: None,
            vertical_contents: Vec::new(),
            horizontal_contents: Vec::new(),
            jumps: Vec::new(),
            image_text_area: None,
            card_action_type: None,
            card_action_url: None,
            card_action_appid: None,
            card_action_pagepath: None,
            task_id: None,
            button_selection: None,
            buttons: Vec::new(),
            checkbox_question_key: None,
            checkbox_mode: None,
            options: Vec::new(),
            submit_button_text: None,
            submit_button_key: None,
            selects: Vec::new(),
            quote_area: None,
        }
    }

    /// 设置模板卡片类型（对应 Java `cardType`）。
    pub fn card_type(mut self, card_type: impl Into<String>) -> Self {
        self.card_type = Some(card_type.into());
        self
    }

    /// 设置来源图片的 url。
    pub fn source_icon_url(mut self, source_icon_url: impl Into<String>) -> Self {
        self.source_icon_url = Some(source_icon_url.into());
        self
    }

    /// 设置来源图片的描述。
    pub fn source_desc(mut self, source_desc: impl Into<String>) -> Self {
        self.source_desc = Some(source_desc.into());
        self
    }

    /// 设置来源文字的颜色（0 默认灰色 1 黑色 2 红色 3 绿色）。
    pub fn source_desc_color(mut self, source_desc_color: i32) -> Self {
        self.source_desc_color = Some(source_desc_color);
        self
    }

    /// 设置更多操作界面的描述。
    pub fn action_menu_desc(mut self, action_menu_desc: impl Into<String>) -> Self {
        self.action_menu_desc = Some(action_menu_desc.into());
        self
    }

    /// 设置操作列表。
    pub fn action_menu_action_list(mut self, action_menu_action_list: Vec<ActionMenuItem>) -> Self {
        self.action_menu_action_list = action_menu_action_list;
        self
    }

    /// 设置一级标题。
    pub fn main_title_title(mut self, main_title_title: impl Into<String>) -> Self {
        self.main_title_title = Some(main_title_title.into());
        self
    }

    /// 设置标题辅助信息。
    pub fn main_title_desc(mut self, main_title_desc: impl Into<String>) -> Self {
        self.main_title_desc = Some(main_title_desc.into());
        self
    }

    /// 设置图片的 url。
    pub fn card_image_url(mut self, card_image_url: impl Into<String>) -> Self {
        self.card_image_url = Some(card_image_url.into());
        self
    }

    /// 设置图片的宽高比。
    pub fn card_image_aspect_ratio(mut self, card_image_aspect_ratio: f32) -> Self {
        self.card_image_aspect_ratio = Some(card_image_aspect_ratio);
        self
    }

    /// 设置关键数据样式的数据内容。
    pub fn emphasis_content_title(mut self, emphasis_content_title: impl Into<String>) -> Self {
        self.emphasis_content_title = Some(emphasis_content_title.into());
        self
    }

    /// 设置关键数据样式的数据描述内容。
    pub fn emphasis_content_desc(mut self, emphasis_content_desc: impl Into<String>) -> Self {
        self.emphasis_content_desc = Some(emphasis_content_desc.into());
        self
    }

    /// 设置二级普通文本。
    pub fn sub_title_text(mut self, sub_title_text: impl Into<String>) -> Self {
        self.sub_title_text = Some(sub_title_text.into());
        self
    }

    /// 设置卡片二级垂直内容。
    pub fn vertical_contents(mut self, vertical_contents: Vec<VerticalContent>) -> Self {
        self.vertical_contents = vertical_contents;
        self
    }

    /// 设置二级标题+文本列表。
    pub fn horizontal_contents(mut self, horizontal_contents: Vec<HorizontalContent>) -> Self {
        self.horizontal_contents = horizontal_contents;
        self
    }

    /// 设置跳转指引样式的列表。
    pub fn jumps(mut self, jumps: Vec<TemplateCardJump>) -> Self {
        self.jumps = jumps;
        self
    }

    /// 设置左图右文样式。
    pub fn image_text_area(mut self, image_text_area: TemplateCardImageTextArea) -> Self {
        self.image_text_area = Some(image_text_area);
        self
    }

    /// 设置整体卡片的点击跳转事件类型。
    pub fn card_action_type(mut self, card_action_type: i32) -> Self {
        self.card_action_type = Some(card_action_type);
        self
    }

    /// 设置跳转事件的 url。
    pub fn card_action_url(mut self, card_action_url: impl Into<String>) -> Self {
        self.card_action_url = Some(card_action_url.into());
        self
    }

    /// 设置跳转事件的小程序 appid。
    pub fn card_action_appid(mut self, card_action_appid: impl Into<String>) -> Self {
        self.card_action_appid = Some(card_action_appid.into());
        self
    }

    /// 设置跳转事件的小程序 pagepath。
    pub fn card_action_pagepath(mut self, card_action_pagepath: impl Into<String>) -> Self {
        self.card_action_pagepath = Some(card_action_pagepath.into());
        self
    }

    /// 设置任务 id。
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.task_id = Some(task_id.into());
        self
    }

    /// 设置按钮交互型卡片的 button_selection。
    pub fn button_selection(mut self, button_selection: TemplateCardButtonSelection) -> Self {
        self.button_selection = Some(button_selection);
        self
    }

    /// 设置按钮列表。
    pub fn buttons(mut self, buttons: Vec<TemplateCardButton>) -> Self {
        self.buttons = buttons;
        self
    }

    /// 设置选择题 key 值。
    pub fn checkbox_question_key(mut self, checkbox_question_key: impl Into<String>) -> Self {
        self.checkbox_question_key = Some(checkbox_question_key.into());
        self
    }

    /// 设置选择题模式（单选 0 多选 1）。
    pub fn checkbox_mode(mut self, checkbox_mode: i32) -> Self {
        self.checkbox_mode = Some(checkbox_mode);
        self
    }

    /// 设置选项 list。
    pub fn options(mut self, options: Vec<CheckboxOption>) -> Self {
        self.options = options;
        self
    }

    /// 设置提交按钮文案。
    pub fn submit_button_text(mut self, submit_button_text: impl Into<String>) -> Self {
        self.submit_button_text = Some(submit_button_text.into());
        self
    }

    /// 设置提交按钮的 key。
    pub fn submit_button_key(mut self, submit_button_key: impl Into<String>) -> Self {
        self.submit_button_key = Some(submit_button_key.into());
        self
    }

    /// 设置下拉式的选择器列表。
    pub fn selects(mut self, selects: Vec<MultipleSelect>) -> Self {
        self.selects = selects;
        self
    }

    /// 设置引用文献样式。
    pub fn quote_area(mut self, quote_area: QuoteArea) -> Self {
        self.quote_area = Some(quote_area);
        self
    }

    /// 设置企业应用的 id。
    pub fn agent_id(mut self, agent_id: i32) -> Self {
        self.base = self.base.agent_id(agent_id);
        self
    }

    /// 设置接收消息的成员。
    pub fn to_user(mut self, to_user: impl Into<String>) -> Self {
        self.base = self.base.to_user(to_user);
        self
    }

    /// 设置接收消息的部门。
    pub fn to_party(mut self, to_party: impl Into<String>) -> Self {
        self.base = self.base.to_party(to_party);
        self
    }

    /// 设置接收消息的标签。
    pub fn to_tag(mut self, to_tag: impl Into<String>) -> Self {
        self.base = self.base.to_tag(to_tag);
        self
    }

    /// 构建消息（safe 置 null）。
    pub fn build(self) -> WxCpMessage {
        let mut m = self.base.build_base();
        m.safe = None;
        m.card_type = self.card_type;
        m.source_icon_url = self.source_icon_url;
        m.source_desc = self.source_desc;
        m.source_desc_color = self.source_desc_color;
        m.action_menu_desc = self.action_menu_desc;
        m.action_menu_action_list = self.action_menu_action_list;
        m.main_title_title = self.main_title_title;
        m.main_title_desc = self.main_title_desc;
        m.image_text_area = self.image_text_area;
        m.card_image_url = self.card_image_url;
        m.card_image_aspect_ratio = self.card_image_aspect_ratio;
        m.emphasis_content_title = self.emphasis_content_title;
        m.emphasis_content_desc = self.emphasis_content_desc;
        m.sub_title_text = self.sub_title_text;
        m.vertical_contents = self.vertical_contents;
        m.horizontal_contents = self.horizontal_contents;
        m.jumps = self.jumps;
        m.card_action_type = self.card_action_type;
        m.card_action_appid = self.card_action_appid;
        m.card_action_pagepath = self.card_action_pagepath;
        m.card_action_url = self.card_action_url;
        m.task_id = self.task_id;
        m.button_selection = self.button_selection;
        m.buttons = self.buttons;
        m.checkbox_mode = self.checkbox_mode;
        m.checkbox_question_key = self.checkbox_question_key;
        m.options = self.options;
        m.submit_button_text = self.submit_button_text;
        m.submit_button_key = self.submit_button_key;
        m.selects = self.selects;
        m.quote_area = self.quote_area;
        m
    }
}
