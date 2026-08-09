//! 订阅通知消息。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.subscribe.WxMpSubscribeMessage`。线格式由
//! `WxMpSubscribeMessageGsonAdapter` 决定：`touser`/`template_id` + `data` 结构。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// 订阅通知消息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpSubscribeMessage {
    /// 接收者 openid。
    #[serde(rename = "touser", skip_serializing_if = "Option::is_none")]
    pub to_user: Option<String>,
    /// 模板 ID。
    #[serde(rename = "template_id", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 模板跳转链接（与 miniprogram 都传时优先跳转小程序）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 跳小程序所需数据。
    #[serde(rename = "miniprogram", skip_serializing_if = "Option::is_none")]
    pub mini_program: Option<MiniProgram>,
    /// 订阅场景值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    /// 消息标题（15 字以内）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 消息内容文本（200 字以内）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_value: Option<String>,
    /// 消息内容文本颜色。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_color: Option<String>,
    /// 跳转网页时填写。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// 订阅通知消息专用数据 map（非空时按 `{key: {"value": ...}}` 输出，对应 Java `dataMap`）。
    #[serde(default)]
    pub data_map: HashMap<String, String>,
}

impl WxMpSubscribeMessage {
    /// 构建空消息。
    pub fn builder() -> Self {
        Self::default()
    }

    /// 设置接收者 openid。
    pub fn to_user(mut self, to_user: impl Into<String>) -> Self {
        self.to_user = Some(to_user.into());
        self
    }

    /// 设置模板 ID。
    pub fn template_id(mut self, template_id: impl Into<String>) -> Self {
        self.template_id = Some(template_id.into());
        self
    }

    /// 设置跳转链接。
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// 设置小程序信息。
    pub fn mini_program(mut self, mini_program: MiniProgram) -> Self {
        self.mini_program = Some(mini_program);
        self
    }

    /// 设置订阅场景值。
    pub fn scene(mut self, scene: impl Into<String>) -> Self {
        self.scene = Some(scene.into());
        self
    }

    /// 设置消息标题。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置消息内容文本。
    pub fn content_value(mut self, content_value: impl Into<String>) -> Self {
        self.content_value = Some(content_value.into());
        self
    }

    /// 设置消息内容文本颜色。
    pub fn content_color(mut self, content_color: impl Into<String>) -> Self {
        self.content_color = Some(content_color.into());
        self
    }

    /// 序列化为 JSON（对应 Java `WxMpSubscribeMessageGsonAdapter`）。
    pub fn to_json(&self) -> Result<String, String> {
        let mut map = serde_json::Map::new();
        if let Some(v) = &self.to_user {
            map.insert("touser".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.template_id {
            map.insert("template_id".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.url {
            map.insert("url".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.page {
            map.insert("page".into(), serde_json::json!(v));
        }
        if let Some(mp) = &self.mini_program {
            let mut mp_json = serde_json::Map::new();
            mp_json.insert("appid".into(), serde_json::json!(mp.appid));
            if mp.use_path {
                mp_json.insert("path".into(), serde_json::json!(mp.page_path));
            } else {
                mp_json.insert("pagepath".into(), serde_json::json!(mp.page_path));
            }
            map.insert("miniprogram".into(), serde_json::Value::Object(mp_json));
        }
        if let Some(v) = &self.scene {
            map.insert("scene".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.title {
            map.insert("title".into(), serde_json::json!(v));
        }
        let mut data = serde_json::Map::new();
        if self.data_map.is_empty() {
            let mut content = serde_json::Map::new();
            if let Some(v) = &self.content_value {
                content.insert("value".into(), serde_json::json!(v));
            }
            if let Some(v) = &self.content_color {
                content.insert("color".into(), serde_json::json!(v));
            }
            data.insert("content".into(), serde_json::Value::Object(content));
        } else {
            for (key, value) in &self.data_map {
                let mut content = serde_json::Map::new();
                content.insert("value".into(), serde_json::json!(value));
                data.insert(key.clone(), serde_json::Value::Object(content));
            }
        }
        map.insert("data".into(), serde_json::Value::Object(data));
        serde_json::to_string(&serde_json::Value::Object(map))
            .map_err(|e| format!("订阅通知消息序列化失败: {e}"))
    }
}

/// 小程序信息（对应 Java `WxMpSubscribeMessage.MiniProgram`）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MiniProgram {
    /// 小程序 appid。
    pub appid: String,
    /// 小程序页面路径。
    pub page_path: String,
    /// 是否使用 path，否则使用 pagepath（对应 Java `usePath`）。
    #[serde(default)]
    pub use_path: bool,
}

impl MiniProgram {
    /// 构建小程序信息。
    pub fn new(appid: impl Into<String>, page_path: impl Into<String>, use_path: bool) -> Self {
        Self {
            appid: appid.into(),
            page_path: page_path.into(),
            use_path,
        }
    }
}
