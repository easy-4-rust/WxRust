//! WxTalentServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxTalentServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_talent_service::WxTalentService;
use crate::bean::talent::{
    TalentOrderDetailParam, TalentOrderDetailResponse, TalentOrderListParam,
    TalentOrderListResponse, TalentWindowProductDetailParam, TalentWindowProductDetailResponse,
    TalentWindowProductListParam, TalentWindowProductListResponse,
};
use crate::enums::url_talent as url;

/// 带货助手服务实现。
pub struct WxTalentServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxTalentServiceImpl {
    /// 构建带货助手服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxTalentService for WxTalentServiceImpl {
    async fn get_order_list(
        &self,
        param: TalentOrderListParam,
    ) -> Result<TalentOrderListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GET_ORDER_LIST_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_order_detail(
        &self,
        param: TalentOrderDetailParam,
    ) -> Result<TalentOrderDetailResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GET_ORDER_DETAIL_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_window_product_list(
        &self,
        param: TalentWindowProductListParam,
    ) -> Result<TalentWindowProductListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GET_WINDOW_PRODUCT_LIST_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_window_product_detail(
        &self,
        param: TalentWindowProductDetailParam,
    ) -> Result<TalentWindowProductDetailResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GET_WINDOW_PRODUCT_DETAIL_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
