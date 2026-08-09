//! 公众号的自动回复规则。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.result.WxMpCurrentAutoReplyInfo`。

use serde::{Deserialize, Serialize};

use super::wx_mp_user::deserialize_bool_from_int;

/// 公众号的自动回复规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpCurrentAutoReplyInfo {
    /// 关注后自动回复是否开启（0/1）。
    #[serde(
        rename = "is_add_friend_reply_open",
        default,
        deserialize_with = "deserialize_bool_from_int"
    )]
    pub is_add_friend_reply_open: Option<bool>,
    /// 消息自动回复是否开启（0/1）。
    #[serde(
        rename = "is_autoreply_open",
        default,
        deserialize_with = "deserialize_bool_from_int"
    )]
    pub is_auto_reply_open: Option<bool>,
    /// 关注后自动回复的信息。
    #[serde(
        rename = "add_friend_autoreply_info",
        skip_serializing_if = "Option::is_none"
    )]
    pub add_friend_auto_reply_info: Option<AutoReplyInfo>,
    /// 消息自动回复的信息。
    #[serde(
        rename = "message_default_autoreply_info",
        skip_serializing_if = "Option::is_none"
    )]
    pub message_default_auto_reply_info: Option<AutoReplyInfo>,
    /// 关键词自动回复的信息。
    #[serde(
        rename = "keyword_autoreply_info",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyword_auto_reply_info: Option<KeywordAutoReplyInfo>,
}

impl WxMpCurrentAutoReplyInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("自动回复规则解析失败: {e}"))
    }
}

/// 自动回复信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoReplyInfo {
    /// 回复类型（text/image/voice/video/news）。
    #[serde(default)]
    pub r#type: String,
    /// 回复内容。
    #[serde(default)]
    pub content: String,
}

/// 关键词自动回复信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeywordAutoReplyInfo {
    /// 规则列表。
    #[serde(default)]
    pub list: Vec<AutoReplyRule>,
}

/// 关键词自动回复规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoReplyRule {
    /// 规则名称。
    #[serde(rename = "rule_name", default)]
    pub rule_name: String,
    /// 创建时间（Unix 秒，对应 Java `WxDateTypeAdapter`）。
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    /// 回复模式（reply_all 全部回复 / random_one 随机回复一条）。
    #[serde(rename = "reply_mode", default)]
    pub reply_mode: String,
    /// 关键词列表信息。
    #[serde(rename = "keyword_list_info", default)]
    pub keyword_list_info: Vec<KeywordInfo>,
    /// 回复列表信息。
    #[serde(rename = "reply_list_info", default)]
    pub reply_list_info: Vec<ReplyInfo>,
}

/// 关键词信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeywordInfo {
    /// 关键词类型（text）。
    #[serde(default)]
    pub r#type: String,
    /// 匹配模式（contain 包含 / equal 等于）。
    #[serde(rename = "match_mode", default)]
    pub match_mode: String,
    /// 关键词内容。
    #[serde(default)]
    pub content: String,
}

/// 回复信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplyInfo {
    /// 回复类型。
    #[serde(default)]
    pub r#type: String,
    /// 回复内容。
    #[serde(default)]
    pub content: String,
    /// 图文消息信息。
    #[serde(rename = "news_info", skip_serializing_if = "Option::is_none")]
    pub news_info: Option<NewsInfo>,
}

/// 图文消息信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NewsInfo {
    /// 图文消息列表。
    #[serde(default)]
    pub list: Vec<NewsItem>,
}

/// 图文消息项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NewsItem {
    /// 封面图片链接。
    #[serde(rename = "cover_url", default)]
    pub cover_url: String,
    /// 作者。
    #[serde(default)]
    pub author: String,
    /// 图文消息链接。
    #[serde(rename = "content_url", default)]
    pub content_url: String,
    /// 摘要。
    #[serde(default)]
    pub digest: String,
    /// 是否显示封面（0/1）。
    #[serde(
        rename = "show_cover",
        default,
        deserialize_with = "deserialize_bool_from_int"
    )]
    pub show_cover: Option<bool>,
    /// 原文链接。
    #[serde(rename = "source_url", default)]
    pub source_url: String,
    /// 标题。
    #[serde(default)]
    pub title: String,
}
