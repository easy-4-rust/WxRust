//! WxStoreProductStockServiceImpl（对应 Java
//! `com.binarywang.wxjava.store.api.impl.WxStoreProductStockServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxStoreService;
use crate::api::wx_store_product_stock_service::WxStoreProductStockService;
use crate::bean::base::WxStoreBaseResponse;
use crate::bean::product::stock::{StockFlowParam, StockFlowResponse};
use crate::bean::product::{SkuStockBatchResponse, SkuStockResponse};
use crate::enums::url_product_stock as url;

/// 商品库存服务实现。
pub struct WxStoreProductStockServiceImpl {
    service: Weak<dyn WxStoreService>,
}

impl WxStoreProductStockServiceImpl {
    /// 构建商品库存服务。
    pub fn new(service: Weak<dyn WxStoreService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxStoreProductStockService for WxStoreProductStockServiceImpl {
    async fn update_stock(
        &self,
        product_id: String,
        sku_id: String,
        diff_type: i32,
        num: i32,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
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
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = serde_json::json!({
            "product_id": product_id,
            "sku_id": sku_id
        })
        .to_string();
        let response = svc.post(url::GET_STOCK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_sku_stock_batch(
        &self,
        product_ids: Vec<String>,
    ) -> Result<SkuStockBatchResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        // Java SkuStockBatchParam 使用 @JsonProperty("product_id") —— 单数形式
        let body = serde_json::json!({"product_id": product_ids}).to_string();
        let response = svc.post(url::GET_STOCK_BATCH_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_stock_flow(
        &self,
        param: StockFlowParam,
    ) -> Result<StockFlowResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GET_STOCK_FLOW_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
