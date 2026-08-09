//! WxChannelBrandService（对应 Java `me.chanjar.weixin.channel.api.WxChannelBrandService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::audit::AuditApplyResponse;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::brand::{Brand, BrandApplyListResponse, BrandInfoResponse, BrandListResponse};

/// 品牌服务（对应 Java `WxChannelBrandService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_brand_service_impl` 的
/// `WxChannelBrandServiceImpl`（Java `WxChannelBrandServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelBrandService: Send + Sync {
    /// 获取品牌库列表（对应 Java `WxChannelBrandService#listAllBrand(Integer, String)`）。
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

    /// 新增品牌资质（对应 Java `WxChannelBrandService#addBrandApply(Brand)`）。
    async fn add_brand_apply(&self, brand: Brand) -> Result<AuditApplyResponse, WxErrorException>;

    /// 修改品牌资质（对应 Java `WxChannelBrandService#updateBrandApply(Brand)`）。
    async fn update_brand_apply(
        &self,
        brand: Brand,
    ) -> Result<AuditApplyResponse, WxErrorException>;

    /// 撤回品牌资质审核（对应 Java
    /// `WxChannelBrandService#cancelBrandApply(String, String)`）。
    async fn cancel_brand_apply(
        &self,
        brand_id: String,
        audit_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 删除品牌资质（对应 Java `WxChannelBrandService#deleteBrandApply(String)`）。
    async fn delete_brand_apply(
        &self,
        brand_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取品牌资质申请详情（对应 Java `WxChannelBrandService#getBrandApply(String)`）。
    async fn get_brand_apply(
        &self,
        brand_id: String,
    ) -> Result<BrandInfoResponse, WxErrorException>;

    /// 获取品牌资质申请列表（对应 Java
    /// `WxChannelBrandService#listBrandApply(Integer, String, Integer)`）。
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
    /// `WxChannelBrandService#listValidBrandApply(Integer, String)`）。
    async fn list_valid_brand_apply(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<BrandApplyListResponse, WxErrorException>;
}
