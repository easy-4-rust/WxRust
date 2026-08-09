//! WxLeagueProductService（对应 Java `me.chanjar.weixin.channel.api.WxLeagueProductService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::league::product::{
    BatchAddParam, BatchAddResponse, ProductDetailParam, ProductDetailResponse, ProductListParam,
    ProductListResponse, ProductUpdateParam, ProductUpdateResponse,
};

/// 优选联盟 商品操作服务（对应 Java `WxLeagueProductService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_league_product_service_impl` 的
/// `WxLeagueProductServiceImpl`（Java `WxLeagueProductServiceImpl`）。
#[async_trait]
pub trait WxLeagueProductService: Send + Sync {
    /// 批量新增联盟商品（对应 Java `WxLeagueProductService#batchAddProduct`）。
    async fn batch_add_product(
        &self,
        param: BatchAddParam,
    ) -> Result<BatchAddResponse, WxErrorException>;

    /// 更新联盟商品信息（对应 Java `WxLeagueProductService#updateProduct`）。
    async fn update_league_product(
        &self,
        param: ProductUpdateParam,
    ) -> Result<ProductUpdateResponse, WxErrorException>;

    /// 删除联盟商品（对应 Java `WxLeagueProductService#deleteProduct`；
    /// `type`：1 普通推广商品 / 2 定向推广商品 / 3 专属推广商品；
    /// `info_id`：特殊推广商品计划 id，type 为特殊推广商品时必填）。
    async fn delete_league_product(
        &self,
        r#type: Option<i32>,
        product_id: String,
        info_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 拉取联盟商品详情（对应 Java `WxLeagueProductService#getProductDetail`）。
    async fn get_product_detail(
        &self,
        param: ProductDetailParam,
    ) -> Result<ProductDetailResponse, WxErrorException>;

    /// 拉取联盟商品推广列表（对应 Java `WxLeagueProductService#listProduct`）。
    async fn list_league_product(
        &self,
        param: ProductListParam,
    ) -> Result<ProductListResponse, WxErrorException>;
}
