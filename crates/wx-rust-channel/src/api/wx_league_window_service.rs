//! WxLeagueWindowService（对应 Java `me.chanjar.weixin.channel.api.WxLeagueWindowService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::league::window::{
    AuthInfoResponse, AuthStatusResponse, ProductSearchParam, WindowProductListResponse,
    WindowProductResponse,
};

/// 优选联盟 团长合作达人管理服务（对应 Java `WxLeagueWindowService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_league_window_service_impl` 的
/// `WxLeagueWindowServiceImpl`（Java `WxLeagueWindowServiceImpl`）。
#[async_trait]
pub trait WxLeagueWindowService: Send + Sync {
    /// 添加团长商品到橱窗（对应 Java `WxLeagueWindowService#addProduct`）。
    async fn add_league_window_product(
        &self,
        appid: String,
        openfinderid: String,
        product_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 查询橱窗上团长商品列表（对应 Java `WxLeagueWindowService#listProduct`）。
    async fn list_league_window_product(
        &self,
        param: ProductSearchParam,
    ) -> Result<WindowProductListResponse, WxErrorException>;

    /// 从橱窗移除团长商品（对应 Java `WxLeagueWindowService#removeProduct`）。
    async fn remove_league_window_product(
        &self,
        appid: String,
        openfinderid: String,
        product_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 查询橱窗上团长商品详情（对应 Java `WxLeagueWindowService#getProductDetail`）。
    async fn get_league_window_product_detail(
        &self,
        appid: String,
        openfinderid: String,
        product_id: String,
    ) -> Result<WindowProductResponse, WxErrorException>;

    /// 获取达人橱窗授权链接（对应 Java `WxLeagueWindowService#getWindowAuthInfo`）。
    async fn get_window_auth_info(
        &self,
        finder_id: String,
    ) -> Result<AuthInfoResponse, WxErrorException>;

    /// 获取达人橱窗授权状态（对应 Java `WxLeagueWindowService#getWindowAuthStatus`）。
    async fn get_window_auth_status(
        &self,
        finder_id: String,
    ) -> Result<AuthStatusResponse, WxErrorException>;
}
