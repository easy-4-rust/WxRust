//! WxStoreFavoriteServiceImpl（对应 Java
//! `com.binarywang.wxjava.store.api.impl.WxStoreFavoriteServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxStoreService;
use crate::api::wx_store_favorite_service::WxStoreFavoriteService;
use crate::bean::favorite::FavoriteCountResponse;
use crate::enums::url_favorite as url;

/// 收藏管理服务实现。
pub struct WxStoreFavoriteServiceImpl {
    service: Weak<dyn WxStoreService>,
}

impl WxStoreFavoriteServiceImpl {
    /// 构建收藏管理服务。
    pub fn new(service: Weak<dyn WxStoreService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxStoreFavoriteService for WxStoreFavoriteServiceImpl {
    async fn get_favorite_count(&self) -> Result<FavoriteCountResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let response = svc.post(url::GET_FAVORITE_COUNT_URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
