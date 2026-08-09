//! WxMpFreePublish服务
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpFreePublishService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::freepublish::{WxMpFreePublishInfo, WxMpFreePublishList, WxMpFreePublishStatus};

/// WxMpFreePublish服务。
#[async_trait]
pub trait WxMpFreePublishService: Send + Sync {
    async fn submit(&self, media_id: &str) -> Result<String, WxErrorException>;

    async fn get_push_status(
        &self,
        publish_id: &str,
    ) -> Result<WxMpFreePublishStatus, WxErrorException>;

    async fn delete_push(&self, article_id: &str, index: i32) -> Result<bool, WxErrorException>;

    async fn get_article_from_id(
        &self,
        article_id: &str,
    ) -> Result<WxMpFreePublishInfo, WxErrorException>;

    async fn get_publication_records(
        &self,
        offset: i32,
        count: i32,
    ) -> Result<WxMpFreePublishList, WxErrorException>;
}
