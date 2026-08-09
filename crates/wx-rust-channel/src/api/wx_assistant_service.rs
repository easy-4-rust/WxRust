//! WxAssistantService（对应 Java `me.chanjar.weixin.channel.api.WxAssistantService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::window::request::{
    AddWindowProductRequest, GetWindowProductListRequest, WindowProductRequest,
};
use crate::bean::window::response::{GetWindowProductListResponse, GetWindowProductResponse};

/// 视频号助手 橱窗管理服务（对应 Java `WxAssistantService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_assistant_service_impl` 的
/// `WxAssistantServiceImpl`（Java `WxAssistantServiceImpl`）。
#[async_trait]
pub trait WxAssistantService: Send + Sync {
    /// 上架商品到橱窗（对应 Java `WxAssistantService#addWindowProduct`）。
    async fn add_window_product(
        &self,
        req: AddWindowProductRequest,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取橱窗商品详情（对应 Java `WxAssistantService#getWindowProduct`）。
    async fn get_window_product(
        &self,
        req: WindowProductRequest,
    ) -> Result<GetWindowProductResponse, WxErrorException>;

    /// 获取已添加到橱窗的商品列表（对应 Java `WxAssistantService#getWindowProductList`；
    /// 接口限制 page_size × page_index ≤ 10000，命中限制建议改用 last_buffer 顺序翻页）。
    async fn get_window_product_list(
        &self,
        req: GetWindowProductListRequest,
    ) -> Result<GetWindowProductListResponse, WxErrorException>;

    /// 下架橱窗商品（对应 Java `WxAssistantService#offWindowProduct`）。
    async fn off_window_product(
        &self,
        req: WindowProductRequest,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;
}
