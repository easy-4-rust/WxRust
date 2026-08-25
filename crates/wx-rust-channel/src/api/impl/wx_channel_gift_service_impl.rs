//! WxChannelGiftServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelGiftServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_gift_service::WxChannelGiftService;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::product::{
    GiftActivityAddResponse, GiftActivityInfo, GiftProductAddResponse, GiftProductGetResponse,
    GiftProductInfo, GiftProductListParam, GiftProductListResponse,
};
use crate::enums::url_gift as url;

/// 赠品与买赠活动服务实现。
pub struct WxChannelGiftServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelGiftServiceImpl {
    /// 构建赠品服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxChannelGiftService for WxChannelGiftServiceImpl {
    async fn add_gift_product(
        &self,
        info: GiftProductInfo,
    ) -> Result<GiftProductAddResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GIFT_PRODUCT_ADD_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn update_gift_product(
        &self,
        info: GiftProductInfo,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GIFT_PRODUCT_UPDATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn set_product_as_gift(
        &self,
        product_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({"product_id": product_id}).to_string();
        let response = svc.post(url::GIFT_PRODUCT_ON_SALE_SET_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_gift_product(
        &self,
        product_id: String,
    ) -> Result<GiftProductGetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({"product_id": product_id}).to_string();
        let response = svc.post(url::GIFT_PRODUCT_GET_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn list_gift_product(
        &self,
        param: GiftProductListParam,
    ) -> Result<GiftProductListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GIFT_PRODUCT_LIST_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn update_gift_stock(
        &self,
        product_id: String,
        sku_id: String,
        diff_type: i32,
        num: i32,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({
            "product_id": product_id,
            "sku_id": sku_id,
            "diff_type": diff_type,
            "num": num
        })
        .to_string();
        let response = svc.post(url::GIFT_PRODUCT_STOCK_UPDATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn add_gift_activity(
        &self,
        info: GiftActivityInfo,
    ) -> Result<GiftActivityAddResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GIFT_ACTIVITY_ADD_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn delete_gift_activity(
        &self,
        activity_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({"activity_id": activity_id}).to_string();
        let response = svc.post(url::GIFT_ACTIVITY_DELETE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn stop_gift_activity(
        &self,
        activity_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({"activity_id": activity_id}).to_string();
        let response = svc.post(url::GIFT_ACTIVITY_STOP_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
