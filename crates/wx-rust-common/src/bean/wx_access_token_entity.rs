//! token 数据对象（带 appid）。
//!
//! 对应 Java `me.chanjar.weixin.common.bean.WxAccessTokenEntity`。

use serde::{Deserialize, Serialize};

use super::WxAccessToken;

/// 带 appid 的 access token。
///
/// 在 [`WxAccessToken`] 基础上增加所属 appid，用于多租户场景区分。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WxAccessTokenEntity {
    /// access token 值
    #[serde(rename = "access_token", default)]
    pub access_token: String,

    /// 有效期（秒）
    #[serde(rename = "expires_in", default)]
    pub expires_in: i32,

    /// 所属 appid
    pub appid: String,
}

impl WxAccessTokenEntity {
    /// 从普通 access token 与 appid 构建。
    pub fn from_access_token(token: WxAccessToken, appid: impl Into<String>) -> Self {
        Self {
            access_token: token.access_token,
            expires_in: token.expires_in,
            appid: appid.into(),
        }
    }
}
