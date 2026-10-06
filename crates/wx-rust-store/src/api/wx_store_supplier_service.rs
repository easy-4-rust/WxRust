//! WxStoreSupplierService（对应 Java `com.binarywang.wxjava.store.api.WxStoreSupplierService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxStoreBaseResponse;
use crate::bean::supplier::{
    DistributeTypeResponse, DropshipAssignRequest, DropshipDetailResponse, DropshipListRequest,
    DropshipListResponse, DropshipResponse, DropshipSearchRequest, ProductDistributeRequest,
    ProductListResponse, SupplierInfoResponse, SupplierListResponse,
};

/// 代发管理服务（对应 Java `WxStoreSupplierService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_supplier_service_impl` 的
/// `WxStoreSupplierServiceImpl`（Java `WxStoreSupplierServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreSupplierService: Send + Sync {
    /// 获取供货商列表（对应 Java `WxStoreSupplierService#getSupplierList()`）。
    async fn get_supplier_list_default(&self) -> Result<SupplierListResponse, WxErrorException>;

    /// 获取供货商列表（对应 Java `WxStoreSupplierService#getSupplierList(Integer, String)`）。
    async fn get_supplier_list(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<SupplierListResponse, WxErrorException>;

    /// 获取分配方式（对应 Java `WxStoreSupplierService#getDistribute()`）。
    async fn get_distribute(&self) -> Result<DistributeTypeResponse, WxErrorException>;

    /// 设置全店订单手动分配（对应 Java `WxStoreSupplierService#setManuallyDistribute()`）。
    async fn set_manually_distribute(&self) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 设置全店订单自动分配（对应 Java `WxStoreSupplierService#setAllDistribute(String)`）。
    async fn set_all_distribute(
        &self,
        supplier_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 设置按商品自动分配（对应 Java `WxStoreSupplierService#setProductDistribute(ProductDistributeRequest)`）。
    async fn set_product_distribute(
        &self,
        req: ProductDistributeRequest,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 获取商品对应的自动分配供货商（对应 Java `WxStoreSupplierService#getProductDefaultDistribute(String)`）。
    async fn get_product_default_distribute(
        &self,
        product_id: String,
    ) -> Result<SupplierInfoResponse, WxErrorException>;

    /// 获取按商品自动分配的商品列表（对应 Java `WxStoreSupplierService#getProductList(String)`）。
    async fn get_product_list(
        &self,
        supplier_id: String,
    ) -> Result<ProductListResponse, WxErrorException>;

    /// 分配订单代发（对应 Java `WxStoreSupplierService#assignOrder(DropshipAssignRequest)`）。
    async fn assign_order(
        &self,
        req: DropshipAssignRequest,
    ) -> Result<DropshipResponse, WxErrorException>;

    /// 取消分配代发单（对应 Java `WxStoreSupplierService#cancelDropship(String)`）。
    async fn cancel_dropship(
        &self,
        order_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 查询代发单详情（对应 Java `WxStoreSupplierService#getDropship(String)`）。
    async fn get_dropship(
        &self,
        order_id: String,
    ) -> Result<DropshipDetailResponse, WxErrorException>;

    /// 拉取代发单列表（对应 Java `WxStoreSupplierService#listDropship(DropshipListRequest)`）。
    async fn list_dropship(
        &self,
        req: DropshipListRequest,
    ) -> Result<DropshipListResponse, WxErrorException>;

    /// 搜索代发单（对应 Java `WxStoreSupplierService#searchDropship(DropshipSearchRequest)`）。
    async fn search_dropship(
        &self,
        req: DropshipSearchRequest,
    ) -> Result<DropshipListResponse, WxErrorException>;
}
