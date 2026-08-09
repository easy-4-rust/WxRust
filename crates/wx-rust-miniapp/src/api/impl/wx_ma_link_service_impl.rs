//! URL Link / Short Link 服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaLinkServiceImpl`：
//! 全部方法委托门面默认实现（门面已镜像 Java Impl 的 URL/请求体/响应解析）。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g2_services::WxMaLinkService;
use crate::bean::shortlink::GenerateShortLinkRequest;
use crate::bean::urllink::{GenerateUrlLinkRequest, QueryUrlLinkRequest, QueryUrlLinkResponse};

/// 小程序 URL Link / Short Link 服务实现。
pub struct WxMaLinkServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaLinkServiceImpl {
    /// 构建链接服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaLinkService for WxMaLinkServiceImpl {
    /// 对应 Java `WxMaLinkServiceImpl.generateUrlLink`。
    ///
    /// 响应含 `url_link` 字段时返回其值，否则抛 `无url_link`
    /// （Java `new WxErrorException("无url_link")`）。
    async fn generate_url_link(
        &self,
        request: &GenerateUrlLinkRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.generate_url_link(request).await
    }

    /// 对应 Java `WxMaLinkServiceImpl.generateShortLink`。
    ///
    /// 响应含 `link` 字段时返回其值，否则抛 `无link`
    /// （Java `new WxErrorException("无link")`）。
    async fn generate_short_link(
        &self,
        request: &GenerateShortLinkRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.generate_short_link(request).await
    }

    /// 对应 Java `WxMaLinkServiceImpl.queryUrlLink`。
    async fn query_url_link(
        &self,
        request: &QueryUrlLinkRequest,
    ) -> Result<QueryUrlLinkResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.query_url_link(request).await
    }
}
