//! 小程序交易组件-图片上传服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShopImgServiceImpl`。
//! Java 经 `MinishopUploadRequestCustomizeExecutor` 以 multipart 表单上传：
//! 表单字段 `resp_type`、`upload_type`（文件 0 / 链接 1）、`media`（文件）或
//! `img_url`（链接）；响应 errcode != 0 抛错，否则解析
//! `WxMinishopImageUploadCustomizeResult`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::bean::result::WxMinishopImageUploadCustomizeResult;
use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaShopImgService;
use crate::config::DEFAULT_API_HOST_URL;
use crate::enums::g3_urls::url_g3_shop::shop_img as img_url;

/// 小程序交易组件-图片上传服务实现。
pub struct WxMaShopImgServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShopImgServiceImpl {
    /// 构建图片上传服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 注入 access_token 并做自定义域名替换（对应 Java `executeInternal` 的
    /// token 注入 + `getEffectiveApiHostUrl()` 替换语义）。
    async fn build_url(svc: &dyn WxMaService, url: &str) -> Result<String, WxErrorException> {
        let config = svc.wx_ma_config();
        let access_token = svc.get_access_token().await?;
        let effective_host = config.effective_api_host_url();
        let url = if effective_host != DEFAULT_API_HOST_URL {
            url.replace(DEFAULT_API_HOST_URL, &effective_host)
        } else {
            url.to_string()
        };
        // Java `execute`：uri 已有查询参数时以 `&` 追加 access_token
        let token_param = if url.contains('?') {
            format!("&access_token={access_token}")
        } else {
            format!("?access_token={access_token}")
        };
        Ok(format!("{url}{token_param}"))
    }

    /// multipart 表单上传并解析结果（对应 Java
    /// `JoddHttpMinishopMediaUploadRequestCustomizeExecutor.execute`）。
    async fn upload_form(
        svc: &dyn WxMaService,
        url: &str,
        form: reqwest::multipart::Form,
    ) -> Result<WxMinishopImageUploadCustomizeResult, WxErrorException> {
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
            Some(wx_rust_common::enums::WxType::MiniApp),
        );
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        serde_json::from_str(&text).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxMaShopImgService for WxMaShopImgServiceImpl {
    /// 对应 Java `WxMaShopImgServiceImpl.uploadImg(File)`：
    /// respType 固定 "0"，multipart 上传 media 文件。
    async fn upload_img(
        &self,
        file_path: &str,
    ) -> Result<WxMinishopImageUploadCustomizeResult, WxErrorException> {
        self.upload_img_with_resp_type(file_path, "0").await
    }

    /// 对应 Java `WxMaShopImgServiceImpl.uploadImg(File, String)`：
    /// 表单携带 `resp_type`、`upload_type=0` 与 `media` 文件。
    async fn upload_img_with_resp_type(
        &self,
        file_path: &str,
        resp_type: &str,
    ) -> Result<WxMinishopImageUploadCustomizeResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let url = Self::build_url(svc.as_ref(), &img_url::img_upload_url(config.as_ref())).await?;
        let file_bytes = std::fs::read(file_path)
            .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "file".to_string());
        let media = reqwest::multipart::Part::bytes(file_bytes).file_name(file_name);
        let form = reqwest::multipart::Form::new()
            .text("resp_type", resp_type.to_string())
            .text("upload_type", "0".to_string())
            .part("media", media);
        Self::upload_form(svc.as_ref(), &url, form).await
    }

    /// 对应 Java `WxMaShopImgServiceImpl.uploadImg(String, String)`：
    /// 表单携带 `resp_type`、`upload_type=1` 与 `img_url` 图片链接。
    async fn upload_img_from_url(
        &self,
        img_url: &str,
        resp_type: &str,
    ) -> Result<WxMinishopImageUploadCustomizeResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let url = Self::build_url(svc.as_ref(), &img_url::img_upload_url(config.as_ref())).await?;
        let form = reqwest::multipart::Form::new()
            .text("resp_type", resp_type.to_string())
            .text("upload_type", "1".to_string())
            .text("img_url", img_url.to_string());
        Self::upload_form(svc.as_ref(), &url, form).await
    }
}
