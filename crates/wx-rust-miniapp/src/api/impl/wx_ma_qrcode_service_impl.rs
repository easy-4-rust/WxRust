//! 二维码相关服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaQrcodeServiceImpl`：
//! 全部方法委托门面默认实现（门面已镜像 Java Impl 的 URL/请求体/字节响应
//! 执行器 `QrcodeBytesRequestExecutor` 与文件保存语义）。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g2_services::WxMaQrcodeService;
use crate::bean::WxMaCodeLineColor;

/// 二维码相关服务实现。
pub struct WxMaQrcodeServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaQrcodeServiceImpl {
    /// 构建二维码服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaQrcodeService for WxMaQrcodeServiceImpl {
    /// 对应 Java `WxMaQrcodeServiceImpl.createQrcodeBytes`。
    async fn create_qrcode_bytes(
        &self,
        path: &str,
        width: i32,
    ) -> Result<Vec<u8>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_qrcode_bytes(path, width).await
    }

    /// 对应 Java `WxMaQrcodeServiceImpl.createQrcode(String, int, String)`。
    async fn create_qrcode_to_path(
        &self,
        path: &str,
        width: i32,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_qrcode_to_path(path, width, file_path).await
    }

    /// 对应 Java `WxMaQrcodeServiceImpl.createQrcode(String, int)`。
    async fn create_qrcode(&self, path: &str, width: i32) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_qrcode(path, width).await
    }

    /// 对应 Java `WxMaQrcodeServiceImpl.createQrcode(String, String)`。
    async fn create_qrcode_default_to_path(
        &self,
        path: &str,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_qrcode_default_to_path(path, file_path).await
    }

    /// 对应 Java `WxMaQrcodeServiceImpl.createQrcode(String)`（width 默认 430）。
    async fn create_qrcode_default(&self, path: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_qrcode_default(path).await
    }

    /// 对应 Java `WxMaQrcodeServiceImpl.createWxaCodeBytes`。
    async fn create_wxa_code_bytes(
        &self,
        path: &str,
        env_version: Option<&str>,
        width: i32,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<Vec<u8>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_wxa_code_bytes(path, env_version, width, auto_color, line_color, is_hyaline)
            .await
    }

    /// 对应 Java
    /// `WxMaQrcodeServiceImpl.createWxaCode(String, String, int, String,
    /// boolean, WxMaCodeLineColor, boolean)`。
    async fn create_wxa_code_to_path(
        &self,
        path: &str,
        env_version: Option<&str>,
        width: i32,
        file_path: &str,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_wxa_code_to_path(
            path,
            env_version,
            width,
            file_path,
            auto_color,
            line_color,
            is_hyaline,
        )
        .await
    }

    /// 对应 Java
    /// `WxMaQrcodeServiceImpl.createWxaCode(String, String, int, boolean,
    /// WxMaCodeLineColor, boolean)`。
    async fn create_wxa_code(
        &self,
        path: &str,
        env_version: Option<&str>,
        width: i32,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_wxa_code(path, env_version, width, auto_color, line_color, is_hyaline)
            .await
    }

    /// 对应 Java `WxMaQrcodeServiceImpl.createWxaCode(String, int, String)`。
    async fn create_wxa_code_width_to_path(
        &self,
        path: &str,
        width: i32,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_wxa_code_width_to_path(path, width, file_path)
            .await
    }

    /// 对应 Java `WxMaQrcodeServiceImpl.createWxaCode(String, int)`。
    async fn create_wxa_code_default(
        &self,
        path: &str,
        width: i32,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_wxa_code_default(path, width).await
    }

    /// 对应 Java `WxMaQrcodeServiceImpl.createWxaCode(String, String)`。
    async fn create_wxa_code_simple_to_path(
        &self,
        path: &str,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_wxa_code_simple_to_path(path, file_path).await
    }

    /// 对应 Java `WxMaQrcodeServiceImpl.createWxaCode(String)`。
    async fn create_wxa_code_default_simple(&self, path: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_wxa_code_default_simple(path).await
    }

    /// 对应 Java `WxMaQrcodeServiceImpl.createWxaCodeUnlimitBytes`。
    async fn create_wxa_code_unlimit_bytes(
        &self,
        scene: &str,
        page: &str,
        check_path: bool,
        env_version: Option<&str>,
        width: i32,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<Vec<u8>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_wxa_code_unlimit_bytes(
            scene,
            page,
            check_path,
            env_version,
            width,
            auto_color,
            line_color,
            is_hyaline,
        )
        .await
    }

    /// 对应 Java
    /// `WxMaQrcodeServiceImpl.createWxaCodeUnlimit(String, String, String,
    /// boolean, String, int, boolean, WxMaCodeLineColor, boolean)`。
    async fn create_wxa_code_unlimit_to_path(
        &self,
        scene: &str,
        page: &str,
        file_path: &str,
        check_path: bool,
        env_version: Option<&str>,
        width: i32,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_wxa_code_unlimit_to_path(
            scene,
            page,
            file_path,
            check_path,
            env_version,
            width,
            auto_color,
            line_color,
            is_hyaline,
        )
        .await
    }

    /// 对应 Java
    /// `WxMaQrcodeServiceImpl.createWxaCodeUnlimit(String, String, boolean,
    /// String, int, boolean, WxMaCodeLineColor, boolean)`。
    async fn create_wxa_code_unlimit(
        &self,
        scene: &str,
        page: &str,
        check_path: bool,
        env_version: Option<&str>,
        width: i32,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_wxa_code_unlimit(
            scene,
            page,
            check_path,
            env_version,
            width,
            auto_color,
            line_color,
            is_hyaline,
        )
        .await
    }

    /// 对应 Java `WxMaQrcodeServiceImpl.createWxaCodeUnlimit(String, String,
    /// String)`。
    async fn create_wxa_code_unlimit_default_to_path(
        &self,
        scene: &str,
        page: &str,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_wxa_code_unlimit_default_to_path(scene, page, file_path)
            .await
    }

    /// 对应 Java `WxMaQrcodeServiceImpl.createWxaCodeUnlimit(String, String)`。
    async fn create_wxa_code_unlimit_default(
        &self,
        scene: &str,
        page: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.create_wxa_code_unlimit_default(scene, page).await
    }
}
