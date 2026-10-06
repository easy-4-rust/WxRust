//! WxStoreFavoriteService（对应 Java `com.binarywang.wxjava.store.api.WxStoreFavoriteService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::favorite::FavoriteCountResponse;

/// 收藏管理服务（对应 Java `WxStoreFavoriteService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_favorite_service_impl` 的
/// `WxStoreFavoriteServiceImpl`（Java `WxStoreFavoriteServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreFavoriteService: Send + Sync {
    /// 获取店铺收藏的人数（对应 Java `WxStoreFavoriteService#getFavoriteCount()`）。
    async fn get_favorite_count(&self) -> Result<FavoriteCountResponse, WxErrorException>;
}
