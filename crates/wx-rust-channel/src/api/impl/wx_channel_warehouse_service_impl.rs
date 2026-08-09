//! WxChannelWarehouseServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelWarehouseServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_warehouse_service::WxChannelWarehouseService;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::warehouse::{
    LocationPriorityResponse, PriorityLocationParam, StockGetParam, UpdateLocationParam,
    WarehouseIdsResponse, WarehouseLocation, WarehouseParam, WarehouseResponse,
    WarehouseStockParam, WarehouseStockResponse,
};
use crate::enums::url_warehouse as url;

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

/// 区域仓库服务实现。
pub struct WxChannelWarehouseServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelWarehouseServiceImpl {
    /// 构建区域仓库服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxChannelWarehouseService for WxChannelWarehouseServiceImpl {
    /// 对应 Java `WxChannelWarehouseServiceImpl.createWarehouse`：
    /// 序列化 `WarehouseParam` 后 POST `ADD_WAREHOUSE_URL`。
    async fn create_warehouse(
        &self,
        param: WarehouseParam,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ADD_WAREHOUSE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelWarehouseServiceImpl.listWarehouse`：
    /// `StreamPageParam`（空值跳过，Java Jackson `NON_NULL`）后 POST
    /// `LIST_WAREHOUSE_URL`。
    async fn list_warehouse(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<WarehouseIdsResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
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
        ]);
        let response = svc.post(url::LIST_WAREHOUSE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelWarehouseServiceImpl.getWarehouse`：
    /// `{"out_warehouse_id":".."}` 后 POST `GET_WAREHOUSE_URL`。
    async fn get_warehouse(
        &self,
        out_warehouse_id: String,
    ) -> Result<WarehouseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[(
            "out_warehouse_id",
            serde_json::Value::String(out_warehouse_id),
        )]);
        let response = svc.post(url::GET_WAREHOUSE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelWarehouseServiceImpl.updateWarehouse`：
    /// `{"out_warehouse_id":"..","name":"..","intro":".."}` 后 POST
    /// `UPDATE_WAREHOUSE_URL`。
    async fn update_warehouse(
        &self,
        out_warehouse_id: String,
        name: String,
        intro: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[
            (
                "out_warehouse_id",
                serde_json::Value::String(out_warehouse_id),
            ),
            ("name", serde_json::Value::String(name)),
            ("intro", serde_json::Value::String(intro)),
        ]);
        let response = svc.post(url::UPDATE_WAREHOUSE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelWarehouseServiceImpl.addWarehouseArea`：
    /// 序列化 `UpdateLocationParam` 后 POST `ADD_COVER_AREA_URL`。
    async fn add_warehouse_area(
        &self,
        out_warehouse_id: String,
        cover_locations: Vec<WarehouseLocation>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = UpdateLocationParam {
            out_warehouse_id,
            cover_locations,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ADD_COVER_AREA_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelWarehouseServiceImpl.deleteWarehouseArea`：
    /// 序列化 `UpdateLocationParam` 后 POST `DELETE_COVER_AREA_URL`。
    async fn delete_warehouse_area(
        &self,
        out_warehouse_id: String,
        cover_locations: Vec<WarehouseLocation>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = UpdateLocationParam {
            out_warehouse_id,
            cover_locations,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::DELETE_COVER_AREA_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelWarehouseServiceImpl.setWarehousePriority`：
    /// 序列化 `PriorityLocationParam` 后 POST `SET_WAREHOUSE_PRIORITY_URL`。
    async fn set_warehouse_priority(
        &self,
        param: PriorityLocationParam,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::SET_WAREHOUSE_PRIORITY_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelWarehouseServiceImpl.getWarehousePriority`：
    /// `WarehouseLocationParam`（空值跳过）后 POST `GET_WAREHOUSE_PRIORITY_URL`。
    async fn get_warehouse_priority(
        &self,
        address_id1: Option<i32>,
        address_id2: Option<i32>,
        address_id3: Option<i32>,
        address_id4: Option<i32>,
    ) -> Result<LocationPriorityResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[
            (
                "address_id1",
                address_id1
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "address_id2",
                address_id2
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "address_id3",
                address_id3
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "address_id4",
                address_id4
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc.post(url::GET_WAREHOUSE_PRIORITY_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelWarehouseServiceImpl.updateWarehouseStock`：
    /// 序列化 `WarehouseStockParam` 后 POST `UPDATE_WAREHOUSE_STOCK_URL`。
    async fn update_warehouse_stock(
        &self,
        param: WarehouseStockParam,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::UPDATE_WAREHOUSE_STOCK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelWarehouseServiceImpl.getWarehouseStock`：
    /// 序列化 `StockGetParam` 后 POST `GET_WAREHOUSE_STOCK_URL`。
    async fn get_warehouse_stock(
        &self,
        product_id: String,
        sku_id: String,
        out_warehouse_id: String,
    ) -> Result<WarehouseStockResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = StockGetParam {
            product_id,
            sku_id,
            out_warehouse_id,
        };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::GET_WAREHOUSE_STOCK_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
