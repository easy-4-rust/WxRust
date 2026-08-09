//! 小程序 Scheme 码相关服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaSchemeService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::scheme::{WxMaGenerateNfcSchemeRequest, WxMaGenerateSchemeRequest};

/// 小程序 Scheme 码相关操作服务。
#[async_trait]
pub trait WxMaSchemeService: Send + Sync {
    /// 获取小程序 scheme 码（对应 Java
    /// `WxMaSchemeService.generate(WxMaGenerateSchemeRequest)`）。
    ///
    /// POST `/wxa/generatescheme`，响应含 `openlink` 字段时返回其值。
    async fn generate(
        &self,
        request: &WxMaGenerateSchemeRequest,
    ) -> Result<String, WxErrorException>;

    /// 获取 NFC 的小程序 scheme（对应 Java
    /// `WxMaSchemeService.generateNFC(WxMaGenerateNfcSchemeRequest)`）。
    ///
    /// POST `/wxa/generatenfcscheme`，响应含 `openlink` 字段时返回其值。
    async fn generate_nfc(
        &self,
        request: &WxMaGenerateNfcSchemeRequest,
    ) -> Result<String, WxErrorException>;
}
