//! WxStoreProductService（对应 Java `com.binarywang.wxjava.store.api.WxStoreProductService`）。

use crate::bean::product::{
    GiftActivityAddResponse, GiftActivityInfo, GiftProductAddResponse, GiftProductGetResponse,
    GiftProductInfo, GiftProductListParam, GiftProductListResponse,
};
use wx_rust_common::error::WxErrorException;

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
    ProductCategoryClassifyResponse, ProductSchemeParam, ProductSchemeResponse,
    SkuStockBatchResponse, SkuStockResponse, SpuFastInfo, SpuGetResponse, SpuInfo, SpuListResponse,
    SpuUpdateInfo, SpuUpdateResponse,
};

/// 商品服务（对应 Java `WxStoreProductService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_product_service_impl` 的
/// `WxStoreProductServiceImpl`（Java `WxStoreProductServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreProductService: Send + Sync {
    /// 添加商品（对应 Java `WxStoreProductService#addProduct(SpuUpdateInfo)`）。
    async fn add_product(&self, info: SpuUpdateInfo)
    -> Result<SpuUpdateResponse, WxErrorException>;

    /// 更新商品（对应 Java `WxStoreProductService#updateProduct(SpuUpdateInfo)`）。
    async fn update_product(
        &self,
        info: SpuUpdateInfo,
    ) -> Result<SpuUpdateResponse, WxErrorException>;

    /// 添加商品（对应 Java `WxStoreProductService#addProduct(SpuInfo)`）。
    async fn add_product_with_spu_info(
        &self,
        info: SpuInfo,
    ) -> Result<SpuUpdateResponse, WxErrorException>;

    /// 更新商品（对应 Java `WxStoreProductService#updateProduct(SpuInfo)`）。
    async fn update_product_with_spu_info(
        &self,
        info: SpuInfo,
    ) -> Result<SpuUpdateResponse, WxErrorException>;

    /// 免审更新商品（对应 Java `WxStoreProductService#updateProductAuditFree(SpuFastInfo)`）。
    async fn update_product_audit_free(
        &self,
        info: SpuFastInfo,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 更新商品库存（仅对 `edit_status != 2` 的商品适用，其他状态的商品无法
    /// 通过该接口修改库存；对应 Java
    /// `WxStoreProductService#updateStock(String, String, Integer, Integer)`）。
    async fn update_stock(
        &self,
        product_id: String,
        sku_id: String,
        diff_type: Option<i32>,
        num: Option<i32>,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 删除商品（对应 Java `WxStoreProductService#deleteProduct(String)`）。
    async fn delete_product(
        &self,
        product_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 撤回商品审核（对应 Java `WxStoreProductService#cancelProductAudit(String)`）。
    async fn cancel_product_audit(
        &self,
        product_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 获取商品（对应 Java `WxStoreProductService#getProduct(String, Integer)`）。
    ///
    /// # 参数
    /// - `data_type`：默认取 1。1 获取线上数据，2 获取草稿数据，3 同时获取线上和
    ///   草稿数据（注意：需成功上架后才有线上数据）
    async fn get_product(
        &self,
        product_id: String,
        data_type: Option<i32>,
    ) -> Result<SpuGetResponse, WxErrorException>;

    /// 获取商品列表（对应 Java
    /// `WxStoreProductService#listProduct(Integer, String, Integer)`）。
    async fn list_product(
        &self,
        page_size: Option<i32>,
        next_key: String,
        status: Option<i32>,
    ) -> Result<SpuListResponse, WxErrorException>;

    /// 上架商品（对应 Java `WxStoreProductService#upProduct(String)`）。
    async fn up_product(&self, product_id: String)
    -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 下架商品（对应 Java `WxStoreProductService#downProduct(String)`）。
    async fn down_product(
        &self,
        product_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 获取商品实时库存（对应 Java
    /// `WxStoreProductService#getSkuStock(String, String)`）。
    async fn get_sku_stock(
        &self,
        product_id: String,
        sku_id: String,
    ) -> Result<SkuStockResponse, WxErrorException>;

    /// 批量获取库存信息（单次请求不能超过 50 个商品 ID；对应 Java
    /// `WxStoreProductService#getSkuStockBatch(List<String>)`）。
    async fn get_sku_stock_batch(
        &self,
        product_ids: Vec<String>,
    ) -> Result<SkuStockBatchResponse, WxErrorException>;

    /// 获取商品 H5 链接（对应 Java `WxStoreProductService#getProductH5Url(String)`）。
    async fn get_product_h5_url(
        &self,
        product_id: String,
    ) -> Result<ProductH5UrlResponse, WxErrorException>;

    /// 获取商品二维码（对应 Java `WxStoreProductService#getProductQrCode(String)`）。
    async fn get_product_qr_code(
        &self,
        product_id: String,
    ) -> Result<ProductQrCodeResponse, WxErrorException>;

    /// 获取商品口令（对应 Java `WxStoreProductService#getProductTagLink(String)`）。
    async fn get_product_tag_link(
        &self,
        product_id: String,
    ) -> Result<ProductTagLinkResponse, WxErrorException>;

    /// 添加限时抢购任务（对应 Java `WxStoreProductService#addLimitTask(LimitTaskParam)`）。
    async fn add_limit_task(
        &self,
        param: LimitTaskParam,
    ) -> Result<LimitTaskAddResponse, WxErrorException>;

    /// 拉取限时抢购任务列表（对应 Java
    /// `WxStoreProductService#listLimitTask(Integer, String, Integer)`）。
    async fn list_limit_task(
        &self,
        page_size: Option<i32>,
        next_key: String,
        status: Option<i32>,
    ) -> Result<LimitTaskListResponse, WxErrorException>;

    /// 停止限时抢购任务（对应 Java `WxStoreProductService#stopLimitTask(String)`）。
    async fn stop_limit_task(
        &self,
        task_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 停止限时抢购任务（对应 Java `WxStoreProductService#deleteLimitTask(String)`）。
    async fn delete_limit_task(
        &self,
        task_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 获取商品移动应用跳转 scheme 码（对应 Java
    /// `WxStoreProductService#getProductScheme(ProductSchemeParam)`）。
    async fn get_product_scheme(
        &self,
        param: ProductSchemeParam,
    ) -> Result<ProductSchemeResponse, WxErrorException>;

    /// 商品类目推荐（对应 Java
    /// `WxStoreProductService#classifyProductCategory(ProductCategoryClassifyParam)`）。
    async fn classify_product_category(
        &self,
        param: ProductCategoryClassifyParam,
    ) -> Result<ProductCategoryClassifyResponse, WxErrorException>;

    /// 商品定时开售（对应 Java
    /// `WxStoreProductService#beginTimingSale(BeginTimingSaleParam)`）。
    async fn begin_timing_sale(
        &self,
        param: BeginTimingSaleParam,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 取消商品定时开售（对应 Java
    /// `WxStoreProductService#cancelTimingSale(String)`）。
    async fn cancel_timing_sale(
        &self,
        product_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 外部商品映射（对应 Java
    /// `WxStoreProductService#externalProductMapping(ExternalProductMappingParam)`）。
    async fn external_product_mapping(
        &self,
        param: ExternalProductMappingParam,
    ) -> Result<ExternalProductMappingResponse, WxErrorException>;

    /// 类目预检（对应 Java
    /// `WxStoreProductService#categoryPreCheck(CategoryPreCheckParam)`）。
    async fn category_pre_check(
        &self,
        param: CategoryPreCheckParam,
    ) -> Result<CategoryPreCheckResponse, WxErrorException>;

    /// 获取商品上架策略（对应 Java `WxStoreProductService#getProductAuditStrategy`）。
    async fn get_product_audit_strategy(
        &self,
    ) -> Result<ProductAuditStrategyResponse, WxErrorException>;

    /// 设置商品上架策略（对应 Java
    /// `WxStoreProductService#setProductAuditStrategy(ProductAuditStrategySetParam)`）。
    async fn set_product_audit_strategy(
        &self,
        param: ProductAuditStrategySetParam,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 获取商品提审限额（对应 Java `WxStoreProductService#getProductAuditQuota`）。
    async fn get_product_audit_quota(&self) -> Result<ProductAuditQuotaResponse, WxErrorException>;

    /// 外部商品映射（新版）（对应 Java
    /// `WxStoreProductService#externalProductMappingNew(ExternalProductMappingNewParam)`）。
    async fn external_product_mapping_new(
        &self,
        param: ExternalProductMappingNewParam,
    ) -> Result<ExternalProductMappingNewResponse, WxErrorException>;

    /// 商品品牌推荐（对应 Java
    /// `WxStoreProductService#productBrandRecommend(ProductBrandRecommendParam)`）。
    async fn product_brand_recommend(
        &self,
        param: ProductBrandRecommendParam,
    ) -> Result<ProductBrandRecommendResponse, WxErrorException>;

    /// 新增第三方货源信息（对应 Java
    /// `WxStoreProductService#addProductThirdPartySource(AddProductThirdPartySourceParam)`）。
    async fn add_product_third_party_source(
        &self,
        param: AddProductThirdPartySourceParam,
    ) -> Result<AddProductThirdPartySourceResponse, WxErrorException>;

    /// 获取库存流水（对应 Java
    /// `WxStoreProductService#getStockFlow(StockFlowParam)`）。
    async fn get_stock_flow(
        &self,
        param: StockFlowParam,
    ) -> Result<StockFlowResponse, WxErrorException>;

    /// 添加非卖商品（对应 Java `WxStoreProductService#addGiftProduct(GiftProductInfo)`）。
    async fn add_gift_product(
        &self,
        info: GiftProductInfo,
    ) -> Result<GiftProductAddResponse, WxErrorException>;

    /// 更新非卖商品（对应 Java `WxStoreProductService#updateGiftProduct(GiftProductInfo)`）。
    async fn update_gift_product(
        &self,
        info: GiftProductInfo,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 在售商品转赠品（对应 Java `WxStoreProductService#setProductAsGift(String)`）。
    async fn set_product_as_gift(
        &self,
        product_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 获取赠品（对应 Java `WxStoreProductService#getGiftProduct(String)`）。
    async fn get_gift_product(
        &self,
        product_id: String,
    ) -> Result<GiftProductGetResponse, WxErrorException>;

    /// 获取赠品列表（对应 Java `WxStoreProductService#listGiftProduct(GiftProductListParam)`）。
    async fn list_gift_product(
        &self,
        param: GiftProductListParam,
    ) -> Result<GiftProductListResponse, WxErrorException>;

    /// 更新赠品库存（对应 Java `WxStoreProductService#updateGiftStock(String, String, Integer, Integer)`）。
    async fn update_gift_stock(
        &self,
        product_id: String,
        sku_id: String,
        diff_type: i32,
        num: i32,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 创建赠品活动（对应 Java `WxStoreProductService#addGiftActivity(GiftActivityInfo)`）。
    async fn add_gift_activity(
        &self,
        info: GiftActivityInfo,
    ) -> Result<GiftActivityAddResponse, WxErrorException>;

    /// 删除赠品活动（对应 Java `WxStoreProductService#deleteGiftActivity(String)`）。
    async fn delete_gift_activity(
        &self,
        activity_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 停止赠品活动（对应 Java `WxStoreProductService#stopGiftActivity(String)`）。
    async fn stop_gift_activity(
        &self,
        activity_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;
}
