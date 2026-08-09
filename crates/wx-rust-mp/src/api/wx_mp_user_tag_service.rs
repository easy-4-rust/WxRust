//! WxMpUserTag服务
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpUserTagService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::tag::{WxTagListUser, WxUserTag};

/// WxMpUserTag服务。
#[async_trait]
pub trait WxMpUserTagService: Send + Sync {
    async fn tag_create(&self, name: &str) -> Result<WxUserTag, WxErrorException>;

    async fn tag_get(&self) -> Result<Vec<WxUserTag>, WxErrorException>;

    async fn tag_update(&self, tag_id: i64, name: &str) -> Result<bool, WxErrorException>;

    async fn tag_delete(&self, tag_id: i64) -> Result<bool, WxErrorException>;

    async fn tag_list_user(
        &self,
        tag_id: i64,
        next_openid: &str,
    ) -> Result<WxTagListUser, WxErrorException>;

    async fn user_tag_list(&self, openid: &str) -> Result<Vec<i64>, WxErrorException>;

    async fn batch_tagging(&self, tag_id: i64, openids: &[&str]) -> Result<bool, WxErrorException>;

    async fn batch_untagging(
        &self,
        tag_id: i64,
        openids: &[&str],
    ) -> Result<bool, WxErrorException>;
}
