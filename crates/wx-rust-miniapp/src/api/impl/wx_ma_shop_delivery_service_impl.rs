//! 小程序交易组件-物流发货服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShopDeliveryServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaShopDeliveryService;
use crate::bean::shop::request::{WxMaShopDeliveryRecieveRequest, WxMaShopDeliverySendRequest};
use crate::bean::shop::response::{WxMaShopBaseResponse, WxMaShopDeliveryGetCompanyListResponse};
use crate::enums::g3_urls::url_g3_shop::shop_delivery as delivery_url;

/// 小程序交易组件-物流发货服务实现。
pub struct WxMaShopDeliveryServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShopDeliveryServiceImpl {
    /// 构建物流发货服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaShopDeliveryService for WxMaShopDeliveryServiceImpl {
    /// 对应 Java `WxMaShopDeliveryServiceImpl.getCompanyList`：
    /// POST `GET_COMPANY_LIST`（空对象）后校验 errcode 并解析响应。
    async fn get_company_list(
        &self,
    ) -> Result<WxMaShopDeliveryGetCompanyListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(&delivery_url::get_company_list_url(config.as_ref()), "{}")
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopDeliveryServiceImpl.send`：
    /// POST `DELIVERY_SEND` 后校验 errcode 并解析响应。
    async fn send(
        &self,
        request: &WxMaShopDeliverySendRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&delivery_url::delivery_send_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopDeliveryServiceImpl.receive`：
    /// POST `DELIVERY_RECEIVE`（原常量拼写 recieve）后校验 errcode 并解析响应。
    async fn receive(
        &self,
        request: &WxMaShopDeliveryRecieveRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&delivery_url::delivery_receive_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
