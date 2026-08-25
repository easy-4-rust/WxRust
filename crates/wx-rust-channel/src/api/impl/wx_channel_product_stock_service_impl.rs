//! WxChannelProductStockServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelProductStockServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_product_stock_service::WxChannelProductStockService;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::product::stock::{StockFlowParam, StockFlowResponse};
use crate::bean::product::{SkuStockBatchResponse, SkuStockResponse};
use crate::enums::url_product_stock as url;

/// 商品库存服务实现。
pub struct WxChannelProductStockServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelProductStockServiceImpl {
    /// 构建商品库存服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxChannelProductStockService for WxChannelProductStockServiceImpl {
    async fn update_stock(
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
        let response = svc.post(url::UPDATE_STOCK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_sku_stock(
        &self,
        product_id: String,
        sku_id: String,
    ) -> Result<SkuStockResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({
            "product_id": product_id,
            "sku_id": sku_id
        })
        .to_string();
        // 复用已有的 product service 的 get_sku_stock URL
        let response = svc
            .post(
                "https://api.weixin.qq.com/channels/ec/product/sku/stock/get",
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_sku_stock_batch(
        &self,
        product_ids: Vec<String>,
    ) -> Result<SkuStockBatchResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = serde_json::json!({"product_ids": product_ids}).to_string();
        let response = svc
            .post(
                "https://api.weixin.qq.com/channels/ec/product/sku/stock/batch/get",
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_stock_flow(
        &self,
        param: StockFlowParam,
    ) -> Result<StockFlowResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GET_STOCK_FLOW_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
