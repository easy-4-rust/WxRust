//! OCR 识别服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaOcrServiceImpl`：Java 侧
//! 无独立接口文件，直接实现 common 接口 `me.chanjar.weixin.common.service.
//! WxOcrService`，Rust 侧对应直接实现 `wx_rust_common::service::WxOcrService`
//! trait（本实现即 common trait 的首个实现者，mp 域实现的是自己的独立 trait）。
//!
//! 与 Java 逐方法对齐的语义：
//! - 六个 URL 版方法（身份证/银行卡/行驶证/驾驶证/营业执照/通用印刷体）均为
//!   **POST**：imgUrl 先经 `URLEncoder.encode(imgUrl, UTF_8)`（Rust 侧
//!   `url::form_urlencoded::byte_serialize`，同语义：空格转 `+`、`~` 转
//!   `%7E`、其余 `%XX` 大写）后经 `String.format(常量, imgUrl)` 填入
//!   `?img_url=%s` query 参数，请求体为空（Java `post(url, (String) null)`）。
//! - 文件版方法（`idCard(File)` 等）走 common `OcrDiscernRequestExecutor`
//!   multipart 上传，common trait 仅保留 URL 版签名，故本实现只覆盖 URL 版
//!   （对应 Java `WxOcrService` 的 URL 方法族）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::bean::ocr::{
    WxOcrBankCardResult, WxOcrBizLicenseResult, WxOcrCommResult, WxOcrDrivingLicenseResult,
    WxOcrDrivingResult, WxOcrIdCardResult,
};
use wx_rust_common::error::WxErrorException;
use wx_rust_common::service::WxOcrService;

use crate::api::WxMaService;
use crate::enums::g4_urls::url_g4_ability::ocr as ocr_url;

/// 小程序 OCR 识别服务实现（对应 Java `WxMaOcrServiceImpl`）。
pub struct WxMaOcrServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaOcrServiceImpl {
    /// 构建 OCR 识别服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// URL 编码（对应 Java `URLEncoder.encode(imgUrl, StandardCharsets.UTF_8)`；
    /// Java 捕获 `UnsupportedEncodingException` 后忽略——UTF-8 恒存在，Rust
    /// 无此分支）。
    fn encode_img_url(img_url: &str) -> String {
        url::form_urlencoded::byte_serialize(img_url.as_bytes()).collect::<String>()
    }

    /// POST 图片 URL 查询（对应 Java
    /// `service.post(String.format(URL, imgUrl), (String) null)`：URL 已携带
    /// 编码后的 img_url，请求体为空字符串）。
    async fn post_img(svc: &dyn WxMaService, url: &str) -> Result<String, WxErrorException> {
        svc.post(url, "").await
    }
}

#[async_trait]
impl WxOcrService for WxMaOcrServiceImpl {
    /// 身份证识别（对应 Java `WxMaOcrServiceImpl.idCard(String imgUrl)`）。
    async fn ocr_id_card(&self, img_url: &str) -> Result<WxOcrIdCardResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let encoded = Self::encode_img_url(img_url);
        let response = Self::post_img(
            svc.as_ref(),
            &ocr_url::id_card_url(config.as_ref(), &encoded),
        )
        .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 银行卡识别（对应 Java `WxMaOcrServiceImpl.bankCard(String imgUrl)`）。
    async fn ocr_bank_card(&self, img_url: &str) -> Result<WxOcrBankCardResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let encoded = Self::encode_img_url(img_url);
        let response = Self::post_img(
            svc.as_ref(),
            &ocr_url::bank_card_url(config.as_ref(), &encoded),
        )
        .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 行驶证识别（对应 Java `WxMaOcrServiceImpl.driving(String imgUrl)`）。
    async fn ocr_driving(&self, img_url: &str) -> Result<WxOcrDrivingResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let encoded = Self::encode_img_url(img_url);
        let response = Self::post_img(
            svc.as_ref(),
            &ocr_url::driving_url(config.as_ref(), &encoded),
        )
        .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 驾驶证识别（对应 Java `WxMaOcrServiceImpl.drivingLicense(String imgUrl)`）。
    async fn ocr_driving_license(
        &self,
        img_url: &str,
    ) -> Result<WxOcrDrivingLicenseResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let encoded = Self::encode_img_url(img_url);
        let response = Self::post_img(
            svc.as_ref(),
            &ocr_url::driving_license_url(config.as_ref(), &encoded),
        )
        .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 营业执照识别（对应 Java `WxMaOcrServiceImpl.bizLicense(String imgUrl)`）。
    async fn ocr_biz_license(
        &self,
        img_url: &str,
    ) -> Result<WxOcrBizLicenseResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let encoded = Self::encode_img_url(img_url);
        let response = Self::post_img(
            svc.as_ref(),
            &ocr_url::biz_license_url(config.as_ref(), &encoded),
        )
        .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 通用印刷体识别（对应 Java `WxMaOcrServiceImpl.comm(String imgUrl)`）。
    async fn ocr_comm(&self, img_url: &str) -> Result<WxOcrCommResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let encoded = Self::encode_img_url(img_url);
        let response =
            Self::post_img(svc.as_ref(), &ocr_url::comm_url(config.as_ref(), &encoded)).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
