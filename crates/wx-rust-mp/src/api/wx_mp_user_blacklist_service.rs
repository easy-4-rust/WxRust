//! WxMpUserBlacklist服务
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpUserBlacklistService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::result::WxMpUserBlacklistGetResult;

/// WxMpUserBlacklist服务。
#[async_trait]
pub trait WxMpUserBlacklistService: Send + Sync {
    async fn get_blacklist(
        &self,
        next_openid: &str,
    ) -> Result<WxMpUserBlacklistGetResult, WxErrorException>;

    async fn push_to_blacklist(&self, openid_list: &[String]) -> Result<(), WxErrorException>;

    async fn pull_from_blacklist(&self, openid_list: &[String]) -> Result<(), WxErrorException>;
}
