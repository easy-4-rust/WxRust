//! 模板消息（公众号模板消息 / 小程序模板消息二选一）。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaUniformMessage`。
//! 线格式由 `WxMaUniformMessageGsonAdapter` 决定：
//! - `isMpTemplateMsg == true` 时输出 `{"touser": ..., "mp_template_msg": {...}}`
//!   （appid/template_id/url/miniprogram/data）；
//! - 否则输出 `{"touser": ..., "weapp_template_msg": {...}}`
//!   （template_id/page/form_id/data/emphasis_keyword）。
//! serde 派生仅为平铺字段的解析便利；对外序列化以 `to_json`（adapter 线格式）为准。

use serde::{Deserialize, Serialize};

use crate::bean::WxMaTemplateData;

/// 公众号模板消息所要跳转的小程序（对应 Java `WxMaUniformMessage.MiniProgram`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MiniProgram {
    /// 小程序 appid，小程序的必须与公众号具有绑定关系。
    pub appid: Option<String>,
    /// 小程序页面路径（最终线格式键名由 `use_path`/`use_page_path` 决定）。
    pub page_path: Option<String>,
    /// 是否使用 `path` 作为线格式键名（微信官方接口变化多端）。
    #[serde(default)]
    pub use_path: bool,
    /// 是否使用 `pagePath` 作为线格式键名。
    #[serde(default)]
    pub use_page_path: bool,
}

/// 模板消息（对应 Java `WxMaUniformMessage`）。
///
/// 序列化走手写 `Serialize`（`WxMaUniformMessageGsonAdapter` 线格式）；
/// `Deserialize` 派生为平铺解析便利（Java 无 fromJson，线格式不会回传）。
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct WxMaUniformMessage {
    /// 是否发送公众号模版消息，否则发送小程序模版消息。
    #[serde(rename = "isMpTemplateMsg", default)]
    pub is_mp_template_msg: bool,
    /// 用户 openid（小程序的 openid，或 mp_template_msg.appid 对应公众号的 openid）。
    pub to_user: Option<String>,
    /// 公众号 appid，要求与小程序有绑定且同主体。
    pub appid: Option<String>,
    /// 公众号或小程序模板 ID。
    pub template_id: Option<String>,
    /// 公众号模板消息所要跳转的 url。
    pub url: Option<String>,
    /// 小程序页面路径（小程序模板消息）。
    pub page: Option<String>,
    /// 小程序模板消息 formid。
    pub form_id: Option<String>,
    /// 公众号模板消息所要跳转的小程序。
    pub mini_program: Option<MiniProgram>,
    /// 模板数据。
    #[serde(default)]
    pub data: Vec<WxMaTemplateData>,
    /// 模板需要放大的关键词，不填则默认无放大。
    pub emphasis_keyword: Option<String>,
}

impl WxMaUniformMessage {
    /// 追加模板数据（对应 Java `addData`）。
    pub fn add_data(&mut self, datum: WxMaTemplateData) -> &mut Self {
        self.data.push(datum);
        self
    }

    /// 序列化为 JSON（对应 Java `WxMaUniformMessageGsonAdapter` 线格式）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("模板消息序列化失败: {e}"))
    }
}

impl Serialize for WxMaUniformMessage {
    /// 对应 Java `WxMaUniformMessageGsonAdapter.serialize`：按
    /// `is_mp_template_msg` 分支输出 `mp_template_msg`/`weapp_template_msg`。
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut message_json = serde_json::Map::new();
        if let Some(v) = &self.to_user {
            message_json.insert("touser".into(), serde_json::json!(v));
        }
        if self.is_mp_template_msg {
            let mut msg = serde_json::Map::new();
            if let Some(v) = &self.appid {
                msg.insert("appid".into(), serde_json::json!(v));
            }
            msg.insert("template_id".into(), serde_json::json!(self.template_id));
            if let Some(v) = &self.url {
                msg.insert("url".into(), serde_json::json!(v));
            }
            if let Some(mini_program) = &self.mini_program {
                let mut mini_program_json = serde_json::Map::new();
                mini_program_json.insert("appid".into(), serde_json::json!(mini_program.appid));
                let key = if mini_program.use_path {
                    "path"
                } else if mini_program.use_page_path {
                    "pagePath"
                } else {
                    "pagepath"
                };
                mini_program_json.insert(key.into(), serde_json::json!(mini_program.page_path));
                msg.insert(
                    "miniprogram".into(),
                    serde_json::Value::Object(mini_program_json),
                );
            }
            if !self.data.is_empty() {
                let mut data = serde_json::Map::new();
                for td in &self.data {
                    let mut data_json = serde_json::Map::new();
                    data_json.insert("value".into(), serde_json::json!(&td.value));
                    // Java adapter：color 非 null 才输出；生成 bean 中 color 为 String（缺失为空串）
                    if !td.color.is_empty() {
                        data_json.insert("color".into(), serde_json::json!(&td.color));
                    }
                    data.insert(td.name.clone(), serde_json::Value::Object(data_json));
                }
                msg.insert("data".into(), serde_json::Value::Object(data));
            }
            message_json.insert("mp_template_msg".into(), serde_json::Value::Object(msg));
        } else {
            // 小程序模版消息
            let mut msg = serde_json::Map::new();
            msg.insert("template_id".into(), serde_json::json!(self.template_id));
            if let Some(v) = &self.page {
                msg.insert("page".into(), serde_json::json!(v));
            }
            if let Some(v) = &self.form_id {
                msg.insert("form_id".into(), serde_json::json!(v));
            }
            let mut data = serde_json::Map::new();
            for td in &self.data {
                let mut data_json = serde_json::Map::new();
                data_json.insert("value".into(), serde_json::json!(&td.value));
                data.insert(td.name.clone(), serde_json::Value::Object(data_json));
            }
            msg.insert("data".into(), serde_json::Value::Object(data));
            if let Some(v) = &self.emphasis_keyword {
                msg.insert("emphasis_keyword".into(), serde_json::json!(v));
            }
            message_json.insert("weapp_template_msg".into(), serde_json::Value::Object(msg));
        }
        serde_json::Value::Object(message_json).serialize(serializer)
    }
}
