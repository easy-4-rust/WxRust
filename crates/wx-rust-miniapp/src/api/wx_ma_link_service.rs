//! URL Link / Short Link 服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaLinkService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shortlink::GenerateShortLinkRequest;
use crate::bean::urllink::{GenerateUrlLinkRequest, QueryUrlLinkRequest, QueryUrlLinkResponse};

/// 小程序 URL Link / Short Link 服务。
#[async_trait]
pub trait WxMaLinkService: Send + Sync {
    /// 获取小程序 URL Link（对应 Java
    /// `WxMaLinkService.generateUrlLink(GenerateUrlLinkRequest)`）。
    ///
    /// POST `/wxa/generate_urllink`，响应含 `url_link` 字段时返回其值。
    async fn generate_url_link(
        &self,
        request: &GenerateUrlLinkRequest,
    ) -> Result<String, WxErrorException>;

    /// 获取小程序 Short Link（对应 Java
    /// `WxMaLinkService.generateShortLink(GenerateShortLinkRequest)`）。
    ///
    /// POST `/wxa/genwxashortlink`，响应含 `link` 字段时返回其值。
    async fn generate_short_link(
        &self,
        request: &GenerateShortLinkRequest,
    ) -> Result<String, WxErrorException>;

    /// 查询 URL Link（对应 Java
    /// `WxMaLinkService.queryUrlLink(QueryUrlLinkRequest)`）。
    ///
    /// POST `/wxa/query_urllink`。
    async fn query_url_link(
        &self,
        request: &QueryUrlLinkRequest,
    ) -> Result<QueryUrlLinkResponse, WxErrorException>;
}
