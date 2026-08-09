//! component_access_token 数据对象。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.WxOpenComponentAccessToken`：
//! `POST /cgi-bin/component/api_component_token` 的响应体（Gson 字段映射
//! 直接对应 JSON 键名，Rust 以 serde 派生表达同一线格式）。

use serde::{Deserialize, Serialize};

/// component_access_token 响应体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxOpenComponentAccessToken {
    /// component_access_token 值（对应 Java `componentAccessToken` 字段）。
    #[serde(rename = "component_access_token")]
    pub component_access_token: String,
    /// 有效期（秒），默认 -1（对应 Java `expiresIn = -1`）。
    #[serde(rename = "expires_in")]
    pub expires_in: i32,
}

impl WxOpenComponentAccessToken {
    /// 从 JSON 解析（对应 Java 静态方法 `fromJson(String)`，
    /// `WxOpenGsonBuilder` → serde_json）。
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// component_access_token 值（对应 Java `getComponentAccessToken()`）。
    pub fn component_access_token(&self) -> &str {
        &self.component_access_token
    }

    /// 有效期（秒）（对应 Java `getExpiresIn()`）。
    pub fn expires_in(&self) -> i32 {
        self.expires_in
    }
}
