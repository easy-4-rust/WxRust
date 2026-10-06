//! WxStoreProductStockService（对应 Java `com.binarywang.wxjava.store.api.WxStoreProductStockService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxStoreBaseResponse;
use crate::bean::product::stock::{StockFlowParam, StockFlowResponse};
use crate::bean::product::{SkuStockBatchResponse, SkuStockResponse};

/// 商品库存服务（对应 Java `WxStoreProductStockService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_product_stock_service_impl` 的
/// `WxStoreProductStockServiceImpl`（Java `WxStoreProductStockServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreProductStockService: Send + Sync {
    /// 更新商品库存（对应 Java `WxStoreProductStockService#updateStock(String, String, Integer, Integer)`）。
    async fn update_stock(
        &self,
        product_id: String,
        sku_id: String,
        diff_type: i32,
        num: i32,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 获取商品实时库存（对应 Java `WxStoreProductStockService#getSkuStock(String, String)`）。
    async fn get_sku_stock(
        &self,
        product_id: String,
        sku_id: String,
    ) -> Result<SkuStockResponse, WxErrorException>;

    /// 批量获取库存信息（对应 Java `WxStoreProductStockService#getSkuStockBatch(List)`）。
    async fn get_sku_stock_batch(
        &self,
        product_ids: Vec<String>,
    ) -> Result<SkuStockBatchResponse, WxErrorException>;

    /// 获取商品库存流水（对应 Java `WxStoreProductStockService#getStockFlow(StockFlowParam)`）。
    async fn get_stock_flow(
        &self,
        param: StockFlowParam,
    ) -> Result<StockFlowResponse, WxErrorException>;
}
