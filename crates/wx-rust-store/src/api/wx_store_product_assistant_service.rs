//! WxStoreProductAssistantService（对应 Java `com.binarywang.wxjava.store.api.WxStoreProductAssistantService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxStoreBaseResponse;
use crate::bean::product::assistant::{
    BeginTimingSaleParam, CancelTimingSaleParam, CategoryPreCheckParam, CategoryPreCheckResponse,
    ExternalProductMappingNewParam, ExternalProductMappingNewResponse, ExternalProductMappingParam,
    ExternalProductMappingResponse, ProductBrandRecommendParam, ProductBrandRecommendResponse,
};

/// 商品辅助功能服务（对应 Java `WxStoreProductAssistantService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_product_assistant_service_impl` 的
/// `WxStoreProductAssistantServiceImpl`（Java `WxStoreProductAssistantServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreProductAssistantService: Send + Sync {
    /// 发品前校验（对应 Java `WxStoreProductAssistantService#categoryPreCheck(CategoryPreCheckParam)`）。
    async fn category_pre_check(
        &self,
        param: CategoryPreCheckParam,
    ) -> Result<CategoryPreCheckResponse, WxErrorException>;

    /// 获取商品品牌推荐（对应 Java `WxStoreProductAssistantService#getProductBrandRecommend(ProductBrandRecommendParam)`）。
    async fn get_product_brand_recommend(
        &self,
        param: ProductBrandRecommendParam,
    ) -> Result<ProductBrandRecommendResponse, WxErrorException>;

    /// 获取站内外商品属性映射（对应 Java `WxStoreProductAssistantService#externalProductMapping(ExternalProductMappingParam)`）。
    async fn external_product_mapping(
        &self,
        param: ExternalProductMappingParam,
    ) -> Result<ExternalProductMappingResponse, WxErrorException>;

    /// 获取商品属性映射及推荐（对应 Java `WxStoreProductAssistantService#externalProductMappingNew(ExternalProductMappingNewParam)`）。
    async fn external_product_mapping_new(
        &self,
        param: ExternalProductMappingNewParam,
    ) -> Result<ExternalProductMappingNewResponse, WxErrorException>;

    /// 将定时开售商品改为立即开售（对应 Java `WxStoreProductAssistantService#beginTimingSale(BeginTimingSaleParam)`）。
    async fn begin_timing_sale(
        &self,
        param: BeginTimingSaleParam,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 取消商品定时开售（对应 Java `WxStoreProductAssistantService#cancelTimingSale(CancelTimingSaleParam)`）。
    async fn cancel_timing_sale(
        &self,
        param: CancelTimingSaleParam,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;
}
