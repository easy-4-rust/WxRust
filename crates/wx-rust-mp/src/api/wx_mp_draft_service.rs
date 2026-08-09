//! WxMpDraft服务
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpDraftService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::draft::{WxMpAddDraft, WxMpDraftInfo, WxMpDraftList, WxMpUpdateDraft};

/// WxMpDraft服务。
#[async_trait]
pub trait WxMpDraftService: Send + Sync {
    async fn add_draft(&self, add_draft: &WxMpAddDraft) -> Result<String, WxErrorException>;

    async fn update_draft(&self, update_draft: &WxMpUpdateDraft) -> Result<bool, WxErrorException>;

    async fn get_draft(&self, media_id: &str) -> Result<WxMpDraftInfo, WxErrorException>;

    async fn del_draft(&self, media_id: &str) -> Result<bool, WxErrorException>;

    async fn list_draft(&self, offset: i32, count: i32) -> Result<WxMpDraftList, WxErrorException>;

    async fn count_draft(&self) -> Result<i64, WxErrorException>;
}
