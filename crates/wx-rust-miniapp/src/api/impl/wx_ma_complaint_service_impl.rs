//! 小程序交易投诉服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaComplaintServiceImpl`：
//! JSON 接口逐方法对齐；`uploadResponseImage` 的 multipart 上传对应 Java
//! `wxMaService.upload(UPLOAD_RESPONSE_IMAGE_URL, CommonUploadParam.fromFile(
//! "image", file))`（字段名 `image`），响应取 `media_id`；InputStream 重载
//! 以字节 + 文件名表达（Java 先落临时文件再走 File 路径）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::enums::WxType;
use wx_rust_common::error::{WxError, WxErrorException};

use crate::api::WxMaService;
use crate::api::g4_services::WxMaComplaintService;
use crate::bean::complaint::{
    WxMaComplaintDetailRequest, WxMaComplaintDetailResult, WxMaComplaintNotifyUrlRequest,
    WxMaComplaintNotifyUrlResult, WxMaComplaintRequest, WxMaComplaintResult, WxMaCompleteRequest,
    WxMaNegotiationHistoryRequest, WxMaNegotiationHistoryResult, WxMaResponseRequest,
};
use crate::config::DEFAULT_API_HOST_URL;
use crate::enums::g4_urls::url_g4_ability::complaint as complaint_url;

/// 小程序交易投诉服务实现。
pub struct WxMaComplaintServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaComplaintServiceImpl {
    /// 构建交易投诉服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 序列化请求对象为 JSON（对应 Java `request.toJson()`）。
    fn to_json<T: serde::Serialize>(request: &T) -> Result<String, WxErrorException> {
        serde_json::to_string(request).map_err(WxErrorException::from)
    }

    /// POST 请求并解析响应（对应 Java `post` + gson `fromJson`）。
    async fn post_as<T>(
        svc: &dyn WxMaService,
        url: &str,
        post_body: &str,
    ) -> Result<T, WxErrorException>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = svc.post(url, post_body).await?;
        serde_json::from_str(&response).map_err(WxErrorException::from)
    }

    /// 构建带 access_token 的上传 URL（对应 Java 执行引擎的 token 注入 +
    /// 自定义域名替换）。
    async fn build_upload_url(
        svc: &dyn WxMaService,
        url: &str,
    ) -> Result<String, WxErrorException> {
        let config = svc.wx_ma_config();
        let access_token = svc.get_access_token().await?;
        let effective_host = config.effective_api_host_url();
        let url = if effective_host != DEFAULT_API_HOST_URL {
            url.replace(DEFAULT_API_HOST_URL, &effective_host)
        } else {
            url.to_string()
        };
        Ok(format!("{url}?access_token={access_token}"))
    }

    /// multipart 上传（字段名 `image`，对应 Java
    /// `CommonUploadParam.fromFile("image", imageFile)`），校验 errcode 后
    /// 解析 `media_id` 返回。
    async fn upload_image(
        svc: &dyn WxMaService,
        url: &str,
        content: Vec<u8>,
        file_name: Option<&str>,
    ) -> Result<String, WxErrorException> {
        let upload_url = Self::build_upload_url(svc, url).await?;
        let part = reqwest::multipart::Part::bytes(content)
            .file_name(file_name.unwrap_or_default().to_string());
        let form = reqwest::multipart::Form::new().part("image", part);
        let text = svc
            .http_client()
            .post(&upload_url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?
            .text()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?;
        let error = WxError::from_json_with_type(&text, Some(WxType::MiniApp));
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        let json: serde_json::Value =
            serde_json::from_str(&text).map_err(WxErrorException::from)?;
        json.get("media_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "media_id 字段缺失"))
    }
}

#[async_trait]
impl WxMaComplaintService for WxMaComplaintServiceImpl {
    /// 查询投诉单列表（对应 Java `WxMaComplaintServiceImpl.queryComplaints`）。
    async fn query_complaints(
        &self,
        request: &WxMaComplaintRequest,
    ) -> Result<WxMaComplaintResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &complaint_url::query_complaints_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 查询投诉单详情（对应 Java `WxMaComplaintServiceImpl.getComplaint`）。
    async fn get_complaint(
        &self,
        request: &WxMaComplaintDetailRequest,
    ) -> Result<WxMaComplaintDetailResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &complaint_url::get_complaint_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 查询投诉协商历史（对应 Java
    /// `WxMaComplaintServiceImpl.queryNegotiationHistorys`）。
    async fn query_negotiation_historys(
        &self,
        request: &WxMaNegotiationHistoryRequest,
    ) -> Result<WxMaNegotiationHistoryResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &complaint_url::query_negotiation_history_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 创建投诉通知回调地址（对应 Java
    /// `WxMaComplaintServiceImpl.addComplaintNotifyUrl`）。
    async fn add_complaint_notify_url(
        &self,
        request: &WxMaComplaintNotifyUrlRequest,
    ) -> Result<WxMaComplaintNotifyUrlResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &complaint_url::add_complaint_notify_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 查询投诉通知回调地址（对应 Java
    /// `WxMaComplaintServiceImpl.getComplaintNotifyUrl`，GET 无 query）。
    async fn get_complaint_notify_url(
        &self,
    ) -> Result<WxMaComplaintNotifyUrlResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .get(
                &complaint_url::get_complaint_notify_url(config.as_ref()),
                "",
            )
            .await?;
        serde_json::from_str(&response).map_err(WxErrorException::from)
    }

    /// 更新投诉通知回调地址（对应 Java
    /// `WxMaComplaintServiceImpl.updateComplaintNotifyUrl`）。
    async fn update_complaint_notify_url(
        &self,
        request: &WxMaComplaintNotifyUrlRequest,
    ) -> Result<WxMaComplaintNotifyUrlResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &complaint_url::update_complaint_notify_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 删除投诉通知回调地址（对应 Java
    /// `WxMaComplaintServiceImpl.deleteComplaintNotifyUrl`，请求体 `{}`）。
    async fn delete_complaint_notify_url(&self) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        svc.post(
            &complaint_url::delete_complaint_notify_url(config.as_ref()),
            "{}",
        )
        .await?;
        Ok(())
    }

    /// 提交回复（对应 Java `WxMaComplaintServiceImpl.submitResponse`）。
    async fn submit_response(&self, request: &WxMaResponseRequest) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        svc.post(
            &complaint_url::submit_response_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await?;
        Ok(())
    }

    /// 反馈处理完成（对应 Java `WxMaComplaintServiceImpl.complete`）。
    async fn complete(&self, request: &WxMaCompleteRequest) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        svc.post(
            &complaint_url::complete_complaint_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await?;
        Ok(())
    }

    /// 商户上传反馈图片（文件路径版，对应 Java
    /// `WxMaComplaintServiceImpl.uploadResponseImage(File)`，返回媒体文件
    /// 标识 ID）。
    async fn upload_response_image(&self, image_path: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let file_name = std::path::Path::new(image_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string());
        let content = std::fs::read(image_path)
            .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?;
        let config = svc.wx_ma_config();
        Self::upload_image(
            svc.as_ref(),
            &complaint_url::upload_response_image_url(config.as_ref()),
            content,
            file_name.as_deref(),
        )
        .await
    }

    /// 商户上传反馈图片（字节版，对应 Java
    /// `WxMaComplaintServiceImpl.uploadResponseImage(InputStream, String)`）。
    async fn upload_response_image_bytes(
        &self,
        content: Vec<u8>,
        file_name: Option<&str>,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::upload_image(
            svc.as_ref(),
            &complaint_url::upload_response_image_url(config.as_ref()),
            content,
            file_name,
        )
        .await
    }
}
