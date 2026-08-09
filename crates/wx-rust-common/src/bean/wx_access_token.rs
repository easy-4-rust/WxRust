//! access token 数据对象。
//!
//! 对应 Java `me.chanjar.weixin.common.bean.WxAccessToken`。

use serde::{Deserialize, Serialize};

/// access token 对象。
///
/// 对应微信 `cgi-bin/token` 接口的响应：`{ "access_token": "...", "expires_in": 7200 }`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WxAccessToken {
    /// access token 值
    #[serde(rename = "access_token", default)]
    pub access_token: String,

    /// 有效期（秒），默认 -1 表示未设置
    #[serde(rename = "expires_in", default = "default_expires_in")]
    pub expires_in: i32,
}

fn default_expires_in() -> i32 {
    -1
}

impl WxAccessToken {
    /// 从微信接口返回的 JSON 报文解析 access token。
    ///
    /// # 参数
    /// - `json`：微信 `cgi-bin/token` 接口返回的 JSON 字符串
    ///
    /// # 返回
    /// 解析出的 `WxAccessToken`；解析失败时返回错误。
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// 构建 access token。
    ///
    /// # 参数
    /// - `access_token`：access token 值
    /// - `expires_in`：有效期（秒）
    pub fn new(access_token: impl Into<String>, expires_in: i32) -> Self {
        Self {
            access_token: access_token.into(),
            expires_in,
        }
    }
}
