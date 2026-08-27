//! 客服消息。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.kefu.WxMpKefuMessage`。线格式由
//! `WxMpKefuMessageGsonAdapter` 决定：`touser`/`msgtype` + 按消息类型分支输出。

use serde::{Deserialize, Serialize};

/// 客服消息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpKefuMessage {
    /// 接收者 openid。
    #[serde(rename = "touser", skip_serializing_if = "Option::is_none")]
    pub to_user: Option<String>,
    /// 消息类型（`WxConsts::KefuMsgType`）。
    #[serde(rename = "msgtype", skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    /// 文本消息内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 素材 media_id（image/voice/video/mpnews）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    /// 视频缩略图 media_id。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_media_id: Option<String>,
    /// 标题（video/music/miniprogrampage）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 描述（video/music）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 音乐链接。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_url: Option<String>,
    /// 高品质音乐链接。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hq_music_url: Option<String>,
    /// 客服账号（会话 ID）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf_account: Option<String>,
    /// 卡券 id（wxcard）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 图文消息 media_id（mpnews）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_news_media_id: Option<String>,
    /// 小程序 appid。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_program_app_id: Option<String>,
    /// 小程序页面路径。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_program_page_path: Option<String>,
    /// 菜单消息头部内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_content: Option<String>,
    /// 菜单消息尾部内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_content: Option<String>,
    /// 图文消息文章列表（news）。
    #[serde(default)]
    pub articles: Vec<WxArticle>,
    /// 发布接口 article_id（mpnewsarticle）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_news_article_id: Option<String>,
    /// 菜单消息里的菜单内容（msgmenu）。
    #[serde(default)]
    pub msg_menus: Vec<MsgMenu>,
}

impl WxMpKefuMessage {
    /// 文本消息 builder。
    pub fn text() -> KefuMessageBuilder {
        KefuMessageBuilder::new("text")
    }

    /// 图片消息 builder。
    pub fn image() -> KefuMessageBuilder {
        KefuMessageBuilder::new("image")
    }

    /// 语音消息 builder。
    pub fn voice() -> KefuMessageBuilder {
        KefuMessageBuilder::new("voice")
    }

    /// 视频消息 builder。
    pub fn video() -> KefuMessageBuilder {
        KefuMessageBuilder::new("video")
    }

    /// 音乐消息 builder。
    pub fn music() -> KefuMessageBuilder {
        KefuMessageBuilder::new("music")
    }

    /// 图文消息（点击跳转到外链）builder。
    pub fn news() -> KefuMessageBuilder {
        KefuMessageBuilder::new("news")
    }

    /// 图文消息（点击跳转到图文消息页面）builder。
    pub fn mpnews() -> KefuMessageBuilder {
        KefuMessageBuilder::new("mpnews")
    }

    /// 卡券消息 builder。
    pub fn wxcard() -> KefuMessageBuilder {
        KefuMessageBuilder::new("wxcard")
    }

    /// 菜单消息 builder。
    pub fn msgmenu() -> KefuMessageBuilder {
        KefuMessageBuilder::new("msgmenu")
    }

    /// 小程序卡片 builder。
    pub fn miniprogrampage() -> KefuMessageBuilder {
        KefuMessageBuilder::new("miniprogrampage")
    }

    /// 图文消息（使用发布接口 article_id）builder。
    pub fn mpnewsarticle() -> KefuMessageBuilder {
        KefuMessageBuilder::new("mpnewsarticle")
    }

    /// 消息类型。
    pub fn get_msg_type(&self) -> &str {
        self.msg_type.as_deref().unwrap_or_default()
    }

    /// 从 JSON 构建（Java Gson 平铺映射语义：嵌套如 `text.content` 不落入顶层字段）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("客服消息解析失败: {e}"))
    }

    /// 序列化为 JSON（对应 Java `WxMpKefuMessageGsonAdapter`）。
    pub fn to_json(&self) -> Result<String, String> {
        let mut map = serde_json::Map::new();
        if let Some(v) = &self.to_user {
            map.insert("touser".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.msg_type {
            map.insert("msgtype".into(), serde_json::json!(v));
        }
        match self.get_msg_type() {
            "text" => {
                let mut text = serde_json::Map::new();
                if let Some(v) = &self.content {
                    text.insert("content".into(), serde_json::json!(v));
                }
                map.insert("text".into(), serde_json::Value::Object(text));
            }
            "image" => {
                let mut image = serde_json::Map::new();
                if let Some(v) = &self.media_id {
                    image.insert("media_id".into(), serde_json::json!(v));
                }
                map.insert("image".into(), serde_json::Value::Object(image));
            }
            "voice" => {
                let mut voice = serde_json::Map::new();
                if let Some(v) = &self.media_id {
                    voice.insert("media_id".into(), serde_json::json!(v));
                }
                map.insert("voice".into(), serde_json::Value::Object(voice));
            }
            "video" => {
                let mut video = serde_json::Map::new();
                if let Some(v) = &self.media_id {
                    video.insert("media_id".into(), serde_json::json!(v));
                }
                if let Some(v) = &self.thumb_media_id {
                    video.insert("thumb_media_id".into(), serde_json::json!(v));
                }
                if let Some(v) = &self.title {
                    video.insert("title".into(), serde_json::json!(v));
                }
                if let Some(v) = &self.description {
                    video.insert("description".into(), serde_json::json!(v));
                }
                map.insert("video".into(), serde_json::Value::Object(video));
            }
            "music" => {
                let mut music = serde_json::Map::new();
                if let Some(v) = &self.title {
                    music.insert("title".into(), serde_json::json!(v));
                }
                if let Some(v) = &self.description {
                    music.insert("description".into(), serde_json::json!(v));
                }
                if let Some(v) = &self.thumb_media_id {
                    music.insert("thumb_media_id".into(), serde_json::json!(v));
                }
                if let Some(v) = &self.music_url {
                    music.insert("musicurl".into(), serde_json::json!(v));
                }
                if let Some(v) = &self.hq_music_url {
                    music.insert("hqmusicurl".into(), serde_json::json!(v));
                }
                map.insert("music".into(), serde_json::Value::Object(music));
            }
            "news" => {
                let articles = serde_json::Value::Array(
                    self.articles
                        .iter()
                        .map(|a| {
                            serde_json::json!({
                                "title": a.title,
                                "description": a.description,
                                "url": a.url,
                                "picurl": a.pic_url,
                            })
                        })
                        .collect(),
                );
                let mut news = serde_json::Map::new();
                news.insert("articles".into(), articles);
                map.insert("news".into(), serde_json::Value::Object(news));
            }
            "mpnews" => {
                let mut mpnews = serde_json::Map::new();
                if let Some(v) = &self.mp_news_media_id {
                    mpnews.insert("media_id".into(), serde_json::json!(v));
                }
                map.insert("mpnews".into(), serde_json::Value::Object(mpnews));
            }
            "wxcard" => {
                let mut wxcard = serde_json::Map::new();
                if let Some(v) = &self.card_id {
                    wxcard.insert("card_id".into(), serde_json::json!(v));
                }
                map.insert("wxcard".into(), serde_json::Value::Object(wxcard));
            }
            "miniprogrampage" => {
                let mut miniprogram_page = serde_json::Map::new();
                if let Some(v) = &self.title {
                    miniprogram_page.insert("title".into(), serde_json::json!(v));
                }
                if let Some(v) = &self.mini_program_app_id {
                    miniprogram_page.insert("appid".into(), serde_json::json!(v));
                }
                if let Some(v) = &self.mini_program_page_path {
                    miniprogram_page.insert("pagepath".into(), serde_json::json!(v));
                }
                if let Some(v) = &self.thumb_media_id {
                    miniprogram_page.insert("thumb_media_id".into(), serde_json::json!(v));
                }
                map.insert(
                    "miniprogrampage".into(),
                    serde_json::Value::Object(miniprogram_page),
                );
            }
            "msgmenu" => {
                let list = serde_json::Value::Array(
                    self.msg_menus
                        .iter()
                        .map(|m| serde_json::json!({ "id": m.id, "content": m.content }))
                        .collect(),
                );
                let mut msgmenu = serde_json::Map::new();
                if let Some(v) = &self.head_content {
                    msgmenu.insert("head_content".into(), serde_json::json!(v));
                }
                msgmenu.insert("list".into(), list);
                if let Some(v) = &self.tail_content {
                    msgmenu.insert("tail_content".into(), serde_json::json!(v));
                }
                map.insert("msgmenu".into(), serde_json::Value::Object(msgmenu));
            }
            "mpnewsarticle" => {
                let mut mp_news_article = serde_json::Map::new();
                if let Some(v) = &self.mp_news_article_id {
                    mp_news_article.insert("article_id".into(), serde_json::json!(v));
                }
                map.insert(
                    "mpnewsarticle".into(),
                    serde_json::Value::Object(mp_news_article),
                );
            }
            other => return Err(format!("非法消息类型，暂不支持: {other}")),
        }

        if let Some(v) = &self.kf_account
            && !v.is_empty()
        {
            let mut customservice = serde_json::Map::new();
            customservice.insert("kf_account".into(), serde_json::json!(v));
            map.insert(
                "customservice".into(),
                serde_json::Value::Object(customservice),
            );
        }

        serde_json::to_string(&serde_json::Value::Object(map))
            .map_err(|e| format!("客服消息序列化失败: {e}"))
    }
}

/// 客服消息 builder（对应 Java `me.chanjar.weixin.mp.builder.kefu` 各 Builder）。
#[derive(Debug, Default)]
pub struct KefuMessageBuilder {
    msg: WxMpKefuMessage,
}

impl KefuMessageBuilder {
    /// 构建指定消息类型的 builder。
    pub fn new(msg_type: &str) -> Self {
        Self {
            msg: WxMpKefuMessage {
                msg_type: Some(msg_type.to_string()),
                ..Default::default()
            },
        }
    }

    /// 设置接收者 openid。
    pub fn to_user(mut self, to_user: impl Into<String>) -> Self {
        self.msg.to_user = Some(to_user.into());
        self
    }

    /// 设置文本内容。
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.msg.content = Some(content.into());
        self
    }

    /// 设置素材 media_id。
    pub fn media_id(mut self, media_id: impl Into<String>) -> Self {
        self.msg.media_id = Some(media_id.into());
        self
    }

    /// 设置视频缩略图 media_id。
    pub fn thumb_media_id(mut self, thumb_media_id: impl Into<String>) -> Self {
        self.msg.thumb_media_id = Some(thumb_media_id.into());
        self
    }

    /// 设置标题。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.msg.title = Some(title.into());
        self
    }

    /// 设置描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.msg.description = Some(description.into());
        self
    }

    /// 设置音乐链接。
    pub fn music_url(mut self, music_url: impl Into<String>) -> Self {
        self.msg.music_url = Some(music_url.into());
        self
    }

    /// 设置高品质音乐链接。
    pub fn hq_music_url(mut self, hq_music_url: impl Into<String>) -> Self {
        self.msg.hq_music_url = Some(hq_music_url.into());
        self
    }

    /// 设置图文文章。
    pub fn add_article(mut self, article: WxArticle) -> Self {
        self.msg.articles.push(article);
        self
    }

    /// 设置小程序 appid。
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.msg.mini_program_app_id = Some(app_id.into());
        self
    }

    /// 设置小程序页面路径。
    pub fn page_path(mut self, page_path: impl Into<String>) -> Self {
        self.msg.mini_program_page_path = Some(page_path.into());
        self
    }

    /// 设置菜单消息菜单项。
    pub fn add_menus(mut self, menus: Vec<MsgMenu>) -> Self {
        self.msg.msg_menus.extend(menus);
        self
    }

    /// 设置菜单消息头部内容。
    pub fn head_content(mut self, head_content: impl Into<String>) -> Self {
        self.msg.head_content = Some(head_content.into());
        self
    }

    /// 设置菜单消息尾部内容。
    pub fn tail_content(mut self, tail_content: impl Into<String>) -> Self {
        self.msg.tail_content = Some(tail_content.into());
        self
    }

    /// 设置发布接口 article_id。
    pub fn article_id(mut self, article_id: impl Into<String>) -> Self {
        self.msg.mp_news_article_id = Some(article_id.into());
        self
    }

    /// 设置卡券 id。
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.msg.card_id = Some(card_id.into());
        self
    }

    /// 构建消息。
    pub fn build(self) -> WxMpKefuMessage {
        self.msg
    }
}

/// 图文消息文章（对应 Java `WxMpKefuMessage.WxArticle`）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct WxArticle {
    /// 标题。
    pub title: String,
    /// 描述。
    pub description: String,
    /// 跳转链接。
    pub url: String,
    /// 图片链接。
    pub pic_url: String,
}

impl WxArticle {
    /// 构建文章。
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        url: impl Into<String>,
        pic_url: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            url: url.into(),
            pic_url: pic_url.into(),
        }
    }
}

/// 菜单消息菜单项（对应 Java `WxMpKefuMessage.MsgMenu`）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct MsgMenu {
    /// 菜单 id。
    pub id: String,
    /// 菜单内容。
    pub content: String,
}

impl MsgMenu {
    /// 构建菜单项。
    pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
        }
    }
}
