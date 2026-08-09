//! WxChannelWarehouseService（对应 Java `me.chanjar.weixin.channel.api.WxChannelWarehouseService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::warehouse::{
    LocationPriorityResponse, PriorityLocationParam, WarehouseIdsResponse, WarehouseLocation,
    WarehouseParam, WarehouseResponse, WarehouseStockParam, WarehouseStockResponse,
};

/// 区域仓库服务（对应 Java `WxChannelWarehouseService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_warehouse_service_impl` 的
/// `WxChannelWarehouseServiceImpl`（Java `WxChannelWarehouseServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelWarehouseService: Send + Sync {
    /// 创建仓库（对应 Java `WxChannelWarehouseService#createWarehouse(WarehouseParam)`）。
    async fn create_warehouse(
        &self,
        param: WarehouseParam,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 查询仓库列表（对应 Java
    /// `WxChannelWarehouseService#listWarehouse(Integer, String)`）。
    ///
    /// # 参数
    /// - `page_size`：每页数量（最大不超过 10）
    /// - `next_key`：由上次请求返回，记录翻页的上下文
    async fn list_warehouse(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<WarehouseIdsResponse, WxErrorException>;

    /// 获取仓库详情（对应 Java `WxChannelWarehouseService#getWarehouse(String)`）。
    async fn get_warehouse(
        &self,
        out_warehouse_id: String,
    ) -> Result<WarehouseResponse, WxErrorException>;

    /// 修改仓库详情（对应 Java
    /// `WxChannelWarehouseService#updateWarehouse(String, String, String)`）。
    async fn update_warehouse(
        &self,
        out_warehouse_id: String,
        name: String,
        intro: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 批量增加覆盖区域（对应 Java
    /// `WxChannelWarehouseService#addWarehouseArea(String, List<WarehouseLocation>)`）。
    async fn add_warehouse_area(
        &self,
        out_warehouse_id: String,
        cover_locations: Vec<WarehouseLocation>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 批量删除覆盖区域（对应 Java
    /// `WxChannelWarehouseService#deleteWarehouseArea(String, List<WarehouseLocation>)`）。
    async fn delete_warehouse_area(
        &self,
        out_warehouse_id: String,
        cover_locations: Vec<WarehouseLocation>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 设置指定地址下的仓的优先级（对应 Java
    /// `WxChannelWarehouseService#setWarehousePriority(PriorityLocationParam)`）。
    async fn set_warehouse_priority(
        &self,
        param: PriorityLocationParam,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取指定地址下的仓的优先级（对应 Java
    /// `WxChannelWarehouseService#getWarehousePriority(Integer, Integer, Integer, Integer)`）。
    async fn get_warehouse_priority(
        &self,
        address_id1: Option<i32>,
        address_id2: Option<i32>,
        address_id3: Option<i32>,
        address_id4: Option<i32>,
    ) -> Result<LocationPriorityResponse, WxErrorException>;

    /// 更新区域仓库存数量（对应 Java
    /// `WxChannelWarehouseService#updateWarehouseStock(WarehouseStockParam)`）。
    async fn update_warehouse_stock(
        &self,
        param: WarehouseStockParam,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取区域仓库存数量（对应 Java
    /// `WxChannelWarehouseService#getWarehouseStock(String, String, String)`）。
    async fn get_warehouse_stock(
        &self,
        product_id: String,
        sku_id: String,
        out_warehouse_id: String,
    ) -> Result<WarehouseStockResponse, WxErrorException>;
}
