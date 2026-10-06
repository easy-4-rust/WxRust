//! WxStoreProductServiceImpl（对应 Java
//! `com.binarywang.wxjava.store.api.impl.WxStoreProductServiceImpl`）。

use crate::bean::product::{
    GiftActivityAddParam, GiftActivityAddResponse, GiftActivityInfo, GiftProductAddResponse,
    GiftProductGetResponse, GiftProductInfo, GiftProductListParam, GiftProductListResponse,
};
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxStoreService;
use crate::api::wx_store_product_service::WxStoreProductService;
use crate::bean::base::WxStoreBaseResponse;
use crate::bean::limit::{LimitTaskAddResponse, LimitTaskListResponse, LimitTaskParam};
use crate::bean::product::assistant::{
    BeginTimingSaleParam, CategoryPreCheckParam, CategoryPreCheckResponse,
    ExternalProductMappingNewParam, ExternalProductMappingNewResponse, ExternalProductMappingParam,
    ExternalProductMappingResponse, ProductBrandRecommendParam, ProductBrandRecommendResponse,
};
use crate::bean::product::link::{
    ProductH5UrlResponse, ProductQrCodeResponse, ProductTagLinkResponse,
};
use crate::bean::product::stock::{StockFlowParam, StockFlowResponse};
use crate::bean::product::{
    AddProductThirdPartySourceParam, AddProductThirdPartySourceResponse, ProductAuditQuotaResponse,
    ProductAuditStrategyResponse, ProductAuditStrategySetParam, ProductCategoryClassifyParam,
    ProductCategoryClassifyResponse, ProductSchemeParam, ProductSchemeResponse, SkuStockBatchParam,
    SkuStockBatchResponse, SkuStockResponse, SpuFastInfo, SpuGetResponse, SpuInfo, SpuListResponse,
    SpuUpdateInfo, SpuUpdateResponse,
};
use crate::enums::url_product as url;

/// 构建 JSON 对象（跳过空值，对应 Java Jackson `JsonInclude.Include.NON_NULL`）。
fn build_json(pairs: &[(&str, serde_json::Value)]) -> String {
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        if !value.is_null() {
            map.insert((*key).to_string(), value.clone());
        }
    }
    serde_json::to_string(&serde_json::Value::Object(map)).unwrap_or_else(|_| "{}".to_string())
}

/// 数字字符串按数字输出（对应 Java `generateProductIdJson` 中
/// `"product_id":` + productId 的裸数字拼接；非数字则回退字符串）。
fn number_or_string(s: &str) -> serde_json::Value {
    if let Ok(n) = s.parse::<i64>() {
        serde_json::Value::from(n)
    } else if let Ok(n) = s.parse::<u64>() {
        serde_json::Value::from(n)
    } else {
        serde_json::Value::String(s.to_string())
    }
}

/// 商品服务实现。
pub struct WxStoreProductServiceImpl {
    service: Weak<dyn WxStoreService>,
}

impl WxStoreProductServiceImpl {
    /// 构建商品服务。
    pub fn new(service: Weak<dyn WxStoreService>) -> Self {
        Self { service }
    }

    /// 生成商品 id JSON（对应 Java `generateProductIdJson`：`product_id` 裸数字，
    /// `data_type` 为空时不输出）。
    fn generate_product_id_json(&self, product_id: &str, data_type: Option<i32>) -> String {
        build_json(&[
            ("product_id", number_or_string(product_id)),
            (
                "data_type",
                data_type
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ])
    }

    /// 简单的商品请求，参数为商品 id，只返回基本结果（对应 Java
    /// `simpleProductRequest`）。
    async fn simple_product_request(
        &self,
        svc: &dyn WxStoreService,
        url: &str,
        product_id: &str,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let req_json = self.generate_product_id_json(product_id, None);
        let response = svc.post(url, &req_json).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxStoreProductService for WxStoreProductServiceImpl {
    /// 对应 Java `WxStoreProductServiceImpl.addProduct(SpuUpdateInfo)`：
    /// 序列化 `SpuUpdateInfo` 后 POST `SPU_ADD_URL`。
    async fn add_product(
        &self,
        info: SpuUpdateInfo,
    ) -> Result<SpuUpdateResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SPU_ADD_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.updateProduct(SpuUpdateInfo)`：
    /// 序列化后 POST `SPU_UPDATE_URL`。
    async fn update_product(
        &self,
        info: SpuUpdateInfo,
    ) -> Result<SpuUpdateResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SPU_UPDATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.addProduct(SpuInfo)`：
    /// 序列化 `SpuInfo` 后 POST `SPU_ADD_URL`。
    async fn add_product_with_spu_info(
        &self,
        info: SpuInfo,
    ) -> Result<SpuUpdateResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SPU_ADD_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.updateProduct(SpuInfo)`：
    /// 序列化后 POST `SPU_UPDATE_URL`。
    async fn update_product_with_spu_info(
        &self,
        info: SpuInfo,
    ) -> Result<SpuUpdateResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SPU_UPDATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.updateProductAuditFree`：
    /// POST `SPU_AUDIT_FREE_UPDATE_URL`；Java 以 `SpuUpdateResponse` 解析后
    /// 按 `WxStoreBaseResponse` 返回（Rust 直接解析基础响应）。
    async fn update_product_audit_free(
        &self,
        info: SpuFastInfo,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SPU_AUDIT_FREE_UPDATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.updateStock`：
    /// `SkuStockParam`（空值跳过，Java Jackson `NON_NULL`）后 POST
    /// `SPU_UPDATE_STOCK_URL`。
    async fn update_stock(
        &self,
        product_id: String,
        sku_id: String,
        diff_type: Option<i32>,
        num: Option<i32>,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[
            ("product_id", serde_json::Value::String(product_id)),
            ("sku_id", serde_json::Value::String(sku_id)),
            (
                "diff_type",
                diff_type
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "num",
                num.map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc.post(url::SPU_UPDATE_STOCK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.deleteProduct`。
    async fn delete_product(
        &self,
        product_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        self.simple_product_request(svc.as_ref(), url::SPU_DEL_URL, &product_id)
            .await
    }

    /// 对应 Java `WxStoreProductServiceImpl.cancelProductAudit`。
    async fn cancel_product_audit(
        &self,
        product_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        self.simple_product_request(svc.as_ref(), url::CANCEL_AUDIT_URL, &product_id)
            .await
    }

    /// 对应 Java `WxStoreProductServiceImpl.getProduct`。
    async fn get_product(
        &self,
        product_id: String,
        data_type: Option<i32>,
    ) -> Result<SpuGetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let req_json = self.generate_product_id_json(&product_id, data_type);
        let response = svc.post(url::SPU_GET_URL, &req_json).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.listProduct`：
    /// `SpuListParam`（空值跳过，Java Jackson `NON_NULL`）后 POST `SPU_LIST_URL`。
    async fn list_product(
        &self,
        page_size: Option<i32>,
        next_key: String,
        status: Option<i32>,
    ) -> Result<SpuListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[
            (
                "page_size",
                page_size
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "next_key",
                if next_key.is_empty() {
                    serde_json::Value::Null
                } else {
                    serde_json::Value::String(next_key)
                },
            ),
            (
                "status",
                status
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc.post(url::SPU_LIST_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.upProduct`。
    async fn up_product(
        &self,
        product_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        self.simple_product_request(svc.as_ref(), url::SPU_LISTING_URL, &product_id)
            .await
    }

    /// 对应 Java `WxStoreProductServiceImpl.downProduct`。
    async fn down_product(
        &self,
        product_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        self.simple_product_request(svc.as_ref(), url::SPU_DELISTING_URL, &product_id)
            .await
    }

    /// 对应 Java `WxStoreProductServiceImpl.getSkuStock`：
    /// `{"product_id":"..","sku_id":".."}` 后 POST `SPU_GET_STOCK_URL`。
    async fn get_sku_stock(
        &self,
        product_id: String,
        sku_id: String,
    ) -> Result<SkuStockResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[
            ("product_id", serde_json::Value::String(product_id)),
            ("sku_id", serde_json::Value::String(sku_id)),
        ]);
        let response = svc.post(url::SPU_GET_STOCK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.getSkuStockBatch`：
    /// 序列化 `SkuStockBatchParam`（key 为 `product_id`）后 POST
    /// `SPU_GET_STOCK_BATCH_URL`。
    async fn get_sku_stock_batch(
        &self,
        product_ids: Vec<String>,
    ) -> Result<SkuStockBatchResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = SkuStockBatchParam {
            product_ids: Some(product_ids),
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SPU_GET_STOCK_BATCH_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.getProductH5Url`。
    async fn get_product_h5_url(
        &self,
        product_id: String,
    ) -> Result<ProductH5UrlResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[("product_id", serde_json::Value::String(product_id))]);
        let response = svc.post(url::SPU_H5URL_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.getProductQrCode`。
    async fn get_product_qr_code(
        &self,
        product_id: String,
    ) -> Result<ProductQrCodeResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[("product_id", serde_json::Value::String(product_id))]);
        let response = svc.post(url::SPU_QRCODE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.getProductTagLink`。
    async fn get_product_tag_link(
        &self,
        product_id: String,
    ) -> Result<ProductTagLinkResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[("product_id", serde_json::Value::String(product_id))]);
        let response = svc.post(url::SPU_TAGLINK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.addLimitTask`：
    /// 序列化 `LimitTaskParam` 后 POST `ADD_LIMIT_TASK_URL`。
    async fn add_limit_task(
        &self,
        param: LimitTaskParam,
    ) -> Result<LimitTaskAddResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ADD_LIMIT_TASK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.listLimitTask`：
    /// `LimitTaskListParam`（空值跳过，Java Jackson `NON_NULL`）后 POST
    /// `LIST_LIMIT_TASK_URL`。
    async fn list_limit_task(
        &self,
        page_size: Option<i32>,
        next_key: String,
        status: Option<i32>,
    ) -> Result<LimitTaskListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[
            (
                "page_size",
                page_size
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "next_key",
                if next_key.is_empty() {
                    serde_json::Value::Null
                } else {
                    serde_json::Value::String(next_key)
                },
            ),
            (
                "status",
                status
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc.post(url::LIST_LIMIT_TASK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.stopLimitTask`：
    /// `{"task_id": ".."}` 后 POST `STOP_LIMIT_TASK_URL`。
    async fn stop_limit_task(
        &self,
        task_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[("task_id", serde_json::Value::String(task_id))]);
        let response = svc.post(url::STOP_LIMIT_TASK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.deleteLimitTask`：
    /// `{"task_id": ".."}` 后 POST `DELETE_LIMIT_TASK_URL`。
    async fn delete_limit_task(
        &self,
        task_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[("task_id", serde_json::Value::String(task_id))]);
        let response = svc.post(url::DELETE_LIMIT_TASK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.getProductScheme`：
    /// 序列化 `ProductSchemeParam` 后 POST `SPU_SCHEME_URL`。
    async fn get_product_scheme(
        &self,
        param: ProductSchemeParam,
    ) -> Result<ProductSchemeResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SPU_SCHEME_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.classifyProductCategory`：
    /// 序列化 `ProductCategoryClassifyParam` 后 POST `SPU_CATEGORY_CLASSIFY_URL`。
    async fn classify_product_category(
        &self,
        param: ProductCategoryClassifyParam,
    ) -> Result<ProductCategoryClassifyResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SPU_CATEGORY_CLASSIFY_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.beginTimingSale`：
    /// 序列化 `BeginTimingSaleParam` 后 POST `SPU_BEGIN_TIMING_SALE_URL`。
    async fn begin_timing_sale(
        &self,
        param: BeginTimingSaleParam,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SPU_BEGIN_TIMING_SALE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.cancelTimingSale`：
    /// `{"product_id":".."}` 后 POST `SPU_CANCEL_TIMING_SALE_URL`。
    async fn cancel_timing_sale(
        &self,
        product_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = build_json(&[("product_id", serde_json::Value::String(product_id))]);
        let response = svc.post(url::SPU_CANCEL_TIMING_SALE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.externalProductMapping`：
    /// 序列化 `ExternalProductMappingParam` 后 POST `SPU_EXTERNAL_PRODUCT_MAPPING_URL`。
    async fn external_product_mapping(
        &self,
        param: ExternalProductMappingParam,
    ) -> Result<ExternalProductMappingResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(url::SPU_EXTERNAL_PRODUCT_MAPPING_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.categoryPreCheck`：
    /// 序列化 `CategoryPreCheckParam` 后 POST `SPU_CATEGORY_PRE_CHECK_URL`。
    async fn category_pre_check(
        &self,
        param: CategoryPreCheckParam,
    ) -> Result<CategoryPreCheckResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SPU_CATEGORY_PRE_CHECK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.getProductAuditStrategy`：
    /// POST `"{}"` 到 `SPU_AUDIT_STRATEGY_GET_URL`。
    async fn get_product_audit_strategy(
        &self,
    ) -> Result<ProductAuditStrategyResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let response = svc.post(url::SPU_AUDIT_STRATEGY_GET_URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.setProductAuditStrategy`：
    /// 序列化 `ProductAuditStrategySetParam` 后 POST `SPU_AUDIT_STRATEGY_SET_URL`。
    async fn set_product_audit_strategy(
        &self,
        param: ProductAuditStrategySetParam,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SPU_AUDIT_STRATEGY_SET_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.getProductAuditQuota`：
    /// POST `"{}"` 到 `SPU_GET_AUDIT_QUOTA_URL`。
    async fn get_product_audit_quota(&self) -> Result<ProductAuditQuotaResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let response = svc.post(url::SPU_GET_AUDIT_QUOTA_URL, "{}").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.externalProductMappingNew`：
    /// 序列化 `ExternalProductMappingNewParam` 后 POST
    /// `SPU_EXTERNAL_PRODUCT_MAPPING_NEW_URL`。
    async fn external_product_mapping_new(
        &self,
        param: ExternalProductMappingNewParam,
    ) -> Result<ExternalProductMappingNewResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(url::SPU_EXTERNAL_PRODUCT_MAPPING_NEW_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.productBrandRecommend`：
    /// 序列化 `ProductBrandRecommendParam` 后 POST `SPU_PRODUCT_BRAND_RECOMMEND_URL`。
    async fn product_brand_recommend(
        &self,
        param: ProductBrandRecommendParam,
    ) -> Result<ProductBrandRecommendResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(url::SPU_PRODUCT_BRAND_RECOMMEND_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.addProductThirdPartySource`：
    /// 序列化 `AddProductThirdPartySourceParam` 后 POST
    /// `SPU_ADD_PRODUCT_THIRD_PARTY_SOURCE_URL`。
    async fn add_product_third_party_source(
        &self,
        param: AddProductThirdPartySourceParam,
    ) -> Result<AddProductThirdPartySourceResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(url::SPU_ADD_PRODUCT_THIRD_PARTY_SOURCE_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxStoreProductServiceImpl.getStockFlow`：
    /// 序列化 `StockFlowParam` 后 POST `SPU_GET_STOCK_FLOW_URL`。
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
        let response = svc.post(url::SPU_GET_STOCK_FLOW_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn add_gift_product(
        &self,
        info: GiftProductInfo,
    ) -> Result<GiftProductAddResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(crate::enums::url_gift::GIFT_PRODUCT_ADD_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn update_gift_product(
        &self,
        info: GiftProductInfo,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(crate::enums::url_gift::GIFT_PRODUCT_UPDATE_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn set_product_as_gift(
        &self,
        product_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = serde_json::json!({"product_id": product_id}).to_string();
        let response = svc
            .post(crate::enums::url_gift::GIFT_PRODUCT_ON_SALE_SET_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_gift_product(
        &self,
        product_id: String,
    ) -> Result<GiftProductGetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = serde_json::json!({"product_id": product_id}).to_string();
        let response = svc
            .post(crate::enums::url_gift::GIFT_PRODUCT_GET_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn list_gift_product(
        &self,
        param: GiftProductListParam,
    ) -> Result<GiftProductListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(crate::enums::url_gift::GIFT_PRODUCT_LIST_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn update_gift_stock(
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
        let response = svc
            .post(crate::enums::url_gift::GIFT_PRODUCT_STOCK_UPDATE_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn add_gift_activity(
        &self,
        info: GiftActivityInfo,
    ) -> Result<GiftActivityAddResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = GiftActivityAddParam::new(info);
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(crate::enums::url_gift::GIFT_ACTIVITY_ADD_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn delete_gift_activity(
        &self,
        activity_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = serde_json::json!({"activity_id": activity_id}).to_string();
        let response = svc
            .post(crate::enums::url_gift::GIFT_ACTIVITY_DELETE_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn stop_gift_activity(
        &self,
        activity_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let body = serde_json::json!({"activity_id": activity_id}).to_string();
        let response = svc
            .post(crate::enums::url_gift::GIFT_ACTIVITY_STOP_URL, &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
