//! 小程序安全服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaSecurityServiceImpl`。
//! 各方法委托门面 `WxMaService`（门面已承载同一 URL/multipart 上传/响应
//! 解析实现，与 Java 委托 `service.execute(...)`/`service.post(...)`
//! 同一语义）。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMaSecurityService, WxMaService};
use crate::bean::WxMaMediaAsyncCheckResult;
use crate::bean::safety::{WxMaUserSafetyRiskRankRequest, WxMaUserSafetyRiskRankResponse};
use crate::bean::security::{
    WxMaMediaSecCheckCheckRequest, WxMaMsgSecCheckCheckRequest, WxMaMsgSecCheckCheckResponse,
};

/// 小程序安全服务实现。
pub struct WxMaSecurityServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaSecurityServiceImpl {
    /// 构建安全服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaSecurityService for WxMaSecurityServiceImpl {
    async fn check_image(&self, file_path: &str) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `checkImage(File)`：`MediaUploadRequestExecutor` multipart
        // 上传（字段 `media`）到 `IMG_SEC_CHECK_URL`，结果非 null 即返回 true
        svc.check_image_file(file_path).await
    }

    async fn check_image_url(&self, file_url: &str) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `checkImage(String)`：先下载到临时目录
        // （`FileUtils.copyURLToFile`，失败抛 -1 "文件地址读取异常"）再
        // 委托 `checkImage(File)`
        svc.check_image_url(file_url).await
    }

    async fn check_message(&self, msg_string: &str) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `checkMessage(String)`：POST `MSG_SEC_CHECK_URL`，
        // 请求体 `{"content": ...}`，成功即返回 true
        svc.check_message(msg_string).await
    }

    async fn check_message_with_request(
        &self,
        msg_request: &WxMaMsgSecCheckCheckRequest,
    ) -> Result<WxMaMsgSecCheckCheckResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `checkMessage(WxMaMsgSecCheckCheckRequest)`：POST 后
        // `parseErrorResponse` 校验 errcode（执行引擎已覆盖同一语义），
        // 再解析为 `WxMaMsgSecCheckCheckResponse`
        svc.check_message_with_request(msg_request).await
    }

    async fn media_check_async(
        &self,
        media_url: &str,
        media_type: i32,
    ) -> Result<WxMaMediaAsyncCheckResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `mediaCheckAsync(String, int)`：POST `MEDIA_CHECK_ASYNC_URL`，
        // 请求体 `{"media_url":..., "media_type":...}`
        svc.media_check_async(media_url, media_type).await
    }

    async fn media_check_async_with_request(
        &self,
        request: &WxMaMediaSecCheckCheckRequest,
    ) -> Result<WxMaMediaAsyncCheckResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `mediaCheckAsync(WxMaMediaSecCheckCheckRequest)`：POST 后
        // `parseErrorResponse` 校验 errcode（执行引擎已覆盖同一语义）
        svc.media_check_async_with_request(request).await
    }

    async fn get_user_risk_rank(
        &self,
        request: &WxMaUserSafetyRiskRankRequest,
    ) -> Result<WxMaUserSafetyRiskRankResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getUserRiskRank`：POST `GET_USER_RISK_RANK` 后显式校验
        // errcode（执行引擎已覆盖同一语义），再
        // `WxMaUserSafetyRiskRankResponse.fromJson`
        svc.get_user_risk_rank(request).await
    }
}
