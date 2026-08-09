//! 二维码相关服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaQrcodeService`。
//!
//! 说明：
//! - Java 重载方法在 Rust 以门面同名函数映射（后缀 `_bytes`/`_to_path`/
//!   `_default` 等区分），本 trait 与门面命名一一对应。
//! - Java 返回 `File` 的方法在 Rust 返回文件路径 `String`（门面
//!   `save_qrcode_file` 语义）；返回字节数组的方法对应 `Vec<u8>`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::WxMaCodeLineColor;

/// 二维码相关操作服务。
///
/// 接口 A（createWxaCode）加上接口 C（createQrcode），总共生成的码数量限制
/// 为 100,000，请谨慎调用。
#[async_trait]
pub trait WxMaQrcodeService: Send + Sync {
    /// 接口 C：获取小程序页面二维码字节（对应 Java
    /// `WxMaQrcodeService.createQrcodeBytes(String, int)`）。
    ///
    /// POST `/cgi-bin/wxaapp/createwxaqrcode`，请求体 `{"path":..., "width":...}`。
    async fn create_qrcode_bytes(
        &self,
        path: &str,
        width: i32,
    ) -> Result<Vec<u8>, WxErrorException>;

    /// 接口 C：获取小程序页面二维码并保存到指定目录（对应 Java
    /// `WxMaQrcodeService.createQrcode(String, int, String)`）。
    ///
    /// filePath 为**目录**，返回实际写入的文件路径。
    async fn create_qrcode_to_path(
        &self,
        path: &str,
        width: i32,
        file_path: &str,
    ) -> Result<String, WxErrorException>;

    /// 接口 C：获取小程序页面二维码并保存为临时文件（对应 Java
    /// `WxMaQrcodeService.createQrcode(String, int)`）。
    async fn create_qrcode(&self, path: &str, width: i32) -> Result<String, WxErrorException>;

    /// 接口 C：获取小程序页面二维码并保存到指定目录（对应 Java
    /// `WxMaQrcodeService.createQrcode(String, String)`，width 默认 430）。
    async fn create_qrcode_default_to_path(
        &self,
        path: &str,
        file_path: &str,
    ) -> Result<String, WxErrorException>;

    /// 接口 C：获取小程序页面二维码（对应 Java
    /// `WxMaQrcodeService.createQrcode(String)`，width 默认 430）。
    async fn create_qrcode_default(&self, path: &str) -> Result<String, WxErrorException>;

    /// 接口 A：获取小程序码字节（对应 Java
    /// `WxMaQrcodeService.createWxaCodeBytes(String, String, int, boolean,
    /// WxMaCodeLineColor, boolean)`）。
    ///
    /// POST `/wxa/getwxacode`；env_version 为空默认 `release`，line_color 为空
    /// 默认黑色。
    async fn create_wxa_code_bytes(
        &self,
        path: &str,
        env_version: Option<&str>,
        width: i32,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<Vec<u8>, WxErrorException>;

    /// 接口 A：获取小程序码并保存到指定目录（对应 Java
    /// `WxMaQrcodeService.createWxaCode(String, String, int, String, boolean,
    /// WxMaCodeLineColor, boolean)`）。
    async fn create_wxa_code_to_path(
        &self,
        path: &str,
        env_version: Option<&str>,
        width: i32,
        file_path: &str,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<String, WxErrorException>;

    /// 接口 A：获取小程序码并保存为临时文件（对应 Java
    /// `WxMaQrcodeService.createWxaCode(String, String, int, boolean,
    /// WxMaCodeLineColor, boolean)`）。
    async fn create_wxa_code(
        &self,
        path: &str,
        env_version: Option<&str>,
        width: i32,
        auto_color: bool,
        line_color: Option<WxMaCodeLineColor>,
        is_hyaline: bool,
    ) -> Result<String, WxErrorException>;

    /// 接口 A：获取小程序码并保存到指定目录（对应 Java
    /// `WxMaQrcodeService.createWxaCode(String, int, String)`，envVersion 默认
    /// release、autoColor 默认 true、isHyaline 默认 false）。
    async fn create_wxa_code_width_to_path(
        &self,
        path: &str,
        width: i32,
        file_path: &str,
    ) -> Result<String, WxErrorException>;

    /// 接口 A：获取小程序码（对应 Java
    /// `WxMaQrcodeService.createWxaCode(String, int)`）。
    async fn create_wxa_code_default(
        &self,
        path: &str,
        width: i32,
    ) -> Result<String, WxErrorException>;

    /// 接口 A：获取小程序码并保存到指定目录（对应 Java
    /// `WxMaQrcodeService.createWxaCode(String, String)`，width 默认 430）。
    async fn create_wxa_code_simple_to_path(
        &self,
        path: &str,
        file_path: &str,
    ) -> Result<String, WxErrorException>;

    /// 接口 A：获取小程序码（对应 Java
    /// `WxMaQrcodeService.createWxaCode(String)`，width 默认 430）。
    async fn create_wxa_code_default_simple(&self, path: &str) -> Result<String, WxErrorException>;

    /// 接口 B：获取小程序码（永久有效、数量暂无限制）字节（对应 Java
    /// `WxMaQrcodeService.createWxaCodeUnlimitBytes(String, String, boolean,
    /// String, int, boolean, WxMaCodeLineColor, boolean)`）。
    ///
    /// POST `/wxa/getwxacodeunlimit`。
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
    ) -> Result<Vec<u8>, WxErrorException>;

    /// 接口 B：获取小程序码并保存到指定目录（对应 Java
    /// `WxMaQrcodeService.createWxaCodeUnlimit(String, String, String, boolean,
    /// String, int, boolean, WxMaCodeLineColor, boolean)`）。
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
    ) -> Result<String, WxErrorException>;

    /// 接口 B：获取小程序码并保存为临时文件（对应 Java
    /// `WxMaQrcodeService.createWxaCodeUnlimit(String, String, boolean, String,
    /// int, boolean, WxMaCodeLineColor, boolean)`）。
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
    ) -> Result<String, WxErrorException>;

    /// 接口 B：获取小程序码并保存到指定目录（对应 Java
    /// `WxMaQrcodeService.createWxaCodeUnlimit(String, String, String)`，
    /// checkPath 默认 true、envVersion 默认 release、width 默认 430）。
    async fn create_wxa_code_unlimit_default_to_path(
        &self,
        scene: &str,
        page: &str,
        file_path: &str,
    ) -> Result<String, WxErrorException>;

    /// 接口 B：获取小程序码（对应 Java
    /// `WxMaQrcodeService.createWxaCodeUnlimit(String, String)`，checkPath 默认
    /// true、envVersion 默认 release、width 默认 430、autoColor 默认 true）。
    async fn create_wxa_code_unlimit_default(
        &self,
        scene: &str,
        page: &str,
    ) -> Result<String, WxErrorException>;
}
