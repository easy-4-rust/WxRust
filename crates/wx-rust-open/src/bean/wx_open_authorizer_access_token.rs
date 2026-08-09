//! 授权方 access_token 数据对象。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.WxOpenAuthorizerAccessToken`：
//! `POST /cgi-bin/component/api_authorizer_token` 的响应体（Lombok `@Data`
//! + `WxOpenGsonBuilder.fromJson`；Rust 以 serde 派生表达同一线格式）。

use serde::{Deserialize, Serialize};

/// 授权方 access_token 响应体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxOpenAuthorizerAccessToken {
    /// 授权方 access_token 值（对应 Java `authorizerAccessToken` 字段）。
    #[serde(rename = "authorizer_access_token")]
    pub authorizer_access_token: String,
    /// 授权方 refresh_token（对应 Java `authorizerRefreshToken` 字段）。
    #[serde(rename = "authorizer_refresh_token")]
    pub authorizer_refresh_token: String,
    /// 有效期（秒），默认 -1（对应 Java `expiresIn = -1`）。
    #[serde(rename = "expires_in")]
    pub expires_in: i32,
}

impl WxOpenAuthorizerAccessToken {
    /// 从 JSON 解析（对应 Java 静态方法 `fromJson(String)`）。
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// 授权方 access_token 值（对应 Java `getAuthorizerAccessToken()`）。
    pub fn authorizer_access_token(&self) -> &str {
        &self.authorizer_access_token
    }

    /// 授权方 refresh_token（对应 Java `getAuthorizerRefreshToken()`）。
    pub fn authorizer_refresh_token(&self) -> &str {
        &self.authorizer_refresh_token
    }

    /// 有效期（秒）（对应 Java `getExpiresIn()`）。
    pub fn expires_in(&self) -> i32 {
        self.expires_in
    }
}
