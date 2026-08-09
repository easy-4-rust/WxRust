//! 公众号用户服务。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpUserService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::result::WxMpUser;

/// 用户服务。
#[async_trait]
pub trait WxMpUserService: Send + Sync {
    /// 获取用户基本信息。
    ///
    /// # 参数
    /// - `openid`：用户 openid
    ///
    /// # 返回
    /// 用户信息。
    async fn user_info(&self, openid: &str) -> Result<WxMpUser, WxErrorException>;
}
