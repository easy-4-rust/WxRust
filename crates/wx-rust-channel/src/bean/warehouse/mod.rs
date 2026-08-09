//! 对应 Java `me.chanjar.weixin.channel.bean.warehouse` 包（生成）。

pub mod location_priority_response;
pub mod priority_location_param;
pub mod stock_get_param;
pub mod update_location_param;
pub mod warehouse;
pub mod warehouse_ids_response;
pub mod warehouse_location;
pub mod warehouse_location_param;
pub mod warehouse_param;
pub mod warehouse_response;
pub mod warehouse_stock_param;
pub mod warehouse_stock_response;

pub use location_priority_response::LocationPriorityResponse;
pub use priority_location_param::PriorityLocationParam;
pub use stock_get_param::StockGetParam;
pub use update_location_param::UpdateLocationParam;
pub use warehouse::Warehouse;
pub use warehouse_ids_response::WarehouseIdsResponse;
pub use warehouse_location::WarehouseLocation;
pub use warehouse_location_param::WarehouseLocationParam;
pub use warehouse_param::WarehouseParam;
pub use warehouse_response::WarehouseResponse;
pub use warehouse_stock_param::WarehouseStockParam;
pub use warehouse_stock_response::WarehouseStockResponse;
