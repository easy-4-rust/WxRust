//! 客服消息（JSON 线格式）。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaKefuMessage`。Gson 按
//! `@SerializedName` 反射输出：`touser`/`msgtype` + 各消息类型分支对象
//! （text/image/link/miniprogrampage/aimsgcontext，null 分支省略）。
//! Rust 以 Option 字段 + `skip_serializing_if` 复现同一线格式。

use serde::{Deserialize, Serialize};

use crate::builder::{
    ImageMessageBuilder, LinkMessageBuilder, MaPageMessageBuilder, TextMessageBuilder,
};

/// 文本消息内容（对应 Java `WxMaKefuMessage.KfText`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KfText {
    /// 文本内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// 图片消息内容（对应 Java `WxMaKefuMessage.KfImage`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KfImage {
    /// 素材 media_id。
    #[serde(rename = "media_id", skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
}

/// 图文链接消息内容（对应 Java `WxMaKefuMessage.KfLink`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KfLink {
    /// 标题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 跳转链接。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 缩略图链接。
    #[serde(rename = "thumb_url", skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
}

/// 小程序卡片消息内容（对应 Java `WxMaKefuMessage.KfMaPage`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KfMaPage {
    /// 标题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 小程序页面路径。
    #[serde(rename = "pagepath", skip_serializing_if = "Option::is_none")]
    pub page_path: Option<String>,
    /// 缩略图 media_id。
    #[serde(rename = "thumb_media_id", skip_serializing_if = "Option::is_none")]
    pub thumb_media_id: Option<String>,
}

/// AI 会话上下文（对应 Java `WxMaKefuMessage.AiMsgContext`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AiMsgContext {
    /// AI 会话消息 id。
    #[serde(rename = "msgid", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
}

/// 客服消息（对应 Java `WxMaKefuMessage`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxMaKefuMessage {
    /// 接收者 openid。
    #[serde(rename = "touser", skip_serializing_if = "Option::is_none")]
    pub to_user: Option<String>,
    /// 消息类型（text/image/link/miniprogrampage）。
    #[serde(rename = "msgtype", skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    /// 文本消息分支。
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<KfText>,
    /// 图片消息分支。
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<KfImage>,
    /// 图文链接消息分支。
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<KfLink>,
    /// 小程序卡片消息分支。
    #[serde(rename = "miniprogrampage", skip_serializing_if = "Option::is_none")]
    pub ma_page: Option<KfMaPage>,
    /// AI 会话上下文（带 `msgid` 时输出）。
    #[serde(rename = "aimsgcontext", skip_serializing_if = "Option::is_none")]
    pub ai_msg_context: Option<AiMsgContext>,
}

impl WxMaKefuMessage {
    /// 获得文本消息 builder（对应 Java `newTextBuilder()`）。
    pub fn new_text_builder() -> TextMessageBuilder {
        TextMessageBuilder::new()
    }

    /// 获得图片消息 builder（对应 Java `newImageBuilder()`）。
    pub fn new_image_builder() -> ImageMessageBuilder {
        ImageMessageBuilder::new()
    }

    /// 获得图文链接消息 builder（对应 Java `newLinkBuilder()`）。
    pub fn new_link_builder() -> LinkMessageBuilder {
        LinkMessageBuilder::new()
    }

    /// 获得小程序卡片消息 builder（对应 Java `newMaPageBuilder()`）。
    pub fn new_ma_page_builder() -> MaPageMessageBuilder {
        MaPageMessageBuilder::new()
    }

    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("客服消息序列化失败: {e}"))
    }
}
