//! WxMpMassMessage服务
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpMassMessageService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxMpMassGetResult, WxMpMassNews, WxMpMassOpenIdsMessage, WxMpMassPreviewMessage,
    WxMpMassSendResult, WxMpMassSpeedGetResult, WxMpMassTagMessage, WxMpMassUploadResult,
    WxMpMassVideo,
};

/// WxMpMassMessage服务。
#[async_trait]
pub trait WxMpMassMessageService: Send + Sync {
    async fn mass_news_upload(
        &self,
        news: &WxMpMassNews,
    ) -> Result<WxMpMassUploadResult, WxErrorException>;

    async fn mass_video_upload(
        &self,
        video: &WxMpMassVideo,
    ) -> Result<WxMpMassUploadResult, WxErrorException>;

    async fn mass_group_message_send(
        &self,
        message: &WxMpMassTagMessage,
    ) -> Result<WxMpMassSendResult, WxErrorException>;

    async fn mass_open_ids_message_send(
        &self,
        message: &WxMpMassOpenIdsMessage,
    ) -> Result<WxMpMassSendResult, WxErrorException>;

    async fn mass_message_preview(
        &self,
        preview: &WxMpMassPreviewMessage,
    ) -> Result<WxMpMassSendResult, WxErrorException>;

    async fn delete(&self, msg_id: i64, article_index: i32) -> Result<(), WxErrorException>;

    async fn message_mass_speed_get(&self) -> Result<WxMpMassSpeedGetResult, WxErrorException>;

    async fn message_mass_speed_set(&self, speed: i32) -> Result<(), WxErrorException>;

    async fn message_mass_get(&self, msg_id: i64) -> Result<WxMpMassGetResult, WxErrorException>;
}
