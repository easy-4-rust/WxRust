//! WxMpComment服务
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpCommentService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::comment::WxMpCommentListVo;

/// WxMpComment服务。
#[async_trait]
pub trait WxMpCommentService: Send + Sync {
    async fn open(&self, msg_data_id: &str, index: Option<i32>) -> Result<(), WxErrorException>;

    async fn close(&self, msg_data_id: &str, index: Option<i32>) -> Result<(), WxErrorException>;

    async fn list(
        &self,
        msg_data_id: &str,
        index: Option<i32>,
        begin: i32,
        count: i32,
        r#type: i32,
    ) -> Result<WxMpCommentListVo, WxErrorException>;

    async fn mark_elect(
        &self,
        msg_data_id: &str,
        index: Option<i32>,
        user_comment_id: i64,
    ) -> Result<(), WxErrorException>;

    async fn unmark_elect(
        &self,
        msg_data_id: &str,
        index: Option<i32>,
        user_comment_id: i64,
    ) -> Result<(), WxErrorException>;

    async fn delete(
        &self,
        msg_data_id: &str,
        index: Option<i32>,
        user_comment_id: i64,
    ) -> Result<(), WxErrorException>;

    async fn reply_add(
        &self,
        msg_data_id: &str,
        index: Option<i32>,
        user_comment_id: i64,
        content: &str,
    ) -> Result<(), WxErrorException>;

    async fn reply_delete(
        &self,
        msg_data_id: &str,
        index: Option<i32>,
        user_comment_id: i64,
    ) -> Result<(), WxErrorException>;
}
