//! WxChannelFavoriteServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelFavoriteServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_favorite_service::WxChannelFavoriteService;
use crate::bean::favorite::FavoriteCountResponse;
use crate::enums::url_favorite as url;

/// 收藏管理服务实现。
pub struct WxChannelFavoriteServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelFavoriteServiceImpl {
    /// 构建收藏管理服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxChannelFavoriteService for WxChannelFavoriteServiceImpl {
    async fn get_favorite_count(&self) -> Result<FavoriteCountResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let response = svc.post(url::GET_FAVORITE_COUNT_URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
