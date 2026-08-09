//! 小程序 Scheme 码服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaSchemeServiceImpl`：
//! POST 生成 scheme，响应 errcode 校验（由执行引擎覆盖）+ 取 `openlink` 字段。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g2_services::WxMaSchemeService;
use crate::bean::scheme::{WxMaGenerateNfcSchemeRequest, WxMaGenerateSchemeRequest};
use crate::enums::g2_urls::url_g2_content::scheme as scheme_url;

/// 小程序 Scheme 码服务实现。
pub struct WxMaSchemeServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaSchemeServiceImpl {
    /// 构建 Scheme 码服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaSchemeService for WxMaSchemeServiceImpl {
    /// 对应 Java `WxMaSchemeServiceImpl.generate`。
    ///
    /// POST `/wxa/generatescheme`；Java 的显式 errcode 校验已被执行引擎
    /// 覆盖（同一语义），响应含 `openlink` 字段时返回其值，否则抛
    /// `openlink 字段缺失`（Java `getAsString` 语义）。
    async fn generate(
        &self,
        request: &WxMaGenerateSchemeRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(&scheme_url::generate_scheme_url(config.as_ref()), &body)
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("openlink")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "openlink 字段缺失"))
    }

    /// 对应 Java `WxMaSchemeServiceImpl.generateNFC`。
    ///
    /// POST `/wxa/generatenfcscheme`；Java 的显式 errcode 校验已被执行引擎
    /// 覆盖（同一语义），响应含 `openlink` 字段时返回其值。
    async fn generate_nfc(
        &self,
        request: &WxMaGenerateNfcSchemeRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(&scheme_url::generate_nfc_scheme_url(config.as_ref()), &body)
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("openlink")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "openlink 字段缺失"))
    }
}
