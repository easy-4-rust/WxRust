//! 模板消息。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.template.WxMpTemplateMessage` +
//! `WxMpTemplateData` + 内部类 `MiniProgram`。`add_data` 含微信模板消息
//! 字符串长度处理（`resetValue` 截断规则）。

use serde::{Deserialize, Serialize};

/// 模板消息数据项。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct WxMpTemplateData {
    /// 字段名（如 first/remark/thing01）。
    pub name: String,
    /// 字段值（`add_data` 时按类型截断）。
    pub value: String,
    /// 字段颜色（默认无）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl WxMpTemplateData {
    /// 构建数据项（无颜色）。
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            color: None,
        }
    }

    /// 构建数据项（带颜色）。
    pub fn with_color(
        name: impl Into<String>,
        value: impl Into<String>,
        color: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            color: Some(color.into()),
        }
    }

    /// 字段名。
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// 字段值。
    pub fn get_value(&self) -> &str {
        &self.value
    }
}

/// 小程序信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct MiniProgram {
    /// 小程序 appid。
    pub appid: String,
    /// 小程序页面路径。
    pub path: String,
}

impl MiniProgram {
    /// 构建小程序信息。
    pub fn new(appid: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            appid: appid.into(),
            path: path.into(),
        }
    }
}

/// 模板消息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpTemplateMessage {
    /// 接收者 openid。
    #[serde(rename = "touser", skip_serializing_if = "Option::is_none")]
    pub to_user: Option<String>,
    /// 模板 id。
    #[serde(rename = "template_id", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 点击模板卡片跳转 url。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 跳小程序所需数据。
    #[serde(rename = "miniprogram", skip_serializing_if = "Option::is_none")]
    pub mini_program: Option<MiniProgram>,
    /// 防重入 id（对应 Java `client_msg_id`）。
    #[serde(rename = "client_msg_id", skip_serializing_if = "Option::is_none")]
    pub client_msg_id: Option<String>,
    /// 模板数据（`data` 为 map 结构：字段名 → 值对象）。
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<WxMpTemplateData>,
}

impl WxMpTemplateMessage {
    /// 构建空模板消息。
    pub fn builder() -> Self {
        Self::default()
    }

    /// 设置接收者 openid。
    pub fn to_user(mut self, to_user: impl Into<String>) -> Self {
        self.to_user = Some(to_user.into());
        self
    }

    /// 设置模板 id。
    pub fn template_id(mut self, template_id: impl Into<String>) -> Self {
        self.template_id = Some(template_id.into());
        self
    }

    /// 设置跳转 url。
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// 设置跳小程序数据。
    pub fn mini_program(mut self, mini_program: MiniProgram) -> Self {
        self.mini_program = Some(mini_program);
        self
    }

    /// 设置防重入 id。
    pub fn client_msg_id(mut self, client_msg_id: impl Into<String>) -> Self {
        self.client_msg_id = Some(client_msg_id.into());
        self
    }

    /// 添加模板数据（含微信模板消息字符串长度处理）。
    ///
    /// 截断规则（对应 Java `resetValue`）：
    /// - `thing*`：超过 20 字 → 前 17 字 + `...`
    /// - `character_string*`：超过 32 字 → 前 29 字 + `...`
    /// - `phone_number*`：超过 17 字 → 前 14 字 + `...`
    /// - `car_number*`：超过 8 字 → 前 5 字 + `...`
    /// - `const*`：超过 20 字 → 前 17 字 + `...`
    pub fn add_data(mut self, mut datum: WxMpTemplateData) -> Self {
        let name = datum.name.clone();
        let value = datum.value.clone();
        // Java StringUtils.substring(value, 0, n)：前 n 个 UTF-16 单元；
        // BMP 字符（中文/英文/数字）与 Rust char 一一对应，取前 n 字符
        let truncated = |value: &str, n: usize| -> String {
            let s: String = value.chars().take(n).collect();
            format!("{s}...")
        };
        let value = if name.starts_with("thing") && value.len() > 20 {
            truncated(&value, 17)
        } else if name.starts_with("character_string") && value.len() > 32 {
            truncated(&value, 29)
        } else if name.starts_with("phone_number") && value.len() > 17 {
            truncated(&value, 14)
        } else if name.starts_with("car_number") && value.len() > 8 {
            truncated(&value, 5)
        } else if name.starts_with("const") && value.len() > 20 {
            truncated(&value, 17)
        } else {
            value
        };
        datum.value = value;
        self.data.push(datum);
        self
    }

    /// 模板数据列表。
    pub fn get_data(&self) -> &[WxMpTemplateData] {
        &self.data
    }

    /// 序列化为 JSON（对应 Java `toJson`：`data` 以字段名 → 值对象输出）。
    pub fn to_json(&self) -> Result<String, String> {
        let mut map = serde_json::Map::new();
        if let Some(v) = &self.to_user {
            map.insert("touser".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.template_id {
            map.insert("template_id".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.client_msg_id {
            map.insert("client_msg_id".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.url {
            map.insert("url".into(), serde_json::json!(v));
        }
        if let Some(v) = &self.mini_program {
            map.insert("miniprogram".into(), serde_json::json!(v));
        }
        let mut data_map = serde_json::Map::new();
        for d in &self.data {
            let mut item = serde_json::Map::new();
            item.insert("value".into(), serde_json::json!(d.value));
            if let Some(c) = &d.color {
                item.insert("color".into(), serde_json::json!(c));
            }
            data_map.insert(d.name.clone(), serde_json::Value::Object(item));
        }
        map.insert("data".into(), serde_json::Value::Object(data_map));
        serde_json::to_string(&serde_json::Value::Object(map))
            .map_err(|e| format!("模板消息序列化失败: {e}"))
    }
}
