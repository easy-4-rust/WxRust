//! WxMpMaterialService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpMaterialService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::material::{
    WxMediaImgUploadResult, WxMpMaterial, WxMpMaterialCountResult, WxMpMaterialFileBatchGetResult,
    WxMpMaterialNews, WxMpMaterialNewsBatchGetResult, WxMpMaterialUploadResult,
    WxMpMaterialVideoInfoResult,
};
use crate::enums::wx_mp_api_url::material as material_url;
use wx_rust_common::bean::result::WxMediaUploadResult;

/// 公众号MaterialService。
#[async_trait]
pub trait WxMpMaterialService: Send + Sync {
    async fn media_upload(
        &self,
        media_type: &str,
        file_path: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException>;

    async fn media_download(&self, media_id: &str) -> Result<Vec<u8>, WxErrorException>;

    async fn media_img_upload(
        &self,
        file_path: &str,
    ) -> Result<WxMediaImgUploadResult, WxErrorException>;

    async fn material_file_upload(
        &self,
        media_type: &str,
        material: &WxMpMaterial,
    ) -> Result<WxMpMaterialUploadResult, WxErrorException>;

    async fn material_video_info(
        &self,
        media_id: &str,
    ) -> Result<WxMpMaterialVideoInfoResult, WxErrorException>;

    async fn material_news_info(
        &self,
        media_id: &str,
    ) -> Result<WxMpMaterialNews, WxErrorException>;

    async fn material_delete(&self, media_id: &str) -> Result<bool, WxErrorException>;

    async fn material_count(&self) -> Result<WxMpMaterialCountResult, WxErrorException>;

    async fn material_news_batch_get(
        &self,
        offset: i32,
        count: i32,
    ) -> Result<WxMpMaterialNewsBatchGetResult, WxErrorException>;

    async fn material_file_batch_get(
        &self,
        r#type: &str,
        offset: i32,
        count: i32,
    ) -> Result<WxMpMaterialFileBatchGetResult, WxErrorException>;
}
