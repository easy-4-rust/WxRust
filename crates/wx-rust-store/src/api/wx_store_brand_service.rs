//! WxStoreBrandService（对应 Java `com.binarywang.wxjava.store.api.WxStoreBrandService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::audit::AuditApplyResponse;
use crate::bean::base::WxStoreBaseResponse;
use crate::bean::brand::{Brand, BrandApplyListResponse, BrandInfoResponse, BrandListResponse};

/// 品牌服务（对应 Java `WxStoreBrandService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_brand_service_impl` 的
/// `WxStoreBrandServiceImpl`（Java `WxStoreBrandServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreBrandService: Send + Sync {
    /// 获取品牌库列表（对应 Java `WxStoreBrandService#listAllBrand(Integer, String)`）。
    ///
    /// # 参数
    /// - `page_size`：每页数量（默认 10，不超过 50）
    /// - `next_key`：由上次请求返回，记录翻页的上下文，传入时会从上次返回的结果
    ///   往后翻一页，不传默认拉取第一页数据
    async fn list_all_brand(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<BrandListResponse, WxErrorException>;

    /// 新增品牌资质（对应 Java `WxStoreBrandService#addBrandApply(Brand)`）。
    async fn add_brand_apply(&self, brand: Brand) -> Result<AuditApplyResponse, WxErrorException>;

    /// 修改品牌资质（对应 Java `WxStoreBrandService#updateBrandApply(Brand)`）。
    async fn update_brand_apply(
        &self,
        brand: Brand,
    ) -> Result<AuditApplyResponse, WxErrorException>;

    /// 撤回品牌资质审核（对应 Java
    /// `WxStoreBrandService#cancelBrandApply(String, String)`）。
    async fn cancel_brand_apply(
        &self,
        brand_id: String,
        audit_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 删除品牌资质（对应 Java `WxStoreBrandService#deleteBrandApply(String)`）。
    async fn delete_brand_apply(
        &self,
        brand_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 获取品牌资质申请详情（对应 Java `WxStoreBrandService#getBrandApply(String)`）。
    async fn get_brand_apply(
        &self,
        brand_id: String,
    ) -> Result<BrandInfoResponse, WxErrorException>;

    /// 获取品牌资质申请列表（对应 Java
    /// `WxStoreBrandService#listBrandApply(Integer, String, Integer)`）。
    ///
    /// # 参数
    /// - `page_size`：每页数量（默认 10，不超过 50）
    /// - `next_key`：翻页上下文
    /// - `status`：审核单状态，不填默认拉全部商品
    async fn list_brand_apply(
        &self,
        page_size: Option<i32>,
        next_key: String,
        status: Option<i32>,
    ) -> Result<BrandApplyListResponse, WxErrorException>;

    /// 获取生效中的品牌资质列表（对应 Java
    /// `WxStoreBrandService#listValidBrandApply(Integer, String)`）。
    async fn list_valid_brand_apply(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<BrandApplyListResponse, WxErrorException>;
}
