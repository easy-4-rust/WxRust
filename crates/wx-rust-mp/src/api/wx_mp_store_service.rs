//! WxMpStore服务
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpStoreService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::store::{WxMpStoreBaseInfo, WxMpStoreListResult};

/// WxMpStore服务。
#[async_trait]
pub trait WxMpStoreService: Send + Sync {
    async fn add(&self, request: &WxMpStoreBaseInfo) -> Result<(), WxErrorException>;

    async fn get(&self, poi_id: &str) -> Result<WxMpStoreBaseInfo, WxErrorException>;

    async fn delete(&self, poi_id: &str) -> Result<(), WxErrorException>;

    async fn list(&self, begin: i32, limit: i32) -> Result<WxMpStoreListResult, WxErrorException>;

    async fn update(&self, request: &WxMpStoreBaseInfo) -> Result<(), WxErrorException>;

    async fn list_categories(&self) -> Result<Vec<String>, WxErrorException>;
}
