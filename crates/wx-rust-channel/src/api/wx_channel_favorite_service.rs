//! WxChannelFavoriteService（对应 Java `me.chanjar.weixin.channel.api.WxChannelFavoriteService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::favorite::FavoriteCountResponse;

/// 收藏管理服务（对应 Java `WxChannelFavoriteService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_favorite_service_impl` 的
/// `WxChannelFavoriteServiceImpl`（Java `WxChannelFavoriteServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelFavoriteService: Send + Sync {
    /// 获取店铺收藏的人数（对应 Java `WxChannelFavoriteService#getFavoriteCount()`）。
    async fn get_favorite_count(&self) -> Result<FavoriteCountResponse, WxErrorException>;
}
