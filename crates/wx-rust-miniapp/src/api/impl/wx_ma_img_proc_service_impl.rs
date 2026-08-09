//! 图像处理服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaImgProcServiceImpl`：Java
//! 侧无独立接口文件，直接实现 common 接口 `me.chanjar.weixin.common.service.
//! WxImgProcService`，Rust 侧对应直接实现 `wx_rust_common::service::
//! WxImgProcService` trait。
//!
//! 与 Java 逐方法对齐的语义：
//! - `qrcode`：**POST**，imgUrl 经 `URLEncoder.encode`（Rust 侧
//!   `url::form_urlencoded::byte_serialize`，同语义）后填入 `?img_url=%s`，
//!   请求体为空字符串（Java `post(url, "")`）。
//! - `super_resolution`：**GET**，imgUrl 编码后填入 `?img_url=%s`，无额外
//!   query（Java `get(url, null)`）。
//! - `ai_crop`：**POST**，imgUrl 编码后填入 `?img_url=%s&ratios=%s`，请求体
//!   为空字符串（Java `post(url, "")`）；ratios 为空时按 Java
//!   `StringUtils.isEmpty(ratios)` 语义置 `""`（对应 Java `aiCrop(imgUrl)`
//!   默认空串重载）。common trait 以 `ratio: f64` 表达该参数：`0.0` 视为未传
//!   （空串），否则以十进制字符串输出（如 `2.35`，与 Java 测试用例
//!   `aiCrop(url, "1,2.35")` 的数值串一致）。
//! - 文件版方法（`qrCode(File)` 等）走 common `OcrDiscernRequestExecutor`
//!   multipart 上传，common trait 仅保留 URL 版签名，故本实现只覆盖 URL 版。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::bean::imgproc::{
    WxImgProcAiCropResult, WxImgProcQrCodeResult, WxImgProcSuperResolutionResult,
};
use wx_rust_common::error::WxErrorException;
use wx_rust_common::service::WxImgProcService;

use crate::api::WxMaService;
use crate::enums::g4_urls::url_g4_ability::img_proc as img_proc_url;

/// 小程序图像处理服务实现（对应 Java `WxMaImgProcServiceImpl`）。
pub struct WxMaImgProcServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaImgProcServiceImpl {
    /// 构建图像处理服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// URL 编码（对应 Java `URLEncoder.encode(imgUrl, StandardCharsets.UTF_8)`；
    /// Java 捕获 `UnsupportedEncodingException` 后忽略——UTF-8 恒存在，Rust
    /// 无此分支）。
    fn encode_img_url(img_url: &str) -> String {
        url::form_urlencoded::byte_serialize(img_url.as_bytes()).collect::<String>()
    }
}

#[async_trait]
impl WxImgProcService for WxMaImgProcServiceImpl {
    /// 二维码/条码识别（对应 Java `WxMaImgProcServiceImpl.qrCode(String imgUrl)`）。
    async fn qrcode(&self, img_url: &str) -> Result<WxImgProcQrCodeResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let encoded = Self::encode_img_url(img_url);
        // Java `post(String.format(QRCODE, imgUrl), "")`
        let response = svc
            .post(&img_proc_url::qrcode_url(config.as_ref(), &encoded), "")
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 图片高清化（对应 Java `WxMaImgProcServiceImpl.superResolution(String imgUrl)`）。
    async fn super_resolution(
        &self,
        img_url: &str,
    ) -> Result<WxImgProcSuperResolutionResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let encoded = Self::encode_img_url(img_url);
        // Java `get(String.format(SUPER_RESOLUTION, imgUrl), null)`：query 为空
        let response = svc
            .get(
                &img_proc_url::super_resolution_url(config.as_ref(), &encoded),
                "",
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 图片智能裁剪（对应 Java `WxMaImgProcServiceImpl.aiCrop(String imgUrl,
    /// String ratios)`，以及空 ratios 时委托的 `aiCrop(String imgUrl)`）。
    async fn ai_crop(
        &self,
        img_url: &str,
        ratio: f64,
    ) -> Result<WxImgProcAiCropResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let encoded = Self::encode_img_url(img_url);
        // Java `StringUtils.isEmpty(ratios)` 时置 ""（common trait 以 f64 表达，
        // 0.0 视为未传）；非 0 时以十进制字符串输出（如 2.35）
        let ratios = if ratio == 0.0 {
            String::new()
        } else {
            format!("{ratio}")
        };
        // Java `post(String.format(AI_CROP, imgUrl, ratios), "")`
        let response = svc
            .post(
                &img_proc_url::ai_crop_url(config.as_ref(), &encoded, &ratios),
                "",
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
