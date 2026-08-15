//! WxMpMaterialService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpMaterialServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpMaterialService, WxMpService};
use crate::bean::material::{
    WxMediaImgUploadResult, WxMpMaterial, WxMpMaterialCountResult, WxMpMaterialFileBatchGetResult,
    WxMpMaterialNews, WxMpMaterialNewsBatchGetResult, WxMpMaterialUploadResult,
    WxMpMaterialVideoInfoResult,
};
use crate::enums::wx_mp_api_url::material as material_url;
use wx_rust_common::bean::result::WxMediaUploadResult;

/// 公众号MaterialService实现。
pub struct WxMpMaterialServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpMaterialServiceImpl {
    /// 构建 公众号MaterialService。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }

    /// 校验响应 errcode 是否为 0。
    fn err_code_is_zero(json: &str) -> Result<bool, WxErrorException> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(value
            .get("errcode")
            .map(|v| v.as_i64() == Some(0))
            .unwrap_or(false))
    }

    /// multipart 上传单个文件字段并返回响应文本。
    async fn upload_file(
        svc: &dyn WxMpService,
        url: &str,
        field: &str,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        let part = reqwest::multipart::Part::bytes(
            std::fs::read(file_path)
                .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?,
        )
        .file_name("file");
        let form = reqwest::multipart::Form::new().part(field.to_string(), part);
        Self::send_form(svc, url, form).await
    }

    /// 发送 multipart 表单并校验 errcode。
    async fn send_form(
        svc: &dyn WxMpService,
        url: &str,
        form: reqwest::multipart::Form,
    ) -> Result<String, WxErrorException> {
        let text = svc
            .http_client()
            .post(url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?
            .text()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?;
        let error = wx_rust_common::error::WxError::from_json_with_type(
            &text,
            Some(wx_rust_common::enums::WxType::Mp),
        );
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        Ok(text)
    }
}

#[async_trait]
impl WxMpMaterialService for WxMpMaterialServiceImpl {
    async fn media_upload(
        &self,
        media_type: &str,
        file_path: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let token = svc.get_access_token().await?;
        let url = material_url::media_upload(config.as_ref(), media_type);
        let response = Self::upload_file(
            svc.as_ref(),
            &format!("{url}?access_token={token}"),
            "media",
            file_path,
        )
        .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn media_download(&self, media_id: &str) -> Result<Vec<u8>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let token = svc.get_access_token().await?;
        let url = material_url::media_get(config.as_ref());
        let url = format!("{url}?access_token={token}&media_id={media_id}");
        let bytes = svc
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("素材下载失败: {e}")))?
            .bytes()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("素材下载失败: {e}")))?;
        Ok(bytes.to_vec())
    }

    async fn media_img_upload(
        &self,
        file_path: &str,
    ) -> Result<WxMediaImgUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let token = svc.get_access_token().await?;
        let url = material_url::media_img_upload(config.as_ref());
        let response = Self::upload_file(
            svc.as_ref(),
            &format!("{url}?access_token={token}"),
            "img",
            file_path,
        )
        .await?;
        WxMediaImgUploadResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn material_file_upload(
        &self,
        media_type: &str,
        material: &WxMpMaterial,
    ) -> Result<WxMpMaterialUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let token = svc.get_access_token().await?;
        let url = material_url::material_add(config.as_ref(), media_type);
        let url = format!("{url}?access_token={token}");
        let file_path = material
            .file
            .as_deref()
            .ok_or_else(|| WxErrorException::from_code(-99, "文件路径为空"))?;
        let mut form = reqwest::multipart::Form::new().part(
            "media",
            reqwest::multipart::Part::bytes(
                std::fs::read(file_path)
                    .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?,
            )
            .file_name("media"),
        );
        if media_type == "video" {
            let desc = serde_json::json!({"title": material.video_title, "introduction": material.video_introduction});
            form = form.text("description", desc.to_string());
        }
        let text = Self::send_form(svc.as_ref(), &url, form).await?;
        WxMpMaterialUploadResult::from_json(&text).map_err(WxErrorException::Serde)
    }

    async fn material_video_info(
        &self,
        media_id: &str,
    ) -> Result<WxMpMaterialVideoInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"media_id": media_id});
        let response = svc
            .post(
                &material_url::material_get(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpMaterialVideoInfoResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn material_news_info(
        &self,
        media_id: &str,
    ) -> Result<WxMpMaterialNews, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"media_id": media_id});
        let response = svc
            .post(
                &material_url::material_get(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpMaterialNews::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn material_delete(&self, media_id: &str) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"media_id": media_id});
        let response = svc
            .post(
                &material_url::material_del(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        Self::err_code_is_zero(&response)
    }

    async fn material_count(&self) -> Result<WxMpMaterialCountResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .get(&material_url::material_count(config.as_ref()), "")
            .await?;
        WxMpMaterialCountResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn material_news_batch_get(
        &self,
        offset: i32,
        count: i32,
    ) -> Result<WxMpMaterialNewsBatchGetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"type": "news", "offset": offset, "count": count});
        let response = svc
            .post(
                &material_url::material_batch_get(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpMaterialNewsBatchGetResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn material_file_batch_get(
        &self,
        r#type: &str,
        offset: i32,
        count: i32,
    ) -> Result<WxMpMaterialFileBatchGetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"type": r#type, "offset": offset, "count": count});
        let response = svc
            .post(
                &material_url::material_batch_get(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpMaterialFileBatchGetResult::from_json(&response).map_err(WxErrorException::Serde)
    }
}
