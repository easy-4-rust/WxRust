//! 小程序订单管理服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaOrderManagementServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaOrderManagementService;
use crate::bean::order::{WxMaOrderManagementGetOrderDetailPath, WxMaOrderManagementResult};
use crate::enums::g3_urls::url_g3_shop::order_management as order_management_url;

/// 小程序订单管理服务实现。
pub struct WxMaOrderManagementServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaOrderManagementServiceImpl {
    /// 构建订单管理服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 通用请求（对应 Java `WxMaOrderManagementServiceImpl.request`）：
    /// POST 后校验 errcode 并解析为指定类型。
    async fn request<T>(svc: &dyn WxMaService, url: &str, body: &str) -> Result<T, WxErrorException>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = svc.post(url, body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxMaOrderManagementService for WxMaOrderManagementServiceImpl {
    /// 对应 Java `WxMaOrderManagementServiceImpl.getOrderDetailPath`：
    /// POST `GET_ORDER_DETAIL_PATH`（空对象）后解析响应。
    async fn get_order_detail_path(
        &self,
    ) -> Result<WxMaOrderManagementGetOrderDetailPath, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::request(
            svc.as_ref(),
            &order_management_url::get_order_detail_path_url(config.as_ref()),
            "{}",
        )
        .await
    }

    /// 对应 Java `WxMaOrderManagementServiceImpl.updateOrderDetailPath`：
    /// 构造 `{"path": path}` 后 POST `UPDATE_ORDER_DETAIL_PATH` 并解析响应。
    async fn update_order_detail_path(
        &self,
        path: &str,
    ) -> Result<WxMaOrderManagementResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "path": path }).to_string();
        Self::request(
            svc.as_ref(),
            &order_management_url::update_order_detail_path_url(config.as_ref()),
            &body,
        )
        .await
    }
}
